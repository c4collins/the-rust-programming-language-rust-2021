cd $1

# Format
rustfmt src/*.rs

# Test
cargo test
# cargo test -- --show-output           # Show all the stdout messages in the test/tested functions
# cargo test -- --test-threads=1        # Don't parallelize test running
# cargo test <test_match>               # Only run tests that contain <test_match>
# cargo test -- --ignored               # Just ignored tests
# cargo test -- --include-ignored       # Run tests but include ignored tests
# cargo test --test <filename>          # Run all tests in <filename>

cd -