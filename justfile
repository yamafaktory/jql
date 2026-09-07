# Audit.
audit:
  cargo audit

# Clippy.
clippy:
  cargo clippy

# Format.
fmt:
  cargo fmt --all

# Fuzz parser.
fuzz:
  cargo fuzz run fuzz_parser

# Fuzz the lazy evaluator against the runner.
fuzz-lazy:
  cargo fuzz run fuzz_lazy

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
