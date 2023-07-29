mod advanced_functions_and_closures;
mod advanced_traits;
mod advanced_types;
mod macros;
mod unsafe_rust;

pub fn run() {
    unsafe_rust::run();
    advanced_traits::run();
    advanced_types::run();
    advanced_functions_and_closures::run();
    macros::run();
}
