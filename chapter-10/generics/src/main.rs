mod fn_and_impl;
mod lifetimes;
mod summaries;
mod traits;

fn main() {
    fn_and_impl::go();
    traits::go();
    lifetimes::go();
}

// NOTE: I don't really have a good way or reason to implement this, but it's important

// # Using Trait Bounds to Conditionally Implement Methods

// By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.
// For example, the type Pair<T> in Listing 10-15 always implements the new function to return a new instance of Pair<T> (recall from the “Defining Methods” section of Chapter 5 that Self is a type alias for the type of the impl block, which in this case is Pair<T>).
// But in the next impl block, Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.

// ```Filename: src/lib.rs

// use std::fmt::Display;

// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }```

// - Listing 10-15: Conditionally implementing methods on a generic type depending on trait bounds

// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library.

// ...

// Blanket implementations appear in the documentation for the trait in the “Implementors” section.

// Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior.
// The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior.
// In dynamically typed languages, we would get an error at runtime if we called a method on a type which didn’t define the method.
// But Rust moves these errors to compile time so we’re forced to fix the problems before our code is even able to run.
// Additionally, we don’t have to write code that checks for behavior at runtime because we’ve already checked at compile time.
// Doing so improves performance without having to give up the flexibility of generics.
