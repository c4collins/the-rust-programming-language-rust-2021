cd $1
# Format
rustfmt src/*.rs
# Test
cargo test
cd -