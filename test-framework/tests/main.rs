#![allow(missing_docs)]

// Why define the test files as modules?
// For files in `/tests`, `cargo` compiles EACH test file into a separate binary.
// We prefer one main test file with one binary for a faster build.
// More detail:
// - https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html#Loomings-%F0%9F%90%B3,
// - https://github.com/rust-lang/cargo/pull/5022#issuecomment-364691154

mod e2e;