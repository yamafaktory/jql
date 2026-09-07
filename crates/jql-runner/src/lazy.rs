use std::{
    collections::HashSet,
    num::NonZeroUsize,
    ops::Range,
};

use jql_parser::{
    group::split,
    parser::parse,
    tokens::{
        Lens,
        LensValue,
        Token,
    },
};
use serde::{
    Deserialize,
    de::IgnoredAny,
};
use serde_json::{
    Map,
    Number,
    Value,
};
use simd_json::{
    Buffers,
    prelude::{
        TypedValue,
        ValueAsScalar,
        ValueType,
    },
    tape::{
        Object as TapeObject,
        Value as TapeValue,
    },
};

use crate::{
    errors::JqlRunnerError,
    runner,
};

/// Takes a raw query and a slice of JSON bytes.
/// Returns a JSON `Value` or an error.
///
/// Selection queries (key, index, range, multi-key, keys `@`, truncate `!`, lens
/// `|=`, and a single pipe `|> … <|`) are evaluated against a simd-json tape,
/// materializing only the selected subtrees. Queries using the flatten operator
/// or nested pipes, and any input the tape builder rejects (nested beyond
/// simd-json's limit, trailing data, malformed) or that carries a number
/// simd-json parses differently, fall back to the `Value`-based [`runner`],
/// preserving its exact behavior.
///
/// # Errors
///
/// Returns a `JqlRunnerError` on failure.
pub fn raw(query: &str, json: &mut [u8]) -> Result<Value, JqlRunnerError> {
    Evaluator::new().raw(query, json)
}

/// Takes a raw query and a slice of JSON bytes holding one or more documents.
/// Returns one JSON `Value` per document.
///
/// A single document takes the same tape path as [`raw`]. Several — concatenated
/// with or without separating whitespace, each pretty-printed or not — are read
/// one at a time by serde_json, which simd-json has no equivalent for, and the
/// query is applied to each.
///
/// # Errors
///
/// Returns a `JqlRunnerError` if the query or any document fails, including when
/// content follows the last document that is not itself a document.
pub fn raw_all(query: &str, json: &mut [u8]) -> Result<Vec<Value>, JqlRunnerError> {
    Evaluator::new().raw_all(query, json)
}

/// An evaluator holding the scratch space a tape build needs, so that a caller
/// with many inputs — a stream of documents, say — pays for it once.
///
/// [`raw`] and [`raw_all`] are the one-shot equivalents.
#[derive(Default)]
pub struct Evaluator {
    buffers: Buffers,
}

impl std::fmt::Debug for Evaluator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("Evaluator").finish_non_exhaustive()
    }
}

impl Evaluator {
    /// Returns an evaluator with empty scratch space, grown as inputs demand.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Takes a raw query and a slice of JSON bytes.
    /// Returns a JSON `Value` or an error.
    ///
    /// See [`raw`] for what the tape evaluates and what falls back.
    ///
    /// # Errors
    ///
    /// Returns a `JqlRunnerError` on failure.
    pub fn raw(&mut self, query: &str, json: &mut [u8]) -> Result<Value, JqlRunnerError> {
        if query.is_empty() {
            return Err(JqlRunnerError::EmptyQueryError);
        }

        let tokens = parse(query)?;

        if let Some(value) = self.evaluate(&tokens, json)?.value {
            return Ok(value);
        }

        runner::token(&tokens, &deserialize(json)?)
    }

    /// Takes a raw query and a slice of JSON bytes holding one or more
    /// documents.
    /// Returns one JSON `Value` per document.
    ///
    /// See [`raw_all`] for how several documents are read.
    ///
    /// # Errors
    ///
    /// Returns a `JqlRunnerError` if the query or any document fails, including
    /// when content follows the last document that is not itself a document.
    pub fn raw_all(&mut self, query: &str, json: &mut [u8]) -> Result<Vec<Value>, JqlRunnerError> {
        if query.is_empty() {
            return Err(JqlRunnerError::EmptyQueryError);
        }

        let tokens = parse(query)?;

        // A single document — the common case — tapes the whole input. The tape
        // builder also rejects anything after the first document, which is what
        // sends multi-document input down the path below.
        let declined = self.evaluate(&tokens, json)?;

        if let Some(value) = declined.value {
            return Ok(vec![value]);
        }

        if let Some(ranges) = declined.document_starts.map_or_else(
            || document_ranges(json),
            |starts| Some(ranges_from_starts(&starts, json.len())),
        ) {
            let mut values = Vec::with_capacity(ranges.len());
            let mut rest = &mut *json;

            // The ranges are contiguous and cover the whole input, so each
            // document is the next chunk of what is left.
            for range in ranges {
                let (document, tail) = rest.split_at_mut(range.len());

                values.push(match self.evaluate(&tokens, document)?.value {
                    Some(value) => value,
                    None => runner::token(&tokens, &deserialize(document)?)?,
                });

                rest = tail;
            }

            return Ok(values);
        }

        // Not a clean sequence of documents: the input must then be exactly one,
        // which is also the path that reads arbitrarily deep nesting.
        Ok(vec![runner::token(&tokens, &deserialize(json)?)?])
    }

    /// Evaluates `tokens` against a single document on the tape.
    ///
    /// A `value` of `None` means the tape declined the input and the caller must
    /// fall back; `document_starts` is then set if the reason was that the input
    /// holds more than one document, saving the caller a scan for where they
    /// begin.
    fn evaluate(&mut self, tokens: &[Token], json: &mut [u8]) -> Result<Outcome, JqlRunnerError> {
        if !can_lazy(tokens) {
            return Ok(Outcome::fallback());
        }

        // A lens compares numbers to decide which elements survive, so a
        // divergence there changes the selection rather than a value in the
        // outcome, where the check below would see it. Those queries keep the
        // up-front scan.
        if lens_compares_numbers(tokens) && numbers_may_diverge(json) {
            return Ok(Outcome::fallback());
        }

        // The only bytes the tape builder writes are the ones it unescapes a
        // string into, so input without a backslash is left intact and can be
        // taped in place — the fallback below still sees pristine bytes.
        // Anything else gets a copy. `tape_leaves_escape_free_input_intact`
        // pins that invariant down.
        let outcome = if memchr::memchr(b'\\', json).is_none() {
            match simd_json::to_tape_with_buffers(json, &mut self.buffers) {
                Ok(tape) => eval(tokens, tape.as_value()),
                Err(_) => return Ok(self.declined(json)),
            }
        } else {
            let mut scratch = json.to_vec();

            match simd_json::to_tape_with_buffers(&mut scratch, &mut self.buffers) {
                Ok(tape) => eval(tokens, tape.as_value()),
                // The pristine input, not the copy the failure may have partly
                // unescaped, is what the offsets are read against.
                Err(_) => return Ok(self.declined(json)),
            }
        };

        if outcome_may_diverge(&outcome, json) {
            return Ok(Outcome::fallback());
        }

        outcome.map(|value| Outcome {
            value: Some(value),
            document_starts: None,
        })
    }

    /// Turns a tape build failure into a declined outcome, reading the document
    /// boundaries out of the stage-1 scan it already paid for.
    ///
    /// The failure itself says nothing usable about why — a second document
    /// reports the same internal tape error as some malformed input — so
    /// [`document_starts`] is what decides whether the scan describes a clean
    /// sequence of documents.
    fn declined(&self, json: &[u8]) -> Outcome {
        Outcome {
            value: None,
            document_starts: document_starts(self.buffers.structural_indexes(), json),
        }
    }
}

/// What [`Evaluator::evaluate`] came back with.
struct Outcome {
    /// The evaluated value, or `None` when the tape declined the input.
    value: Option<Value>,
    /// Where each document begins, when the tape declined because there was
    /// more than one.
    document_starts: Option<Vec<usize>>,
}

impl Outcome {
    fn fallback() -> Self {
        Self {
            value: None,
            document_starts: None,
        }
    }
}

/// Whether the numbers `outcome` carries could have been parsed differently by
/// serde_json, which means the tape must be given up for it.
///
/// Only the numbers that made it out of the evaluation can diverge in a way
/// anyone observes, and a query selects far less than it reads — so the input is
/// scanned for the matching hazard only once one shows up here, and only for
/// that hazard. A query drilling down to a string, or projecting integers, pays
/// nothing.
fn outcome_may_diverge(outcome: &Result<Value, JqlRunnerError>, json: &[u8]) -> bool {
    let (float, zero) = match outcome {
        Ok(value) => number_hazards(value),
        // An error embeds the value it was raised on, and it is compared whole.
        Err(error) => error_value(error).map_or((false, false), number_hazards),
    };

    (float && long_mantissa_may_diverge(json)) || (zero && negative_zero_may_diverge(json))
}

/// The `Value` a `JqlRunnerError` carries, if any.
fn error_value(error: &JqlRunnerError) -> Option<&Value> {
    match error {
        JqlRunnerError::FlattenError(value)
        | JqlRunnerError::InvalidArrayError(value)
        | JqlRunnerError::InvalidObjectError(value)
        | JqlRunnerError::PipeInError(value)
        | JqlRunnerError::IndexOutOfBoundsError { parent: value, .. }
        | JqlRunnerError::KeyNotFoundError { parent: value, .. }
        | JqlRunnerError::MultiKeyNotFoundError { parent: value, .. }
        | JqlRunnerError::RangeOutOfBoundsError { parent: value, .. } => Some(value),
        _ => None,
    }
}

/// Whether `value` holds a float, and whether it holds an integer zero — the two
/// shapes a divergent number can take once parsed.
///
/// A `-0` reaches the tape as the integer zero and serde_json as `-0.0`, so an
/// integer zero in the outcome is what makes that hazard worth looking for.
fn number_hazards(value: &Value) -> (bool, bool) {
    match value {
        Value::Number(number) => (number.is_f64(), number.as_u64() == Some(0)),
        Value::Array(values) => values.iter().fold((false, false), |acc, value| {
            let hazards = number_hazards(value);

            (acc.0 || hazards.0, acc.1 || hazards.1)
        }),
        Value::Object(entries) => entries.iter().fold((false, false), |acc, (_, value)| {
            let hazards = number_hazards(value);

            (acc.0 || hazards.0, acc.1 || hazards.1)
        }),
        Value::Bool(_) | Value::Null | Value::String(_) => (false, false),
    }
}

/// Whether any lens in `tokens` selects on a number.
fn lens_compares_numbers(tokens: &[Token]) -> bool {
    tokens.iter().any(|token| match token {
        Token::LensSelector(lenses) => lenses
            .iter()
            .any(|lens| matches!(lens.get_ref().1, Some(LensValue::Number(_)))),
        _ => false,
    })
}

/// Where each top-level document begins, read from the structural positions
/// stage 1 already found, or `None` if the input is not a clean sequence of at
/// least two documents that are all arrays or objects.
///
/// A failed tape build has scanned the whole input for structural characters
/// before giving up, so the nesting can be followed here without touching the
/// bytes again — every position outside a string, in order. Whatever this
/// declines goes to [`document_ranges`], which reads the input properly.
fn document_starts(structural_indexes: &[u32], json: &[u8]) -> Option<Vec<usize>> {
    let mut starts = Vec::new();
    let mut depth = 0_usize;

    for &index in structural_indexes {
        let index = index as usize;
        let byte = *json.get(index)?;

        if depth == 0 {
            // A top-level scalar or string needs the width of its literal to be
            // known to be skipped, which the positions alone don't give.
            if !matches!(byte, b'{' | b'[') {
                return None;
            }

            starts.push(index);
        }

        match byte {
            b'{' | b'[' => depth += 1,
            b'}' | b']' => depth = depth.checked_sub(1)?,
            _ => {}
        }
    }

    (depth == 0 && starts.len() > 1).then_some(starts)
}

/// Turns document start offsets into the contiguous ranges covering `len`.
/// The first range starts at zero so that leading whitespace belongs to the
/// first document rather than to nothing.
fn ranges_from_starts(starts: &[usize], len: usize) -> Vec<Range<usize>> {
    starts
        .iter()
        .enumerate()
        .map(|(position, &start)| {
            let start = if position == 0 { 0 } else { start };
            let end = starts.get(position + 1).copied().unwrap_or(len);

            start..end
        })
        .collect()
}

/// The byte range of every JSON document in `json`, or `None` if the input is
/// not a clean sequence of at least two documents.
///
/// Documents are skipped rather than built, which is markedly cheaper than
/// reading each one into a `Value` just to learn where it ends. serde_json skips
/// with an explicit stack rather than by recursing, so this is safe at any
/// nesting depth — each document is then read on its own, where a tape that
/// refuses the depth falls back to the growable-stack deserializer.
fn document_ranges(json: &[u8]) -> Option<Vec<Range<usize>>> {
    let mut stream = serde_json::Deserializer::from_slice(json).into_iter::<IgnoredAny>();
    let mut ranges = Vec::new();
    let mut start = 0;

    while let Some(document) = stream.next() {
        document.ok()?;

        let end = stream.byte_offset();

        ranges.push(start..end);
        start = end;
    }

    (ranges.len() > 1).then_some(ranges)
}

/// A digit count this far past a decimal point is already well over the 16 that
/// matters, and caps the work on adversarial input.
const DIGIT_SCAN_CAP: usize = 24;

/// Whether `json` has a number literal that simd-json and serde_json parse to
/// different `f64` bits, so the tape path must be skipped:
///
/// - `-0`: serde_json makes it the float `-0.0`, simd-json the integer `0`.
/// - a fractional literal with 16+ significant digits.
///
/// Not covered: scientific notation. The two parsers round the last bit of
/// `1.5e12`-style literals differently for a meaningful share of values, but
/// recognising them means telling a number's exponent from an `e` inside a
/// string, which needs a quote- and escape-aware scan — too slow to be worth it,
/// while a loose `<digit>e` test false-positives on base64, hashes and names
/// (`"ICMSE2ETest"`) and would give up the tape for whole documents. Plain
/// decimals — `12345.67`, `-12.345678`, `0.9876` — are exact: sampling 150k of
/// them found no divergence. So are integers, which are exact both ways or make
/// the tape build fail anyway.
///
/// The scan short-circuits, and a false positive inside a string only costs a
/// fall back to the runner, never correctness.
fn numbers_may_diverge(json: &[u8]) -> bool {
    negative_zero_may_diverge(json) || long_mantissa_may_diverge(json)
}

/// Whether `json` holds a `-0`, which serde_json makes the float `-0.0` and
/// simd-json the integer `0`.
fn negative_zero_may_diverge(json: &[u8]) -> bool {
    memchr::memmem::find_iter(json, b"-0").any(|index| {
        // A number's minus sign is never preceded by a digit — it follows a
        // structural character, whitespace, an exponent, or starts the input —
        // which rules out the hyphens of a date at one byte.
        (index == 0 || !json[index - 1].is_ascii_digit())
            && !matches!(json.get(index + 2), Some(b'0'..=b'9' | b'.' | b'e' | b'E'))
    })
}

/// Whether `json` holds a fractional literal with 16+ significant digits, where
/// the two parsers can round the last bit differently.
fn long_mantissa_may_diverge(json: &[u8]) -> bool {
    memchr::memchr_iter(b'.', json).any(|dot| {
        // A digit must border the point on both sides for this to be a number:
        // JSON spells a fraction as a point followed by at least one digit. Two
        // byte tests are what keeps the dots of URLs and names off the walks
        // below.
        if dot == 0
            || !json[dot - 1].is_ascii_digit()
            || !json.get(dot + 1).is_some_and(u8::is_ascii_digit)
        {
            return false;
        }

        let before = json[..dot]
            .iter()
            .rev()
            .take(DIGIT_SCAN_CAP)
            .take_while(|byte| byte.is_ascii_digit())
            .count();
        let after = json[dot + 1..]
            .iter()
            .take(DIGIT_SCAN_CAP)
            .take_while(|byte| byte.is_ascii_digit())
            .count();

        before + after >= 16
    })
}

/// Deserializes JSON bytes into a single `Value`, growing the stack for deeply
/// nested input. Parses from `&str` so serde_json can slice string values
/// without re-validating UTF-8.
///
/// The whole input must be that one document: anything but trailing whitespace
/// after it is an error rather than being quietly dropped.
fn deserialize(json: &[u8]) -> Result<Value, JqlRunnerError> {
    let json = str::from_utf8(json).map_err(|_| JqlRunnerError::DeserializationError)?;

    let mut deserializer = serde_json::Deserializer::from_str(json);

    deserializer.disable_recursion_limit();

    let value = Value::deserialize(serde_stacker::Deserializer::new(&mut deserializer))
        .map_err(|_| JqlRunnerError::DeserializationError)?;

    deserializer
        .end()
        .map_err(|_| JqlRunnerError::DeserializationError)?;

    Ok(value)
}

/// Returns `true` when the query only uses operators the tape evaluator handles
/// and its first selector narrows the input (so building the tape pays off).
fn can_lazy(tokens: &[Token]) -> bool {
    // A leading flatten/pipe-out builds the tape only to discard it.
    let usable_first = tokens
        .iter()
        .find(|token| !matches!(token, Token::GroupSeparator))
        .is_some_and(|token| !matches!(token, Token::FlattenOperator | Token::PipeOutOperator));

    // Nested pipe-ins have subtle semantics; leave more than one to the runner.
    let pipe_ins = tokens
        .iter()
        .filter(|token| matches!(token, Token::PipeInOperator))
        .count();

    usable_first
        && pipe_ins <= 1
        && !tokens
            .iter()
            .any(|token| matches!(token, Token::FlattenOperator))
}

/// Evaluates the token groups against the tape root, mirroring
/// [`runner::token`]: a single group yields its value, multiple groups yield an
/// array of per-group values.
fn eval(tokens: &[Token], root: TapeValue) -> Result<Value, JqlRunnerError> {
    let groups = split(tokens);

    if groups.len() == 1 {
        return eval_group(&groups[0], root);
    }

    groups
        .iter()
        .map(|group| eval_group(group, root))
        .collect::<Result<Vec<Value>, _>>()
        .map(Value::Array)
}

/// A position in the evaluation: still a view into the tape, or a value the
/// previous operator produced.
enum Cursor<'tape, 'input> {
    Tape(TapeValue<'tape, 'input>),
    Owned(Value),
}

impl Cursor<'_, '_> {
    fn into_value(self) -> Value {
        match self {
            Cursor::Tape(value) => materialize(value),
            Cursor::Owned(value) => value,
        }
    }
}

/// Evaluates a group's tokens starting from the tape root.
fn eval_group(tokens: &[&Token], root: TapeValue) -> Result<Value, JqlRunnerError> {
    eval_tokens(tokens, Cursor::Tape(root))
}

/// Folds tokens over a cursor. `|>` maps the tokens up to the matching `<|` (or
/// the end) over each element; `<|` without a preceding `|>` is an error. Once an
/// operator the tape path does not implement is reached, the remaining tokens are
/// handed to [`runner`].
fn eval_tokens<'tape, 'input>(
    tokens: &[&Token],
    mut cursor: Cursor<'tape, 'input>,
) -> Result<Value, JqlRunnerError> {
    let mut index = 0;

    while let Some(&token) = tokens.get(index) {
        match token {
            Token::PipeInOperator => {
                let mut elements = cursor_elements(cursor).map_err(JqlRunnerError::PipeInError)?;

                // `group_runner` only clears its `piped` flag from inside the
                // per-element loop, so piping into an empty array leaves it set
                // and every later token — `<|` included — maps over nothing.
                if elements.is_empty() {
                    return Ok(Value::Array(Vec::new()));
                }

                let rest = &tokens[index + 1..];
                let body_len = rest
                    .iter()
                    .position(|token| matches!(token, Token::PipeOutOperator))
                    .unwrap_or(rest.len());

                // Apply each token of the body to every element before moving on
                // to the next, the way `group_runner` does, so that a failing
                // document reports the same token's error.
                for &token in &rest[..body_len] {
                    elements = elements
                        .into_iter()
                        .map(|element| advance(element, token))
                        .collect::<Result<Vec<_>, _>>()?;
                }

                let mapped = elements.into_iter().map(Cursor::into_value).collect();

                cursor = Cursor::Owned(Value::Array(mapped));
                index += 1 + body_len;

                // Consume the matching `<|`, if any.
                if matches!(tokens.get(index), Some(Token::PipeOutOperator)) {
                    index += 1;
                }
            }

            Token::PipeOutOperator => return Err(JqlRunnerError::PipeOutError),

            Token::LensSelector(lenses) => {
                let elements =
                    cursor_elements(cursor).map_err(JqlRunnerError::InvalidArrayError)?;

                let kept = elements
                    .into_iter()
                    .filter(|element| lenses_match(lenses, element))
                    .map(Cursor::into_value)
                    .collect();

                cursor = Cursor::Owned(Value::Array(kept));
                index += 1;
            }

            _ => {
                cursor = match cursor {
                    Cursor::Owned(value) => {
                        return runner::group_runner(&tokens[index..], &value);
                    }
                    Cursor::Tape(value) => match step(value, token)? {
                        Some(next) => next,
                        None => {
                            return runner::group_runner(&tokens[index..], &materialize(value));
                        }
                    },
                };

                index += 1;
            }
        }
    }

    Ok(cursor.into_value())
}

/// Applies one token to one cursor, falling back to [`runner`] when the tape
/// declines it.
fn advance<'tape, 'input>(
    cursor: Cursor<'tape, 'input>,
    token: &Token,
) -> Result<Cursor<'tape, 'input>, JqlRunnerError> {
    match cursor {
        Cursor::Owned(value) => runner::group_runner(&[token], &value).map(Cursor::Owned),
        Cursor::Tape(value) => match step(value, token)? {
            Some(next) => Ok(next),
            None => runner::group_runner(&[token], &materialize(value)).map(Cursor::Owned),
        },
    }
}

/// Splits a cursor into per-element cursors. `Err` carries the value when it is
/// not an array, so the caller can pick the right error variant.
fn cursor_elements<'tape, 'input>(
    cursor: Cursor<'tape, 'input>,
) -> Result<Vec<Cursor<'tape, 'input>>, Value> {
    match cursor {
        Cursor::Tape(value) => value.as_array().map_or_else(
            || Err(materialize(value)),
            |array| Ok(array.iter().map(Cursor::Tape).collect()),
        ),
        Cursor::Owned(Value::Array(items)) => Ok(items.into_iter().map(Cursor::Owned).collect()),
        Cursor::Owned(other) => Err(other),
    }
}

/// Whether any lens matches `element`, mirroring `get_array_lenses`: run the
/// lens' tokens against the element and, if that succeeds, test the optional
/// value; a lens with no value matches on presence alone.
fn lenses_match(lenses: &[Lens], element: &Cursor) -> bool {
    lenses.iter().any(|lens| {
        let (tokens, expected) = lens.get_ref();
        let tokens: Vec<&Token> = tokens.iter().collect();

        let selected = match element {
            Cursor::Tape(value) => eval_tokens(&tokens, Cursor::Tape(*value)),
            Cursor::Owned(value) => runner::group_runner(&tokens, value),
        };

        selected.is_ok_and(|value| match expected {
            Some(LensValue::Bool(expected)) => value.as_bool() == Some(*expected),
            Some(LensValue::Null) => value.is_null(),
            Some(LensValue::Number(expected)) => value.as_u64() == Some(*expected as u64),
            Some(LensValue::String(expected)) => value == *expected,
            None => true,
        })
    })
}

/// Applies one token to a tape view. `Ok(None)` means the tape path does not
/// implement this token and the caller should fall back to [`runner`].
#[allow(clippy::too_many_lines)]
fn step<'tape, 'input>(
    value: TapeValue<'tape, 'input>,
    token: &Token,
) -> Result<Option<Cursor<'tape, 'input>>, JqlRunnerError> {
    // serde_json keeps only the last value for a repeated key; the tape keeps
    // every entry. Any object operator on such an object would diverge, so
    // delegate: `materialize` collapses duplicates the same way serde_json does.
    if operates_on_object(token)
        && let Some(object) = value.as_object()
        && has_duplicate_keys(&object)
    {
        return Ok(None);
    }

    let cursor = match token {
        Token::KeySelector(key) => {
            let Some(object) = value.as_object() else {
                return Err(JqlRunnerError::InvalidObjectError(materialize(value)));
            };

            // Take the last match rather than `Object::get`'s first: serde_json
            // keeps the last value for a repeated key.
            let selected = object
                .iter()
                .filter(|(candidate, _)| candidate == key)
                .map(|(_, child)| child)
                .last();

            Cursor::Tape(selected.ok_or_else(|| JqlRunnerError::KeyNotFoundError {
                key: (*key).to_string(),
                parent: materialize(value),
            })?)
        }

        Token::ArrayIndexSelector(indexes) if indexes.len() == 1 => {
            let index: usize = indexes[0].into();

            Cursor::Tape(value.get_idx(index).ok_or_else(|| {
                JqlRunnerError::IndexOutOfBoundsError {
                    index,
                    parent: materialize(value),
                }
            })?)
        }

        Token::ArrayIndexSelector(indexes) => {
            let selected = indexes
                .iter()
                .map(|index| {
                    let index: usize = (*index).into();

                    value.get_idx(index).map(materialize).ok_or_else(|| {
                        JqlRunnerError::IndexOutOfBoundsError {
                            index,
                            parent: materialize(value),
                        }
                    })
                })
                .collect::<Result<Vec<Value>, _>>()?;

            Cursor::Owned(Value::Array(selected))
        }

        Token::ArrayRangeSelector(range) => {
            let Some(array) = value.as_array() else {
                return Err(JqlRunnerError::InvalidArrayError(materialize(value)));
            };

            let Some(len) = NonZeroUsize::new(array.len()) else {
                return Ok(Some(Cursor::Owned(Value::Array(Vec::new()))));
            };

            let (start, end) = range.to_boundaries(len);

            if start >= len.get() || end >= len.get() {
                return Err(JqlRunnerError::RangeOutOfBoundsError {
                    start,
                    end,
                    parent: materialize(value),
                });
            }

            let (low, high, reverse) = span(start, end);
            let mut selected: Vec<Value> = array
                .iter()
                .skip(low)
                .take(high - low + 1)
                .map(materialize)
                .collect();

            if reverse {
                selected.reverse();
            }

            Cursor::Owned(Value::Array(selected))
        }

        Token::ObjectIndexSelector(indexes) => {
            let Some(object) = value.as_object() else {
                return Err(JqlRunnerError::InvalidObjectError(materialize(value)));
            };

            if object.is_empty() {
                return Ok(Some(Cursor::Owned(Value::Object(Map::new()))));
            }

            let Some(&max) = indexes.iter().max() else {
                // The parser never emits an empty index list.
                return Ok(None);
            };
            let max: usize = max.into();

            if max >= object.len() {
                return Err(JqlRunnerError::IndexOutOfBoundsError {
                    index: max,
                    parent: materialize(value),
                });
            }

            let entries: Vec<(&str, TapeValue)> = object.iter().collect();
            let mut map = Map::with_capacity(indexes.len());

            for index in indexes {
                let index: usize = (*index).into();

                if let Some(&(key, child)) = entries.get(index) {
                    map.insert(key.to_string(), materialize(child));
                }
            }

            Cursor::Owned(Value::Object(map))
        }

        Token::ObjectRangeSelector(range) => {
            let Some(object) = value.as_object() else {
                return Err(JqlRunnerError::InvalidObjectError(materialize(value)));
            };

            let Some(len) = NonZeroUsize::new(object.len()) else {
                return Ok(Some(Cursor::Owned(Value::Object(Map::new()))));
            };

            let (start, end) = range.to_boundaries(len);

            if start >= len.get() || end >= len.get() {
                return Err(JqlRunnerError::RangeOutOfBoundsError {
                    start,
                    end,
                    parent: materialize(value),
                });
            }

            let (low, high, reverse) = span(start, end);
            let mut entries: Vec<(&str, TapeValue)> =
                object.iter().skip(low).take(high - low + 1).collect();

            if reverse {
                entries.reverse();
            }

            Cursor::Owned(Value::Object(
                entries
                    .into_iter()
                    .map(|(key, child)| (key.to_string(), materialize(child)))
                    .collect(),
            ))
        }

        Token::MultiKeySelector(keys) => {
            let Some(object) = value.as_object() else {
                return Err(JqlRunnerError::InvalidObjectError(materialize(value)));
            };

            let mut missing: Vec<String> = keys
                .iter()
                .filter(|key| object.get(**key).is_none())
                .map(|key| (*key).to_string())
                .collect();

            if !missing.is_empty() {
                missing.sort();

                return Err(JqlRunnerError::MultiKeyNotFoundError {
                    keys: missing,
                    parent: materialize(value),
                });
            }

            // Every key is present (checked above); any miss is simply skipped.
            Cursor::Owned(Value::Object(
                keys.iter()
                    .filter_map(|key| {
                        object
                            .get(*key)
                            .map(|child| ((*key).to_string(), materialize(child)))
                    })
                    .collect(),
            ))
        }

        Token::KeyOperator => Cursor::Owned(if let Some(object) = value.as_object() {
            let mut keys: Vec<&str> = object.keys().collect();

            keys.sort_unstable();

            Value::Array(
                keys.into_iter()
                    .map(|key| Value::String(key.to_string()))
                    .collect(),
            )
        } else if let Some(array) = value.as_array() {
            Value::Array((0..array.len()).map(Value::from).collect())
        } else {
            materialize(value)
        }),

        Token::TruncateOperator => Cursor::Owned(match value.value_type() {
            ValueType::Array => Value::Array(Vec::new()),
            ValueType::Object => Value::Object(Map::new()),
            _ => materialize(value),
        }),

        // `GroupSeparator` is removed by `split`; the pipe and lens operators are
        // handled by `eval_tokens`. Only flatten reaches here, and it delegates.
        Token::GroupSeparator
        | Token::FlattenOperator
        | Token::LensSelector(_)
        | Token::PipeInOperator
        | Token::PipeOutOperator => return Ok(None),
    };

    Ok(Some(cursor))
}

/// Whether `token` reads an object by position or key count, and so cannot be
/// reconciled with serde_json's collapsing of duplicate keys.
///
/// `KeySelector` is absent on purpose: it resolves duplicates itself by taking
/// the last match.
fn operates_on_object(token: &Token) -> bool {
    matches!(
        token,
        Token::MultiKeySelector(_)
            | Token::ObjectIndexSelector(_)
            | Token::ObjectRangeSelector(_)
            | Token::KeyOperator
    )
}

/// Whether any key appears more than once in `object`.
fn has_duplicate_keys(object: &TapeObject) -> bool {
    let mut seen = HashSet::with_capacity(object.len());

    object.keys().any(|key| !seen.insert(key))
}

/// Turns a directional `(start, end)` into an ordered `(low, high, reverse)`,
/// mirroring the runner's natural-order handling.
fn span(start: usize, end: usize) -> (usize, usize, bool) {
    if start < end {
        (start, end, false)
    } else {
        (end, start, true)
    }
}

/// Recursively converts a tape `Value` into a `serde_json::Value`, preserving
/// document key order.
fn materialize(value: TapeValue) -> Value {
    if let Some(object) = value.as_object() {
        let mut map = Map::with_capacity(object.len());

        for (key, child) in &object {
            map.insert(key.to_string(), materialize(child));
        }

        return Value::Object(map);
    }

    if let Some(array) = value.as_array() {
        return Value::Array(array.iter().map(materialize).collect());
    }

    if value.as_null().is_some() {
        return Value::Null;
    }

    if let Some(boolean) = value.as_bool() {
        return Value::Bool(boolean);
    }

    if let Some(unsigned) = value.as_u64() {
        return Value::Number(unsigned.into());
    }

    if let Some(signed) = value.as_i64() {
        return Value::Number(signed.into());
    }

    if let Some(float) = value.as_f64() {
        return Number::from_f64(float).map_or(Value::Null, Value::Number);
    }

    // Remaining case: a string.
    value
        .as_str()
        .map_or(Value::Null, |string| Value::String(string.to_string()))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::raw;
    use crate::errors::JqlRunnerError;

    fn run(query: &str, json: &str) -> Result<serde_json::Value, JqlRunnerError> {
        raw(query, &mut json.as_bytes().to_vec())
    }

    #[test]
    fn tape_leaves_escape_free_input_intact() {
        // `evaluate` tapes backslash-free input in place instead of copying it,
        // which is only sound while unescaping stays the sole writer of the
        // buffer. A simd-json release that writes elsewhere must fail here.
        let mut json = br#"{ "a": [1, -2.5, true, null, "plain", { "b": "c" }] }"#.to_vec();
        let expected = json.clone();

        assert!(simd_json::to_tape(&mut json).is_ok());
        assert_eq!(json, expected);

        // The counterpart: an escape does rewrite the buffer, hence the copy.
        let mut escaped = br#"{ "a": "b\nc" }"#.to_vec();
        let before = escaped.clone();

        assert!(simd_json::to_tape(&mut escaped).is_ok());
        assert_ne!(escaped, before);
    }

    #[test]
    fn check_drill_down() {
        assert_eq!(run(r#""a""b""#, r#"{ "a": { "b": 2 } }"#), Ok(json!(2)));
        assert_eq!(
            run(r#""a"[1]"c""#, r#"{ "a": [{ "c": 0 }, { "c": 1 }] }"#),
            Ok(json!(1))
        );
    }

    #[test]
    fn check_selection_operators() {
        assert_eq!(
            run("[2,0]", r#"["a", "b", "c", "d"]"#),
            Ok(json!(["c", "a"]))
        );
        assert_eq!(
            run("[2:0]", r#"["a", "b", "c"]"#),
            Ok(json!(["c", "b", "a"]))
        );
        assert_eq!(
            run(r#"{"c","a"}"#, r#"{ "a": 1, "b": 2, "c": 3 }"#),
            Ok(json!({ "c": 3, "a": 1 }))
        );
        assert_eq!(
            run(r#""a"@"#, r#"{ "a": { "y": 1, "x": 2 } }"#),
            Ok(json!(["x", "y"]))
        );
        assert_eq!(run(r#""a"!"#, r#"{ "a": [1, 2, 3] }"#), Ok(json!([])));
    }

    #[test]
    fn check_groups() {
        assert_eq!(
            run(r#""a","b""#, r#"{ "a": 1, "b": 2 }"#),
            Ok(json!([1, 2]))
        );
    }

    #[test]
    fn check_errors() {
        assert_eq!(
            run(r#""b""#, r#"{ "a": 1 }"#),
            Err(JqlRunnerError::KeyNotFoundError {
                key: "b".to_string(),
                parent: json!({ "a": 1 }),
            })
        );
        assert_eq!(
            run("[1]", r#"["a"]"#),
            Err(JqlRunnerError::IndexOutOfBoundsError {
                index: 1,
                parent: json!(["a"]),
            })
        );
    }

    #[test]
    fn check_delegates_flatten() {
        assert_eq!(
            run(r#""a"..[0]"#, r#"{ "a": [1, [2], [[3]]] }"#),
            Ok(json!(1))
        );
    }

    #[test]
    fn check_empty_query() {
        assert_eq!(run("", "{}"), Err(JqlRunnerError::EmptyQueryError));
    }
}
