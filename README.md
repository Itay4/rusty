# rusty

## Usage
1. Set env var:
```bash
export DATABASE_URL=postgres://root:postgres@localhost:5432/member?sslmode=disable
```
2. Start PostgreSQL:
```bash
docker compose up
```
3. Start the service:
```rust
cargo run
```