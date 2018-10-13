# JQL

> A JSON Query Language CLI tool

## Installation 🚀

```sh
cargo install jql
```

## Usage 🐨

Want to get the version of a NodeJS `package.json` file?

```sh
jql package.json 'version'
```

You can chain selectors with `.` and numbers to access children and indexes in arrays.

```sh
jql package.json 'devDependencies.react'

jql package.json 'keywords.3'
```

Given the following JSON file:

```sh
{
    ".valid": 1337
}
```

You can access the `.valid` key as follow:

```sh
jql package.json '".valid"'
```

And get some (limited) help for now.

```sh
jql --help
```
