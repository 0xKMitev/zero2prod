# DB Initialization

```bash
cargo install sqlx-cli --no-default-features --features postgres
chmod +x ./scripts/init_db.sh
SKIP_DOCKER=true/false ./scripts/init_db.sh
```
