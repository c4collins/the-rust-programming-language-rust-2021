cd $1
# Format
rustfmt src/*.rs
# Build
cargo clean
cargo fix
cargo build --verbose
cd -

# Run
./$1/target/debug/$2

# TODO: write this in rust