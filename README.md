# The Rust Programming Language (for Rust 2021)

I'm working through The Rust Programming Language (for Rust 2021) and this is my code.

I've gone through the book before, a long while ago, and haven't don't enough Rust since, so I'm starting over having learned a lot about other programming things in the interim.

https://doc.rust-lang.org/book/title-page.html

## Usage

1. `cargo run`
2. It will ask you which chapter you want to see
3. Enter a number 1-21 to get a response.
4. **to end:** CTRL+C, or enter any string that won't parse to an 8-bit number (e.g. butts, 256, -1), or 0.


### Timeline Flowchart: 

```mermaid 
flowchart TD
    0> Initial Commit ] == 08 June 2023 ==> 1[Chapter 1: Getting Started]
    1 == 11 June 2023 ==> 2[Chapter 2: Programming a Guessing Game]
    2 == 03 July 2023 ==> 3[Chapter 3: Common Programming Concepts]
    3 == 04 July 2023 ==> 4[Chapter 4: Understanding Ownership]
    4 == 15 July 2023 ==> 5[Chapter 5: Using Structs to Structure Related Data]
    5 == 16 July 2023 ==> 6[Chapter 6: Enums and Pattern Matching]
    6 == 16 July 2023 ==> 7[Chapter 7: Managing Growing Projec ts with Packages, Crates, and Modules]
    7 == 17 July 2023 ==> 8[Chapter 8: Common Collections]
    8 == 19 July 2023 ==> 9[Chapter 9: Error Handling]
    9 == 22 July 2023 ==> 10[Chapter 10: Generic Types, Traits, and Lifetimes]
    10 == 23 July 2023 ==> 11[Chapter 11: Writing Automated Tests]
    11 == 24 July 2023 ==> 12[Chapter 12: An I/O Project: Building a Command Line Program]
    12 == 25 July 2023 ==> 13[Chapter 13: Functional Language Features: Iterators and Closures]
    13 == 26 July 2023 ==> 14[Chapter 14: More about Cargo and Crates.io]
    14 == 27 July 2023 ==> 15[Chapter 15: Smart Pointers]
    15 == 27 July 2023 ==> 16[Chapter 16: Fearless Concurrency]
    16 == 28 July 2023 ==> 17[Chapter 17: Object-Oriented Programming Features of Rust]
    17 == 29 July 2023 ==> 18[Chapter 18: Patterns and Matching]
    18 == 29 July 2023 ==> 19[Chapter 19: Advanced Features]
    19 == TBD ==> 20[Chapter 20: Final Project: Building a Multi-threaded Web Server]
    20 == TBD ==> 21[Chapter 21:Appendix]

    21 -- TBD --> A[A. Keywords]
    21 -- TBD --> B[B. Operators and Symbols]
    21 -- TBD --> C[C. Derivable Traits]
    21 -- TBD --> D[D. Useful Development Tools]
    21 -- TBD --> E[E. Editions]
    21 -- TBD --> F[F. Translations of the Book]
    21 -- TBD --> G[G. How Rust Is Made and 'Nightly Rust']

    2 -- 11 June 2023 --> P1((Guessing Game))
    12 -- 24 July 2023 --> P2((Minigrep))
    13 -- 25 July 2023 --> P3((Minigrep Plus))
    P2 -.-> P3
    20 -- TBD --> P4((Web Server))
```

## Scripts

I wrote the scripts to help me have references to the different cargo features, and I used them for a while, but then I got better at cargo/rust and didn't need them any more so they're just sort of for reference and don't have anything to do with the project any more.  This README file used to have instructions about them, so you could look that up in the commit history if you wanted to