use std::{io, process};

mod chapters;

pub use chapters::*;

#[derive(Debug)]
struct Chapter {
    number: u8,
    name: String,
    run: fn() -> (),
}

impl Chapter {
    fn new(number: u8, name: String, run_fn: fn() -> ()) -> Self {
        let ch = Self {
            number,
            name,
            run: run_fn,
        };
        ch
    }
}

fn main() {
    println!("Starting The Rust Programming Language program!");

    let chapters = [
        Chapter::new(0, String::from("Close Book"), || process::exit(1)),
        Chapter::new(1, String::from("Getting Started"), c01_getting_started::run),
        Chapter::new(
            2,
            String::from("Programming A Guessing Game"),
            c02_programming_a_guessing_game::run,
        ),
        Chapter::new(
            3,
            String::from("Common Programming Concepts"),
            c03_common_programming_concepts::run,
        ),
        Chapter::new(
            4,
            String::from("Understanding Ownership"),
            c04_understanding_ownership::run,
        ),
        Chapter::new(
            5,
            String::from("Using Structs to Structure Related Data"),
            c05_using_structs_to_structure_data::run,
        ),
        Chapter::new(
            6,
            String::from("Enums and Pattern Matching"),
            c06_enums_and_pattern_matching::run,
        ),
        Chapter::new(
            7,
            String::from("Managing Growing Projects with Packages, Crates, and Modules"),
            c07_managing_growing_projects_with_packages_crates_and_modules::run,
        ),
        Chapter::new(
            8,
            String::from("Common Collections"),
            c08_common_collections::run,
        ),
        Chapter::new(9, String::from("Error Handling"), c09_error_handling::run),
        Chapter::new(
            10,
            String::from("Generic Types, Traits, and Lifetimes"),
            c10_generic_types_traits_and_lifetimes::run,
        ),
        Chapter::new(11, String::from("Writing Automated Tests"), || {
            println!(
                "\nNOTE: This is a chapter about testing, so run `cargo test` instead of `cargo run`"
            );
            c11_writing_automated_tests::run();
        }),
        Chapter::new(
            12,
            String::from("An I/O Project: Building a Commnad Line Program"),
            || {
                println!(
                    "\nNOTE: If this fails run it like `cargo run -- to poem.txt --case_insensitive`"
                );
                c12_an_io_project::run();
            },
        ),
        Chapter::new(
            13,
            String::from("Functional Language Features: Iterators and Closures"),
            || {
                c13_iterators_and_closures::run();
                println!("\nNOTE: There are more things in this chapter but you need to run `cargo test` to see them!");
                println!(
                    "\nNOTE: If this failed run it like `cargo run -- to poem.txt --case_insensitive`"
                );
            },
        ),
        Chapter::new(
            14,
            String::from("More about Cargo and Crates.io"),
            c14_cargo_and_crates_io::run,
        ),
        Chapter::new(15, String::from("Smart Pointers"), c15_smart_pointers::run),
        Chapter::new(
            16,
            String::from("Fearless Concurrency"),
            c16_fearless_concurrency::run,
        ),
        Chapter::new(
            17,
            String::from("Object-Oriented Programming Features of Rust"),
            c17_object_orientation_in_rust::run,
        ),
        Chapter::new(
            18,
            String::from("Patterns and Matching"),
            c18_patterns_and_matching::run,
        ),
        Chapter::new(
            19,
            String::from("Advanced Features"),
            c19_advanced_features::run,
        ),
        Chapter::new(
            20,
            String::from("Final Project: Building a Multithreaded Web Server"),
            c20_multithreaded_web_server::run,
        ),
    ];

    // println!("{:?}", chapters); // Debugging -- TODO: learn about logging in Rust

    list_available_chapters(&chapters);

    loop {
        println!("\nWhich chapter do you want to run?");

        let chapter_num_input = get_user_input();

        // Hopefully the book has fewer than 256 chapters
        run_chapter(
            &chapters,
            chapter_num_input.trim().parse::<u8>().unwrap_or(0),
        );
    }
}

fn list_available_chapters(chapters: &[Chapter]) {
    for chapter in chapters {
        println!("\t{}. {}", chapter.number, chapter.name);
    }
}

fn get_user_input() -> String {
    let mut chapter_num_input = String::new();
    io::stdin()
        .read_line(&mut chapter_num_input)
        .expect("Failed to read line");
    chapter_num_input
}

fn run_chapter(chapters: &[Chapter], num: u8) {
    let chapter = chapters
        .iter()
        .find(|ch| ch.number == num)
        .unwrap_or_else(|| {
            println!("{} is not a valid choice", num);
            &chapters[0]
        });

    println!("Running Chapter {}: {}", chapter.number, chapter.name);

    (chapter.run)();
}
