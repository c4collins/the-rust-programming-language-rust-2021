fn main() {
    let unit: String = String::from("pixels");

    // Without any Structure
    let width = 30;
    let height = 50;
    plain(width, height, &unit);

    // As a Tuple
    let rect = (width, height);
    as_tuple(rect, &unit);

    // As a Struct
    let rect = Rectangle {
        width: dbg!(width),
        height: dbg!(height),
    };
    as_rect(&rect, &unit);

    println!("The rectangle is {:#?}", rect);
    dbg!(&rect);
}

fn get_area(width: u32, height: u32) -> u32 {
    // This chapter makes a big deal about changing the signature for this function - but there's nothing wrong with it
    // I understand that they're trying to make a point about code organization, but having this signature is more flexible (as I show here)
    // This function isn't hard to understand in any way as-is, and can be used by anyone wanting to find an area, regardless of their data structure
    // I do understand the simplistic nature of this example, but I also appreciate that it's a contrived example with no explicit merit
    width * height
}

// Without any Structure
fn plain(width: u32, height: u32, unit: &str) {
    let area = get_area(width, height);
    println!("A rectangle of {width} {unit} x {height} {unit} has an area of {area} {unit}²");
}

// As a Tuple
fn as_tuple(rect: (u32, u32), unit: &str) {
    let area = get_area(rect.0, rect.1);
    println!(
        "A rectangle of {0} {unit} x {1} {unit} has an area of {area} {unit}²",
        rect.0, rect.1
    );
}

// As a Struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn as_rect(rect: &Rectangle, unit: &str) {
    let area = get_area(rect.width, rect.height);
    println!(
        "A rectangle of {0} {unit} x {1} {unit} has an area of {area} {unit}²",
        rect.width, rect.height
    );
}
