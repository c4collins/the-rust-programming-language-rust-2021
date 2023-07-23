mod employee_departments;
mod median_and_mode;
mod pig_latin;

use rand;

fn main() {
    let list_of_integers: [u8; 31] = rand::random();
    median_and_mode::go(list_of_integers);
    println!();

    pig_latin::go();
    println!();

    employee_departments::go();
}
