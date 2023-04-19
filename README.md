![jql](jql.svg)

---

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/yamafaktory/jql/ci.yml?branch=main&logo=github&style=flat-square)](https://github.com/yamafaktory/jql/actions/workflows/ci.yml) [![Crates.io](https://img.shields.io/crates/v/jql?style=flat-square)](https://crates.io/crates/jql)

`jql` is a JSON Query Language tool built with Rust 🦀.

Pronounce it as **jackal** 🐺.

## 📜 Philosophy

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

To be fully compliant with the JSON format, `jql` always expect key selectors to be **double-quoted**, see [The JavaScript Object Notation (JSON) Data Interchange Format](https://tools.ietf.org/html/rfc8259#section-13).

```json
{
  ".valid": 1337,
  "": "yeah!",
  "\"": "yup, valid too!"
}
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

## 🍿 Workspace

This crate is both a binary (the CLI tool) and a library that can be directly used https://docs.rs/crate/jql/.

## ⚠️ Non-goal

There's no plan to align `jql` with `jq` or any other similar tool.

## ⚡ Performance

Some benchmarks comparing a set of similar functionalities provided by this tool and [jq](https://stedolan.github.io/jq/) are available [here](PERFORMANCE.md).
