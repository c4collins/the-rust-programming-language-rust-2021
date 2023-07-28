// This module is a mock of a GUI library
mod library;

use library::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("\nThis is where I would but my SelectBox if I had one!");
        println!(
            "\tIt would be {}x{} and have {} Options",
            self.width,
            self.height,
            self.options.len()
        );
        for opt in self.options.iter() {
            println!("{}", opt);
        }
    }
}

pub fn run() {
    println!("\ngui lib starting");

    let screen = Screen {
        components: vec![
            Box::new(Button {
                height: 25,
                width: 100,
                label: String::from("Start"),
            }),
            Box::new(SelectBox {
                height: 50,
                width: 175,
                options: vec![
                    String::from("Foo"),
                    String::from("bar"),
                    String::from("baz"),
                ],
            }),
        ],
    };
    screen.run();
}
