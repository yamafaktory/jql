# JQL ![Build Status](https://github.com/yamafaktory/jql/workflows/ci/badge.svg)

> A JSON Query Language CLI tool built with Rust 🦀

## 📜 Core philosophy

- 📦 Stay lightweight
- 🎮 Keep its features as simple as possible
- 🧠 Avoid redundancy
- 💡 Provide meaningful error messages
- ↔️ Eat JSON as input, process, output JSON back

## 🚀 Installation

### Cargo

```sh
cargo install jql
```

### Archlinux

The AUR package is maintained by @frol.

```sh
yay -S jql
```

## 🛠️ Usage

If you find some of the following examples confusing, please have a look at [The JavaScript Object Notation (JSON) Data Interchange Format](https://tools.ietf.org/html/rfc8259#section-13).

### Root selection

```json
"This is a valid JSON text with one value"
```

```sh
jql '.' example.json
```

```json
"This is a valid JSON text with one value"
```

### Child selection

```json
{
  "some": {
    "property": "yay!"
  }
}
```

```sh
jql '"some"."property"' example.json
```

```json
"yay!"
```

### Index selection

```json
{
  "primes": [7, 11, 13]
}
```

```sh
jql '"primes".[0]' example.json
```

```json
7
```

Please note that the following is also valid:

```sh
jql '"primes"[0]"' example.json
```

```json
7
```

You can also select a set of indexes:

```sh
jql '"primes".[2,0]' example.json
```

```json
[
  13,
  7
]
```

### Range selection

```json
{
  "cats": [{ "first": "Pixie" }, { "second": "Kitkat" }, { "third": "Misty" }]
}
```

```sh
jql '"cats".[1:2]' example.json
```

```json
[
  {
    "second": "Kitkat"
  },
  {
    "third": "Misty"
  }
]
```

Please note that you can reverse it:

```sh
jql '"cats".[2:1]' example.json
```

```json
[
  {
    "third": "Misty"
  },
  {
    "second": "Kitkat"
  }
]
```

Bonus, you can do it again to get it back:

```sh
jql '"cats".[2:1].[1:0]' example.json
```

```json
[
  {
    "second": "Kitkat"
  },
  {
    "third": "Misty"
  }
]
```

Please note that you can still access the children:

```sh
jql '"cats".[2:1].[0]."third"' example.json
```

```json
"Misty"
```

You can also use the start or the end position as a range selector:

```sh
jql '"cats".[1:]' example.json
```

```json
[
  {
    "second": "Kitkat"
  },
  {
    "third": "Misty"
  }
]
```

```sh
jql '"cats".[:1]' example.json
```

```json
[
  {
    "first": "Pixie"
  },
  {
    "second": "Kitkat"
  }
]
```

### Array selection

```json
{
  "primes": [7, 11, 13]
}
```

```sh
jql '"primes".[]' example.json
```

```json
[
  7,
  11,
  13
]
```

Please note that this is basically an alias for a full range selection:

```sh
jql '"primes".[0:2]' example.json
```

### Property selection

```json
{
  "object": { "a": 1, "b": 2, "c": 3 }
}
```

```sh
jql '"object".{"a","c"}' example.json
```

```json
{
  "a": 1,
  "c": 3
}
```

### Multi-selection

```json
{
  "one": [1, 2, 3],
  "two": 2,
  "three": 3
}
```

```sh
jql '"one".[2:0],"two","three"' example.json
```

```json
[
  [
    3,
    2,
    1
  ],
  2,
  3
]
```

### Filter

```json
{
  "laptops": [
    {
      "laptop": {
        "brand": "Apple",
        "options": ["a", "b", "c"]
      }
    },
    {
      "laptop": {
        "brand": "Asus",
        "options": ["d", "e", "f"]
      }
    }
  ]
}
```

```sh
jql '"laptops"|"laptop"' example.json
```

```json
[
  {
    "brand": "Apple",
    "options": ["a", "b", "c"]
  },
  {
    "brand": "Asus",
    "options": ["d", "e", "f"]
  }
]
```

You can also combine a filter with a child selection, a multi-selection and ranges at the same time:

```sh
jql '"laptops"|"laptop"."brand"' example.json
```

```json
[
  "Apple",
  "Asus"
]
```

```sh
jql '"laptops".[1:0]|"laptop"."brand","laptops"|"laptop"."brand"' example.json
```

```json
[
  [
    "Asus",
    "Apple"
  ],
  [
    "Apple",
    "Asus"
  ]
]
```

Please note that you can combine filters to achieve the same result:

```sh
jql '"laptops".[1:0]|"laptop"|"brand","laptops"|"laptop"|"brand"' example.json
```

```json
[
  "Apple",
  "Asus"
]
```

### Flatten arrays

```json
{
  "dna": [[[[["c", "a", "c"]]]], "g", "t", [[["a", ["t"]]]]]
}
```

```sh
jql '.."dna"' example.json
```

```json
[
  "c",
  "a",
  "c",
  "g",
  "t",
  "a",
  "t"
]
```

### Truncate

The truncate selector `!` can be used to stop walking the children's values and to explore an unknown JSON file / structure.
Each children is then transformed into a JSON primitive for convenience, e.g.:

primitive | value                        | result
--------- | ---------------------------- | -------
object    | `{ "a": 1, "b": 2, "c": 3 }` | `{}`
array     | `[1, 2, 3]`                  | `[]`
string    | `"foo"`                      | `"foo"`
number    | `666`                        | `666`
null      | `null`                       | `null`

```json
{
  "foo": {
    "a": null,
    "b": "bar",
    "c": 1337,
    "d": {
      "woot": [
        1,
        2,
        3
      ]
    }
  }
}
```

```sh
jql '.!' example.json
```

```json
{
  "foo": {}
}
```

```sh
jql '"foo"!' example.json
```

```json
{
  "a": null,
  "b": "bar",
  "c": 1337,
  "d": {}
}
```

### Special characters

```json
{
  ".valid": 1337,
  "": "yeah!",
  "\"": "yup, valid too!"
}
```

```sh
jql '".valid"' example.json
```

```json
1337
```

```sh
jql '""' example.json
```

```json
"yeah!"
```

```sh
jql '"\""' example.json
```

```json
"yup, valid too!"
```

## 💻 Shell integration

### How to save the output

```sh
jql '"foo"."bar"' input.json > output.json
```

### How to read from stdin

```sh
cat example.json | jql '"foo"."bar"'
```

### Available flags 🤖

#### Help

```sh
jql -h
jql --help
```

#### Version

```sh
jql -V
jql --version
```

#### Inlining the JSON output

```sh
jql -i '"some"."selector"' example.json
jql --inline '"some"."selector"' example.json
```

#### Raw output

Use the `raw-output` flag on a string selection to directly return the raw string without JSON double-quotes:

```sh
echo "{\"foo\":\"bar\"}" | jql --raw-output '"foo"'
bar
echo "{\"foo\":\"bar\"}" | jql -r '"foo"'
bar
```

#### Streaming

Use the `stream` flag to read a stream of JSON lines:

```sh
while true; do echo '{"foo": 2}'; sleep 1; done | cargo run '.!' --stream
```

```sh
while true; do echo '{"foo": 2}'; sleep 1; done | cargo run '.!' -s
```

Please note that this option is only about reading valid JSON output streamed line by line (e.g. Docker logs with the `--follow` flag). This is not an option to read an incomplete streamed content (e.g. a very large input)!

## 🍿 Library

This crate is both a binary (the CLI tool) and a library that can be directly used https://docs.rs/crate/jql/.


## ⚡ Performance

Some benchmarks comparing a set of similar functionalities provided by this tool and [jq](https://stedolan.github.io/jq/) are available [here](PERFORMANCE.md).