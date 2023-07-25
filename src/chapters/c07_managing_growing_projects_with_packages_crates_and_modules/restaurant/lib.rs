pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house;

pub use crate::front_of_house::hosting;

mod customer {

    use super::back_of_house;
    use super::front_of_house;
    use super::hosting;

    pub fn eat_at_restaurant() {
        // use path
        hosting::add_to_waitlist();
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();
        // Relative path
        front_of_house::hosting::add_to_waitlist();

        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like some {} toast please!", meal.toast);

        // Th next statement won't compile if it's uncommented
        // Since we're not allowed to see or change the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

use std::collections::HashMap;

// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, collections};

// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// Glob
use std::collections::*;
