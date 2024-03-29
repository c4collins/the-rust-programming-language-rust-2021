const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Constants must have type annotation

pub fn main() {
    println!(
        "Three hours in seconds is {} seconds",
        THREE_HOURS_IN_SECONDS
    );

    // Mutability
    let mut x = 5;
    println!("The value of x is {x}!");
    x = 6;
    println!("The value of x is {x}!");

    // Shadowing
    let y = x + 1;
    {
        let y = y * 2;
        println!("The value of the inner-scope y is {y}!");
    }
    println!("The value of y is {y}!");

    // Shadowing + changing types
    let spaces = "    ";
    let spaces = spaces.len();
    println!("There are {spaces} spaces!");
}
