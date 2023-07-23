use std::fmt::Display;

pub fn run() {
    fixed_reference();
    which_string_is_longer();
    referenced_struct();
}

// fn broken_reference() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("r: {}", r);
// }

fn fixed_reference() {
    let x = 5;
    let r = &x;
    println!("r: {}", r);
}

fn which_string_is_longer() {
    let string1 = String::from("abcd");
    let mut result;
    {
        let string2 = String::from("xyx");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result); // Does work

        result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "Oh boy!");
        println!("The longest string is {}", result); // Does work
    }

    // println!("The longest string is {}", result); // Does not work
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn referenced_struct() {
    let i;
    {
        let novel = String::from("Call me Ishmael.  Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
        // i is in scope here because first_sentence is in scope here beause novel is still in scope here.
        let part = i.announce_and_return_part("Nice!");
        println!("{}> {}", i.level(), part);
    }
    // println!("{}", i.part); // but this one will cause an error because the references have gone out of scope.
}

// Everything Everywhere All At Once
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
