cd $1
# Format
rustfmt src/*.rs
# Build
cargo clean
cargo fix
cargo build --verbose
cargo run
cd -

# TODO: write this in rust