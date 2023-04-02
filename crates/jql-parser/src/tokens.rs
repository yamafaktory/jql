use std::{
    fmt,
    string::ToString,
};

/// `Index` used for arrays and objects.
/// Internally mapped to a `u32` with the newtype pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Index(pub(crate) u32);

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index ({})", self.0)
    }
}

/// `Range` used for arrays and objects.
/// Internally mapped to a tuple of `Option` of `Index`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range(pub(crate) Option<Index>, pub(crate) Option<Index>);

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format_bound = |bound: &Option<Index>| match bound {
            Some(index) => index.to_string(),
            None => String::new(),
        };

        write!(
            f,
            "Range [{}:{}]",
            format_bound(&self.0),
            format_bound(&self.1)
        )
    }
}

/// `Lens` used for `LensSelector`.
/// Internally mapped to a tuple of `Option` of `Index`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lens<'a>(pub(crate) &'a str, pub(crate) Option<LensValue<'a>>);

impl<'a> fmt::Display for Lens<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.0,
            match &self.1 {
                Some(lens_value) => {
                    lens_value.to_string()
                }
                None => String::new(),
            }
        )
    }
}

/// Lens value type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LensValue<'a> {
    /// Variant for a JSON number.
    Number(u32),
    /// Variant for JSON null.
    Null,
    /// Variant for a JSON string.
    String(&'a str),
}

impl<'a> fmt::Display for LensValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LensValue::Null => {
                write!(f, "null")
            }
            LensValue::String(string) => write!(f, "{string}"),
            LensValue::Number(number) => {
                write!(f, "{number}")
            }
        }
    }
}

/// Parser tokens type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    /// Array index selector.
    ArrayIndexSelector(Vec<Index>),
    /// Array range selector.
    ArrayRangeSelector(Range),
    /// Flatten operator.
    FlattenOperator,
    /// Group separator.
    GroupSeparator,
    /// Key selector.
    KeySelector(&'a str),
    /// Lens selector.
    LensSelector(Vec<Lens<'a>>),
    /// Multi key selector.
    MultiKeySelector(Vec<&'a str>),
    /// Object index selector.
    ObjectIndexSelector(Vec<Index>),
    /// Object range selector.
    ObjectRangeSelector(Range),
    /// Pipe in operator.
    PipeInOperator,
    /// Pipe out operator.
    PipeOutOperator,
    /// Truncate operator.
    TruncateOperator,
}

impl<'a> Token<'a> {
    fn get_name(&self) -> &'a str {
        match self {
            Token::ArrayIndexSelector(_) => "Array Index Selector",
            Token::ArrayRangeSelector(_) => "Array Range Selector",
            Token::FlattenOperator => "Flatten Operator",
            Token::GroupSeparator => "Group Separator",
            Token::KeySelector(_) => "Key Selector",
            Token::LensSelector(_) => "Lens Selector",
            Token::MultiKeySelector(_) => "Multi Key Selector",
            Token::ObjectIndexSelector(_) => "Object Index Selector",
            Token::ObjectRangeSelector(_) => "Object Range Selector",
            Token::PipeInOperator => "Pipe In Operator",
            Token::PipeOutOperator => "Pipe Out Operator",
            Token::TruncateOperator => "Truncate Operator",
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::ArrayIndexSelector(indexes) | Token::ObjectIndexSelector(indexes) => {
                let formatted_indexes = indexes
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ");

                write!(f, "{} [{formatted_indexes}]", self.get_name())
            }
            Token::ArrayRangeSelector(range) | Token::ObjectRangeSelector(range) => {
                write!(f, "{} {}", self.get_name(), range)
            }
            Token::KeySelector(key) => {
                write!(f, "{} {key}", self.get_name())
            }
            Token::LensSelector(lenses) => {
                let formatted_indexes = lenses
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ");

                write!(f, "{} [{formatted_indexes}]", self.get_name())
            }
            Token::MultiKeySelector(multi_key) => {
                let formatted_keys = multi_key.join(", ");

                write!(f, "{} {formatted_keys}", self.get_name())
            }
            Token::FlattenOperator
            | Token::GroupSeparator
            | Token::PipeInOperator
            | Token::PipeOutOperator
            | Token::TruncateOperator => {
                write!(f, "{}", self.get_name())
            }
        }
    }
}

/// Trait used to expose custom display methods.
pub trait View {
    /// Returns a stringified version of `self`.
    fn stringify(&self) -> String;
}

impl<'a, T: AsRef<[Token<'a>]>> View for T {
    fn stringify(&self) -> String {
        self.as_ref()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ")
    }
}
