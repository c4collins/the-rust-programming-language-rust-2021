//! This chapter covers how Rust is like and how Rust is not like an Object-Oriented Language
//! Specifically, if you find yourself thinking about Rust in OOP-related ways, this chapter will help
//! you to understand why Rust is how it is, and how to do what you want without violating Rust's promises

mod averaged_collection;
mod blog;
mod gui;
mod rusty_blog;

use averaged_collection::AveragedCollection;
use blog::Post;
use rusty_blog::Post as P;

pub fn run() {
    averaged_collection();
    gui::run();
    make_blog_post();
    make_blog_post_then_reject_it();
    rusty_blog_post();
}

fn averaged_collection() {
    let mut collection = AveragedCollection::new();
    // Type a loop my using an integer suffix on the thing
    for i in 1..10u32 {
        let mul = i.pow(i); // idk why I did this
        collection.add(mul);
        println!("Added {}, average is now {}", mul, collection.average())
    }
    for _ in 1..9 {
        let i = collection.delete().unwrap();
        println!("Removed {}, average is now {}", i, collection.average())
    }
}

fn make_blog_post() {
    let mut post = Post::new();

    let post_text = "I ate a salad for lunch today\n";

    post.add_text(post_text);
    println!("post.content: {}", post.content());
    assert_ne!(post_text, post.content());

    post.request_review();
    println!("post.content: {}", post.content());
    assert_ne!(post_text, post.content());

    // Partial Approval
    post.approve();
    println!("post.content: {}", post.content());
    assert_ne!(post_text, post.content());

    post.add_text(post_text); // Since we're not in draft this should fail and print a message

    // Double Approval
    post.approve();
    println!("post.content: {}", post.content());
    assert_eq!(post_text, post.content());
}

fn make_blog_post_then_reject_it() {
    let mut post = Post::new();

    let post_text = "I ate a salad for lunch today\n";

    post.add_text(post_text);
    println!("post.content: {}", post.content());
    assert_ne!(post_text, post.content());

    post.request_review();
    println!("post.content: {}", post.content());
    assert_ne!(post_text, post.content());

    post.reject();
    println!("post.content: {}", post.content());
    assert_ne!(post_text, post.content());
}

fn rusty_blog_post() {
    let mut post = P::new();
    let test_text = "I ate a salad for lunch today";
    post.add_text(test_text);
    let post = post.request_review();
    let post: P = post.approve();
    assert_eq!(test_text, post.content());
    println!("\nRust-y post content is: {}", post.content());
}
