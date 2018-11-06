# JQL

> A JSON Query Language CLI tool

The core philosophy of jql:
- Keep its features as simple as possible
- Avoid redunduncy
- Provide meaningful error messages
- Eat JSON as input, process, output JSON back

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
jql example.json '.'
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

### Multi-selection

```json
{
  "one": [1, 2, 3],
  "two": 2,
  "three": 3
}
```

```sh
jql example.json 'one.2:0,two,three'
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
jql example.json 'laptops|laptop'
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
jql example.json 'laptops|laptop.brand'
```

```json
[
  "Apple",
  "Asus"
]
```

```sh
jql example.json 'laptops.1:0|laptop.brand,laptops|laptop.brand'
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

Please note that only one filter can be applied at the group level. If you want to achieve something like `.|foo|bar`, simply go for `.|foo.bar`.

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

## How to save the output

```sh
jql input.json 'foo.bar' > output.json
```

## Help 📖

```sh
jql --help
```
