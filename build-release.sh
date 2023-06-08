cd $1
# Format
rustfmt src/*.rs
# Build
cargo build --verbose --release
cargo doc
cd -

# Run
./$1/target/debug/$2

# TODO: write this in rust