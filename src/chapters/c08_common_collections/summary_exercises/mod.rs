mod employee_departments;
mod median_and_mode;
mod pig_latin;

use rand;

pub fn main() {
    let list_of_integers: [u8; 31] = rand::random();
    median_and_mode::run(list_of_integers);
    println!();

    pig_latin::run();
    println!();

    employee_departments::run();
}
