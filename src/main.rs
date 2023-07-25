use std::io;

mod chapters;

use chapters::{
    c01_getting_started, c02_programming_a_guessing_game, c03_common_programming_concepts,
    c04_understanding_ownership, c05_using_structs_to_structure_data,
    c06_enums_and_pattern_matching, c07_managing_growing_projects_with_packages_crates_and_modules,
    c08_common_collections, c09_error_handling, c10_generic_types_traits_and_lifetimes,
    c11_writing_automated_tests, c12_an_io_project, c13_iterators_and_closures,
};

fn main() {
    println!("hi");

    println!("Which chapter do you want to run?");
    let mut chapter_num = String::new();
    io::stdin()
        .read_line(&mut chapter_num)
        .expect("Failed to read line");

    match chapter_num.trim() {
        "1" | "01" => c01_getting_started::run(),
        _ => println!("Invalid Option"),
    }
}
