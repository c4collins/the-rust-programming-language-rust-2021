cd $1
# Format
rustfmt src/*.rs
# Build
cargo clean
cargo check
cargo fix
cargo build --verbose
cargo doc
cd -

# Run
./$1/target/debug/$2

# TODO: write this in rust