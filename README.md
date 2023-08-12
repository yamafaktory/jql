<div align="center">
  
  ![jql](jql.svg)

---

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/yamafaktory/jql/ci.yml?branch=main&logo=github&style=flat-square)](https://github.com/yamafaktory/jql/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/jql?style=flat-square)](https://crates.io/crates/jql)
[![Docs.rs](https://img.shields.io/docsrs/jql-parser?label=jql-parser%20docs&style=flat-square)](https://docs.rs/jql-parser/latest/jql_parser/)
[![Docs.rs](https://img.shields.io/docsrs/jql-runner?label=jql-runner%20docs&style=flat-square)](https://docs.rs/jql-runner/latest/jql_runner/)
</div>

`jql` is a JSON Query Language tool built with Rust 🦀.

Pronounce it as **jackal** 🐺.

## 📜 Philosophy

- ⚡Be fast
- 🪶 Stay lightweight
- 🎮 Keep its features as simple as possible
- 🧠 Avoid redundancy
- 💡 Provide meaningful error messages
- 🍰 Eat JSON as input, process, output JSON back

## 🚀 Installation

### Alpine Linux

The package is maintained by @jirutka.

```sh
apk add jql
```

### Archlinux

The AUR package is maintained by @barklan.

```sh
yay -S jql
```

### Cargo

```sh
cargo install jql
```

### Fedora

```sh
dnf install jql
```

### FreeBSD

```sh
pkg install jql
```

### Homebrew

```sh
brew install jql
```

### Nix

```sh
nix-env -i jql
```

### openSUSE

```sh
zypper install jql
```

### Manual installation from GitHub

Compiled binary versions are automatically uploaded to GitHub when a new release is made. You can install `jql` manually by [downloading a release](https://github.com/yamafaktory/jql/releases).

## 🛠️ Usage

To make a selection from a JSON input, `jql` expects a **query** as a sequence of **tokens**.

To be fully compliant with the JSON format, `jql` always expect key selectors to be **double-quoted**, see [The JavaScript Object Notation (JSON) Data Interchange Format](https://tools.ietf.org/html/rfc8259#section-13).

```json
{
  ".valid": 1337,
  "": "yeah!",
  "\"": "yup, valid too!"
}
```

Consequently, to be shell compliant, a query must be either enclosed by single quotation marks or every inner double quotation mark must be escaped.

### Separators

#### Group separator

Group separators build up an array from sub-queries.

**JSON input**

```json
{ "a": 1, "b": 2, "c": 3 }
```

**Query**

```sh
'"a","b","c"'
```

**JSON output**

```json
[1, 2, 3]
```

### Selectors

#### Arrays

##### Array index selector

Indexes can be used in arbitrary order.

**JSON input**

```json
[1, 2, 3]
```

**Query**

```sh
'[2,1]'
```

**JSON output**

```json
[3, 2]
```

##### Array range selector

Range can be in natural order `[0:2]`, reversed `[2:0]`, without lower `[:2]` or upper bound `[0:]`.

**JSON input**

```json
[1, 2, 3]
```

**Query**

```sh
'[2:1]'
```

**JSON output**

```json
[3, 2]
```

##### Lens selector

Lens can be a combination of one or more selectors with or an optional value, a value being any of **boolean** | **null** | **number** | **string**.

**JSON input**

```json
[
  { "a": 1, "b": { "d": 2 } },
  { "a": 2, "b": "some" },
  { "a": 2, "b": { "d": null } },
  { "a": 2, "b": true },
  { "c": 3, "b": 4 }
]
```

**Query**

```sh
'|={"b""d"=2, "c"}'
```

**JSON output**

```json
[
  { "a": 1, "b": { "d": 2 } },
  { "c": 3, "b": 4 }
]
```

#### Objects

##### Key selector

Any valid JSON key can be used.

**JSON input**

```json
{ "a": 1, "b": 2, "c": 3 }
```

**Query**

```sh
'"c"'
```

**JSON output**

```json
3
```

##### Multi key selector

Keys can be used in arbitrary order.

**JSON input**

```json
{ "a": 1, "b": 2, "c": 3 }
```

**Query**

```sh
'{"c","a"}'
```

**JSON output**

```json
{ "c": 3, "a": 1 }
```

##### Object index selector

Indexes can be used in arbitrary order.

**JSON input**

```json
{ "a": 1, "b": 2, "c": 3 }
```

**Query**

```sh
'{2,0}'
```

**JSON output**

```json
{ "c": 3, "a": 1 }
```

##### Object range selector

Range can be in natural order `{0:2}`, reversed `{2:0}`, without lower `{:2}` or upper bound `{0:}`.

**JSON input**

```json
{ "a": 1, "b": 2, "c": 3 }
```

**Query**

```sh
'{2:1}'
```

**JSON output**

```json
{ "c": 3, "b": 2 }
```

#### Operators

##### Flatten operator

Flattens arrays and objects.

**JSON input**

```json
[[[[[[[[[[[[[[{ "a": 1 }]]]]]]]]]]]]], [[[[[{ "b": 2 }]]]], { "c": 3 }], null]
```

**Query**

```sh
'..'
```

**JSON output**

```json
[{ "a": 1 }, { "b": 2 }, { "c": 3 }, null]
```

**JSON input**

```json
{ "a": { "c": false }, "b": { "d": { "e": { "f": 1, "g": { "h": 2 } } } } }
```

**Query**

```sh
'..'
```

**JSON output**

```json
{
  "a.c": false,
  "b.d.e.f": 1,
  "b.d.e.g.h": 2
}
```

##### Pipe in operator

Applies the next tokens in parallel on each element of an array.

**JSON input**

```json
{ "a": [{ "b": { "c": 1 } }, { "b": { "c": 2 } }] }
```

**Query**

```sh
'"a"|>"b""c"'
```

**JSON output**

```json
[1, 2]
```

##### Pipe out operator

Stops the parallelization initiated by the pipe in operator.

**JSON input**

```json
{ "a": [{ "b": { "c": 1 } }, { "b": { "c": 2 } }] }
```

**Query**

```sh
'"a"|>"b""c"<|[1]'
```

**JSON output**

```json
2
```

##### Truncate operator

Maps the output into simple JSON primitives **boolean** | **null** | **number** | **string** | **[]** | **{}**.

**JSON input**

```json
{ "a": [1, 2, 3] }
```

**Query**

```sh
'"a"!'
```

**JSON output**

```json
[]
```

## 💻 Shell integration

### How to save the output

```sh
jql '"a"' input.json > output.json
```

### How to read from stdin

```sh
cat test.json | jql '"a"'
```

### Available flags

#### Inline the JSON output

By default, the output is pretty printed in a more human-readable way, this can be disabled.

```sh
-i, --inline
```

#### Read the query from file

The command will read the provided query from a file instead of the stdin.

```sh
-q, --query <FILE>
```

#### Write to stdout without JSON double-quotes

This can be useful to drop the double-quotes surrounding a string primitive.

```sh
-r, --raw-string
```

#### Read a stream of JSON data line by line

This flag is only about reading processing any JSON output streamed line by line (e.g. Docker logs with the `--follow` flag). This is not an option to read an incomplete streamed content (e.g. a very large input).

```sh
-s, --stream
```

#### Validate the JSON data

The command will return a matching exit code based on the validity of the JSON content or file provided.

```sh
-v, --validate
```

#### Print help

```sh
-h, --help
```

#### Print version

```sh
-V, --version
```

#### Help

```sh
jql -h
jql --help
```

## 🦀 Workspace

This project is composed of following crates:

- jql (_binary_)
- [jql-parser](https://docs.rs/jql-parser/latest/jql_parser/) (_library_)
- [jql-runner](https://docs.rs/jql-runner/latest/jql_runner/) (_library_)

## Development

Some commands are available as a `justfile` at the root of the workspace (testing / fuzzing).

### Prerequisites

- [cargo-nextest](https://nexte.st/)
- [just](https://just.systems/man/en/)

### Commands

```sh
just --list
```

## ⚠️ Non-goal

There's no plan to align `jql` with `jq` or any other similar tool.

## ⚡ Performance

Some benchmarks comparing a set of similar functionalities provided by this tool and [jq](https://stedolan.github.io/jq/) are available [here](PERFORMANCE.md).

## 📔 Licenses

- [Apache License, Version 2.0](https://github.com/yamafaktory/jql/blob/main/LICENSE-APACHE)
- [MIT license](https://github.com/yamafaktory/jql/blob/main/LICENSE-MIT)
