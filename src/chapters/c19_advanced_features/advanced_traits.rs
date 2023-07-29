use std::fmt;
use std::ops::Add;

pub fn run() {
    add_points();
    add_meters_to_millimetres();
    trait_method_disambiguation();
    using_supertypes();
    newtype_thin_wrapper_trait_implementation();
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn add_points() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 3, y: 4 };
    let c = a + b;
    assert_eq!(c, Point { x: 4, y: 6 });
    println!("{:?}, + {:?} = {:?}", a, b, c);
}

#[derive(Debug)]
struct Metres(u32);

#[derive(Debug)]
struct Millimetres(u32);

impl Add<Metres> for Millimetres {
    type Output = Millimetres;

    fn add(self, other: Metres) -> Millimetres {
        Millimetres(self.0 + (other.0 * 1000))
    }
}

fn add_meters_to_millimetres() {
    let m = Metres(7);
    let mm = Millimetres(444);
    println!("{:?} + {:?} =", m, mm,); // These get consumed when we add them

    // let length = m + mm; // Doesn't work, we didn't define that
    let length = mm + m;
    println!("{:?}", length);
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking!");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println! {"Up!"};
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn trait_method_disambiguation() {
    let person = Human;
    person.fly();

    Pilot::fly(&person);
    Wizard::fly(&person);

    // <Type as Trait>::function(first_arg, next_arg, ...);
    println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby dog is called a {}", Animal::baby_name()); // Compiler can't know what to do here
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn using_supertypes() {
    let p = Point { x: 7, y: 4 };
    println!("{p}");
    p.outline_print();
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

fn newtype_thin_wrapper_trait_implementation() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
