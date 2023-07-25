use garden::vegetables::Asparagus;

pub mod garden;

pub fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
