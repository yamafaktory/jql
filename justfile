# Audit.
audit:
  cargo audit

# Clippy.
clippy:
  cargo clippy

# Fuzz parser.
fuzz:
  cargo fuzz run fuzz_parser

# Run all tests.
test:
  cargo nextest run

# Run binary tests.
test-bin:
  cargo nextest run -p jql

# Run parser tests.
test-parser:
  cargo nextest run -p jql-parser

# Run runner tests.
test-runner:
  cargo nextest run -p jql-runner
