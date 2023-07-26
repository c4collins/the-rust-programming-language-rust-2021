//! There isn't really any documentation to speak of before this, since this is where I learned about documentation

pub mod art;

use art::mix;
use art::PrimaryColor;

pub fn run() {
    use_add_one();
    use_art();
}

fn use_add_one() {
    let num = 1;
    let plus_one = add_one(num);
    println!("{} + 1 = {}", num, plus_one);
}

/// Adds 1 to the number given
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = rust_book::chapters::c14_cargo_and_crates_io::add_one(arg);
///
/// assert_eq!(answer, 6);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

fn use_art() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let orange = mix(red, yellow).unwrap();
    println!("{:?} + {:?} = {:?}", red, yellow, orange);
}
