# Tests

Cargo runs tests that live inside each crate. `cargo test --workspace --locked` is the local unit-test command. It does not need Docker or PostgreSQL, and it does not run ignored tests.

`crates/trading-domain/tests/adfr_reconciliation.rs` checks the ADFR register, the source manifest, and the 2026-11-15 decision record. It does not implement trading behavior, and it does not pass B10-T01 through B10-T12.

The PostgreSQL test is `crates/trading-persistence/tests/connectivity.rs`. `cargo test --workspace` leaves it ignored. GitHub Actions runs it with `--ignored` when `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL` names a database whose name contains `_test`. The command is documented in [the development guide](../docs/development/README.md).
