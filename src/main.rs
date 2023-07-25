use std::{io, process};

mod chapters;

pub use chapters::{
    c01_getting_started,
    c02_programming_a_guessing_game,
    c03_common_programming_concepts,
    c04_understanding_ownership,
    c05_using_structs_to_structure_data,
    c06_enums_and_pattern_matching,
    c07_managing_growing_projects_with_packages_crates_and_modules,
    c08_common_collections,
    c09_error_handling,
    c10_generic_types_traits_and_lifetimes,
    //c11_writing_automated_tests,
    c12_an_io_project,
    c13_iterators_and_closures,
};

fn main() {
    println!("Starting The Rust Programming Language program!");
    loop {
        println!("Which chapter do you want to run?");

        let mut chapter_num_input = String::new();
        io::stdin()
            .read_line(&mut chapter_num_input)
            .expect("Failed to read line");

        // Hopefully the book has fewer than 256 chapters
        let chapter_num: u8 = chapter_num_input.trim().parse::<u8>().unwrap_or(0);

        if chapter_num < 22 {
            match chapter_num {
                0 => process::exit(1),
                1 => c01_getting_started::run(),
                2 => c02_programming_a_guessing_game::run(),
                3 => c03_common_programming_concepts::run(),
                4 => c04_understanding_ownership::run(),
                5 => c05_using_structs_to_structure_data::run(),
                6 => c06_enums_and_pattern_matching::run(),
                7 => c07_managing_growing_projects_with_packages_crates_and_modules::run(),
                8 => c08_common_collections::run(),
                9 => c09_error_handling::run(),
                10 => c10_generic_types_traits_and_lifetimes::run(),
                11 => println!(
                    "This is a chapter about testing, so run `cargo test` instead of `cargo run`"
                ),
                12 => c12_an_io_project::run(),
                13 => c13_iterators_and_closures::run(),
                _ => println!(
                    "Invalid Option, Chapter {} is not in this program (yet?)",
                    chapter_num_input
                ),
            }
        }
    }
}
