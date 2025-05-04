# Fake Data

A simple Rust script to create synthetic JSON user data.

## Running

```sh
# Generate 100 user records.
cargo run 100                  

# Output 1m user records to a file.
cargo build --release 
target/release/fake-data 1000000 > users.json
```

On my laptop, generating 1m records takes <2s.
