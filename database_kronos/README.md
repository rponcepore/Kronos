# Running Migrator CLI

- Build the entities file (added to readme by me):
    ```sh
    # In backend_kronos directory
    sea-orm-cli generate entity \
        -u postgres://postgres:password@localhost:5432/kronos_db \
        -o src/models/entities \
        --with-serde both
    ```
- Generate a new migration file
    ```sh
    cargo run -- generate MIGRATION_NAME
    ```
- Apply all pending migrations
    ```sh
    cargo run
    ```
    ```sh
    cargo run -- up
    ```
- Apply first 10 pending migrations
    ```sh
    cargo run -- up -n 10
    ```
- Rollback last applied migrations
    ```sh
    cargo run -- down
    ```
- Rollback last 10 applied migrations
    ```sh
    cargo run -- down -n 10
    ```
- Drop all tables from the database, then reapply all migrations
    ```sh
    cargo run -- fresh
    ```
- Rollback all applied migrations, then reapply all migrations
    ```sh
    cargo run -- refresh
    ```
- Rollback all applied migrations
    ```sh
    cargo run -- reset
    ```
- Check the status of all migrations
    ```sh
    cargo run -- status
    ```
For mac users (OLivia)
run this in the migrations folder (once per shell)
# Only need to do this once per shell (or add to ~/.zshrc)
export ZSTD_STATIC=0
export PKG_CONFIG_PATH="/opt/homebrew/opt/zstd/lib/pkgconfig"

# Then in your Rust project folder:
cargo clean
cargo run