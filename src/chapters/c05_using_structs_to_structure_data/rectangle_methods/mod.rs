#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Instantiations
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    // Verification getters
    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }
}

impl Rectangle {
    // Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// NOTE: there's no reason to use multiple impl blocks (here, at least); but there's nothing stopping it

pub fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let unit = "pixels";

    if rect.width() && rect.height() {
        println!(
            "A rectangle of {0} {unit} x {1} {unit} has an area of {2} {unit}²",
            rect.width,
            rect.height,
            rect.area()
        );
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle::square(29);
    let rect3 = Rectangle::new(60, 45);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
