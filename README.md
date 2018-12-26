# JQL

> A JSON Query Language CLI tool

## 📜 Core philosophy of jql:
- 🎮 Keep its features as simple as possible
- 🧠 Avoid redundancy
- 💡 Provide meaningful error messages
- ↔️ Eat JSON as input, process, output JSON back

## 🚀 Installation

```sh
cargo install jql
```

## 🛠️ Usage

If you find some of the following examples confusing, please have a look at [The JavaScript Object Notation (JSON) Data Interchange Format](https://tools.ietf.org/html/rfc8259#section-13).

### Accessing the root

```json
"This is a valid JSON text with one value"
```

```sh
jql '.' example.json
```

```json
"This is a valid JSON text with one value"
```

### Accessing a child

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

### Accessing an index

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

### Accessing a range

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

### Accessing an array

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

### Flattening arrays

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
jql -i 'some.selector' example.json
jql --inline 'some.selector' example.json
```

## 🍿 Library

This crate is both a binary (the CLI tool) and a library that can be directly used https://docs.rs/crate/jql/.
