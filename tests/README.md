# Tests

Cargo runs tests that live inside each crate. `cargo test --workspace` is the unit-test command and does not need Docker.

The PostgreSQL test is `crates/trading-persistence/tests/connectivity.rs`. It is ignored until someone runs it with `--ignored` and `DATABASE_URL`. The command is documented in [the development guide](../docs/development/README.md).
