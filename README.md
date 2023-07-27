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
    2 --> P1((Guessing Game))

    2 == 03 July 2023 ==> 3[Chapter 3: Common Programming Concepts]
    3 -.-> 2

    3 == 04 July 2023 ==> 4[Chapter 4: Understanding Ownership]
    4 -.-> 2

    4 == 15 July 2023 ==> 5[Chapter 5: Using Structs to Structure Related Data]
    5 == 16 July 2023 ==> 6[Chapter 6: Enums and Pattern Matching]
    6 == 16 July 2023 ==> 7[Chapter 7: Managing Growing Projec ts with Packages, Crates, and Modules]
    7 == 17 July 2023 ==> 8[Chapter 8: Common Collections]
    8 == 19 July 2023 ==> 9[Chapter 9: Error Handling]

    9 == 22 July 2023 ==> 10[Chapter 10: Generic Types, Traits, and Lifetimes]
    9 -.-> 2

    10 == 23 July 2023 ==> 11[Chapter 11: Writing Automated Tests]

    11 == 24 July 2023 ==> 12[Chapter 12: An I/O Project: Building a Command Line Program]
    2 -.-> 12
    5 -.-> 12
    6 -.-> 12
    7 -.-> 12
    8 -.-> 12
    9 -.-> 12
    10 -.-> 12
    11 -.-> 12
    12 --> P2((Minigrep))


    00> Project format change ] == 25 July 2023 ==> 000> Rewrite seperate programs as modules ]
    000 --> 1
    000 --> 2
    000 --> 3
    000 --> 4
    000 --> 5
    000 --> 6
    000 --> 7
    000 --> 8
    000 --> 9
    000 --> 10
    000 --> 11
    000 --> 12
    000 --> 13

    12 == 25 July 2023 ==> 13[Chapter 13: Functional Language Features: Iterators and Closures]
    13 -.-> 12
    13 --> P3((Minigrep Plus))

    13 == 26 July 2023 ==> 14[Chapter 14: More about Cargo and Crates.io]

    14 == 27 July 2023 ==> 15[Chapter 15: Smart Pointers]
```

## Scripts

I wrote the scripts to help me have references to the different cargo features, and I used them for a while, but then I got better at cargo/rust and didn't need them any more so they're just sort of for reference and don't have anything to do with the project any more.  This README file used to have instructions about them, so you could look that up in the commit history if you wanted to