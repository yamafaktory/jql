# jql

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/yamafaktory/jql/ci?style=for-the-badge) ![Crates.io](https://img.shields.io/crates/v/jql?style=for-the-badge) [![contribute.design](https://contribute.design/api/shield/yamafaktory/jql)](https://contribute.design/yamafaktory/jql)

> A JSON Query Language CLI tool built with Rust 🦀

## 📜 Core philosophy

- 📦 Stay lightweight
- 🎮 Keep its features as simple as possible
- 🧠 Avoid redundancy
- 💡 Provide meaningful error messages
- ↔️ Eat JSON as input, process, output JSON back

## ⚠️ Non-goal

This tool has absolutely no plan to be on par with `jq` and such other similar CLI.

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
[13, 7]
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
[7, 11, 13]
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

Property selection can also be used with indexes and ranges. Please note that in this case a remapping/transformation is applied to the JSON data:

```json
{
  "alpha": "red",
  "beta": "green",
  "gamma": "blue"
}
```

```sh
jql '{[2,0,1]}' example.json
```

```json
{
  "2": "blue",
  "0": "red",
  "1": "green"
}
```

```sh
jql '{[1:2]}' example.json
```

```json
{
  "1": "green",
  "2": "blue"
}
```

This is pretty unusual, but it might help in some scenarios when e.g. one wants to extract some properties out of a complex JSON structure based on their order:

```json
{
  "some-property": [
    {
      "key1": [
        {
          "subkey1": "value"
        }
      ],
      "key2": 123
    },
    {
      "key3": [
        {
          "subkey3": "value"
        }
      ],
      "key4": "something"
    }
  ]
}
```

```sh
jql '.."some-property"|{[0]}|"0"' example.json
```

```json
[
  {
    "subkey1": "value"
  },
  {
    "subkey3": "value"
  }
]
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
[[3, 2, 1], 2, 3]
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
["Apple", "Asus"]
```

```sh
jql '"laptops".[1:0]|"laptop"."brand","laptops"|"laptop"."brand"' example.json
```

```json
[
  ["Asus", "Apple"],
  ["Apple", "Asus"]
]
```

Please note that you can combine filters to achieve the same result:

```sh
jql '"laptops".[1:0]|"laptop"|"brand","laptops"|"laptop"|"brand"' example.json
```

```json
[
  ["Asus", "Apple"],
  ["Apple", "Asus"]
]
```

### Flatten

#### Arrays

```json
{
  "dna": [[[[["c", "a", "c"]]]], "g", "t", [[["a", ["t"]]]]]
}
```

```sh
jql '.."dna"' example.json
```

```json
["c", "a", "c", "g", "t", "a", "t"]
```

#### Objects

```json
{
  "test": {
    "foo": {
      "bar": false
    }
  }
}
```

```sh
jql '.."test"' example.json
```

```json
{ "foo.bar": false }
```

### Lens

Lenses enable filtering an array of objects by key, key/value pair or a combination of both. Please
note that only `number`, `string` and `null` primitive can be used as value.

```json
{
  "lenses": [
    { "alpha": 1, "beta": null },
    { "beta": 2 },
    { "gamma": 3, "delta": "something" },
    { "alpha": 7 },
    { "delta": 4 }
  ]
}
```

```sh
jql '"lenses"|={"alpha","delta"}' example.json
```

```json
[
  {
    "alpha": 1,
    "beta": null
  },
  {
    "gamma": 3,
    "delta": "something"
  },
  {
    "alpha": 7
  },
  {
    "delta": 4
  }
]
```

```sh
jql '"lenses"|={"alpha":"7","beta":"null"}' example.json
```

```json
[
  {
    "alpha": 1,
    "beta": null
  },
  {
    "alpha": 7
  }
]
```

```sh
jql '"lenses"|={"delta":"something","alpha"}' example.json
```

```json
[
  {
    "alpha": 1,
    "beta": null
  },
  {
    "gamma": 3,
    "delta": "something"
  },
  {
    "alpha": 7
  }
]
```

### Truncate

The truncate selector `!` can be used to stop walking the children's values and to explore an unknown JSON file / structure.
Each child is then transformed into a JSON primitive for convenience, e.g.:

| primitive | value                        | result  |
| --------- | ---------------------------- | ------- |
| object    | `{ "a": 1, "b": 2, "c": 3 }` | `{}`    |
| array     | `[1, 2, 3]`                  | `[]`    |
| string    | `"foo"`                      | `"foo"` |
| number    | `666`                        | `666`   |
| null      | `null`                       | `null`  |

```json
{
  "foo": {
    "a": null,
    "b": "bar",
    "c": 1337,
    "d": {
      "woot": [1, 2, 3]
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

In order to be fully compliant with JSON's object keys, `jql` always expect selectors to be
**double-quoted**.

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

#### Check

The command will return a matching exit code based on the validity of the JSON content or file provided. No selector is needed in this case!

```sh
jql -c example.json
jql --check example.json
```

Please note that this flag is exclusive.

#### From file

The command will reads the provided selectors from a file rather than from a command line.

```sh
jql -f selector.txt example.json
jql --from-file selector.txt example.json
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
while true; do echo '{"foo": 2}'; sleep 1; done | jql '.!' --stream
```

```sh
while true; do echo '{"foo": 2}'; sleep 1; done | jql '.!' -s
```

Please note that this option is only about reading valid JSON output streamed line by line (e.g. Docker logs with the `--follow` flag). This is not an option to read an incomplete streamed content (e.g. a very large input)!

#### Version

```sh
jql -V
jql --version
```

## 🍿 Library

This crate is both a binary (the CLI tool) and a library that can be directly used https://docs.rs/crate/jql/.

## ⚡ Performance

Some benchmarks comparing a set of similar functionalities provided by this tool and [jq](https://stedolan.github.io/jq/) are available [here](PERFORMANCE.md).
