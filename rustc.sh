rustfmt $1/*.rs
rustc --out-dir $1 $1/$2.rs
./$1/$2

# TODO: write this in rust