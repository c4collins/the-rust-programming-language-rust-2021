cd $1
# Format
rustfmt src/*.rs
# Build
cargo clean
cargo check
cargo fix
cargo doc --open
cd -

# TODO: write this in rust