pub fn main() {
    println!("Hello, world!");
    another_function(5, 'h');
    let five_val = five();
    println!("five() is {five_val}");
    let six_val = plus_one(five());
    println!("plus_one(five()) is {six_val}");
}

// Must declare types for function parameters
fn another_function(x: i32, label: char) {
    println!("Another function");
    println!("The value of x is {x}{label}!");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
