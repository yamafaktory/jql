# JQL

> A JSON Query Language CLI tool

## Installation 🚀

```sh
cargo install jql
```

## Usage 🐨

If you find some of the following examples confusing, please have a look at [The JavaScript Object Notation (JSON) Data Interchange Format](https://tools.ietf.org/html/rfc8259#section-13).

### Accessing the root

```json
"This is a valid JSON text with one value"
```

```sh
jql example.json ''
```

```json
"This is a valid JSON text with one value"
```

Please note that the following is also valid:

```sh
jql example.json '.'
```

```json
"This is a valid JSON text with one value"
```

### Accessing a children

```json
{
  "some": {
    "property": "yay!"
  }
}
```

```sh
jql example.json '"some"."property"'
```

```json
"yay!"
```

Please note that the following is also valid:

```sh
jql example.json 'some.property'
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
jql example.json 'primes.0'
```

```json
7
```

Please note that the following is also valid:

```sh
jql example.json 'primes."0"'
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
jql example.json 'cats.1:2'
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
jql example.json 'cats.2:1'
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
jql example.json 'cats.2:1.1:0'
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
jql example.json 'cats.2:1.0.third'
```

```json
"Misty"
```

### Special characters

```json
{
  ".valid": 1337
}
```

```sh
jql example.json '".valid"'
```

```json
1337
```

## Help 📖

```sh
jql --help
```
