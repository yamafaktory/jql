# jql-runner

## About

This crate is a workspace dependency of the [jql](https://github.com/yamafaktory/jql) tool.

## Features

- Raw runner (string slice as input)
- Token runner (tokens as input)
- Lazy runner (mutable byte slice as input; scans drill-down queries into a
  simd-json tape and materializes only the selected subtree, falling back to the
  token runner otherwise)
- Errors

## License

For licensing information, please check the [workspace root](https://github.com/yamafaktory/jql).
