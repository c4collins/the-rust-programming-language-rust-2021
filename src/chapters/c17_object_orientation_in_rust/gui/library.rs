pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        println!("\nRunning Screen:");
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("\nThis is where I would draw my button, if I had one");
        println!(
            "\tIt would be {}x{} and say `{}`",
            self.width, self.height, self.label
        );
    }
}
