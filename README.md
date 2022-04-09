# zero2prod

Run before commit:
```sh
cargo test
cargo tarpaulin --ignore-tests
cargo clippy
cargo audit
```