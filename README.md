# DB Initialization

```bash
cargo install sqlx-cli --no-default-features --features postgres
chmod +x ./scripts/init_db.sh
SKIP_DOCKER=true/false ./scripts/init_db.sh
```

# Running test coverage

```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
```