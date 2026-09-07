# jql-runner

## About

This crate is a workspace dependency of the [jql](https://github.com/yamafaktory/jql) tool.

## Features

- Raw runner (string slice as input)
- Token runner (tokens as input)
- Lazy runner (mutable byte slice as input; evaluates selection queries against
  a simd-json tape and materializes only what the query selects, falling back to
  the token runner for the flatten operator, nested pipes and inputs simd-json
  cannot parse identically), one-shot or holding its scratch space across inputs
- Errors

## License

For licensing information, please check the [workspace root](https://github.com/yamafaktory/jql).
