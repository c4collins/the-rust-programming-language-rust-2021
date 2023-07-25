pub fn main() {
    hello_world();
    functions_and_ownership();
    borrowing();
    mutable_references();
}

fn hello_world() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str appends a literal to a String
    println!("{s}");
}

fn functions_and_ownership() {
    let s = String::from("test string");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    println!("{x} is still available");
    // println!("but {s} is not");

    let owned_from_fn = gives_ownership();
    let owned_from_creation = String::from("hello");
    let owned_from_function_parameter = takes_and_gives_back(owned_from_creation);

    println!("{owned_from_fn} is good");
    // println!("{owned_from_creation} is gone");
    println!("{owned_from_function_parameter} is also good");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn borrowing() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {s1} is {len}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutable_references() {
    let mut s = String::from("Hello");
    change(&mut s);
    let length = calculate_length(&s);
    println!("The line is now {s} and has a length of {length}");
}

fn change(s: &mut String) {
    s.push_str(", world");
}
