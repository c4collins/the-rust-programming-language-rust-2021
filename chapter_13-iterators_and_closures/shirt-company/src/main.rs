use std::thread;
use std::time::Duration;

fn main() {
    store_giveaways();
    expensive_closures();
    example_closures();
    closure_with_immutable_borrow();
    closure_with_mutable_borrow();
    passing_ownership_to_thread_with_move_and_closure();
    the_fn_traits();
}

fn store_giveaways() {
    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preferences: Option<ShirtColor>) -> ShirtColor {
            user_preferences.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }

            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Red],
    };

    let user_pref_1 = Some(ShirtColor::Blue);
    let giveaway_1 = store.giveaway(user_pref_1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref_1, giveaway_1
    );

    let user_pref_2 = None;
    let giveaway_2 = store.giveaway(user_pref_2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref_2, giveaway_2
    );
}
fn expensive_closures() {
    let expensive_closure = || -> u32 {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(2));
        0
    };

    let user_pref_3 = None;
    println!("{}", user_pref_3.unwrap_or_else(expensive_closure));
}

fn example_closures() {
    // Basically equivalent (other than the general differences between functions and closures)
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    println!("{}", s);
    // let n = example_closure(5); // This won't work because the previous line sets the inferred type for the closure
}

fn closure_with_immutable_borrow() {
    let list = vec![1, 2, 3];
    println!("Before defining closure {:?}", list);
    let only_borrow = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrow();
    println!("After calling closure: {:?}", list);
}

fn closure_with_mutable_borrow() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure {:?}", list);
    let mut borrow_mutably = || list.push(6);
    // println!("Before calling closure: {:?}", list); // NOTE: This would fail because the closure owns the reference here
    borrow_mutably();
    println!("After calling closure: {:?}", list); // NOTE: But it's released and usable again here
}

fn passing_ownership_to_thread_with_move_and_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    // println!("After calling closure: {:?}", list); // NOTE: This would fail because the `move` keyword means ownership is lost
}

fn the_fn_traits() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn the_fnmut_trait() {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        list.sort_by_key(|r| r.width);
        println!("{:#?}", list);
    }

    fn the_fnonce_trait() {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        let mut sort_operations: Vec<String> = vec![];
        let value = String::from("by key called");

        list.sort_by_key(|r| {
            // sort_operations.push(value); // NOTE: this is a very contrived example, but this line forces FnMut so this can't work
            r.width
        });
        // println!("{:#?}", list);
        avoiding_fnonce()
    }

    fn avoiding_fnonce() {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];
        let mut num_sort_operations = 0;
        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        });
        println!("{:#?}, sorted in {} operations", list, num_sort_operations);
    }

    the_fnmut_trait();
    the_fnonce_trait();
}
