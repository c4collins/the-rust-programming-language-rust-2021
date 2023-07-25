cd $1
# Format
rustfmt src/*.rs
# Build
cargo build --verbose --release
cargo run
cd -

# TODO: write this in rust