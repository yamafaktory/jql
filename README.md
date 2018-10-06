# JQL

> A JSON Query Language CLI tool

## Installation 🚀

```sh
cargo install jql
```

## Usage 🐨

Want to get the version of a NodeJS package.json file?

```sh
jql package.json version
```

You can chain selectors with `.` and numbers to access children and indexes in arrays.

```sh
jql package.json devDependencies.react

jql package.json keywords.3
```

And get some (limited) help for now.

```sh
jql --help
```
