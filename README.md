# The Rust Programming Language (for Rust 2021)

I'm working through The Rust Programming Language (for Rust 2021) and this is my code.

I've gone through the book before, a long while ago, and haven't don't enough Rust since, so I'm starting over having learned a lot about other programming things in the interim.

https://doc.rust-lang.org/book/title-page.html

## Usage

- in general: 

    `cargo run --bin minigrep`

- with a script:  

    `./run.sh chapter_02-programming_a_guessing_game/guessing_game/`

    These were intended to be mostly references for the commands for various cases (especially the options in test.sh) but they're useful and usable.

    [NOTE: I'd like to rewrite this all as one Rust program but we'll see how it goes.]

    - `build.sh` will do a development build & then run the project
        - `rustc.sh` is the same thing but without cargo
    - `run.sh` will only build if necessary
    - `build-release.sh` will do a release build
    - `check.sh` will do a check without a build step
    - `test.sh` will format and test


### Timeline: 

```mermaid 
flowchart TD
    0[Initial Commit] -- 08 June 2023 --> 1[Chapter 1: Getting Started]
    1 -- 11 June 2023 --> 2[Chapter 2: Programming a Guessing Game]
    2 -- 03 July 2023 --> 3[Chapter 3: Common Programming Concepts]
    3 -- 04 July 2023 --> 4[Chapter 4: Understanding Ownership]
    4 -- 15 July 2023 --> 5[Chapter 5: Using Structs to Structure Related Data]
    5 -- 16 July 2023--> 6[Chapter 6: Enums and Pattern Matching]
    6 -- 16 July 2023 --> 7[Chapter 7: Managing Growing Projec ts with Packages, Crates, and Modules]
    7 -- 17 July 2023 --> 8[Chapter 8: Common Collections]
    8 -- 19 July 2023 --> 9[Chapter 9: Error Handling]
    9 -- 22 July 2023 --> 10[Chapter 10: Generic Types, Traits, and Lifetimes]
    10 -- 23 July 2023 --> 11[Chapter 11: Writing Automated Tests]
    11 -- 24 July 2023 --> 12[Chapter 12: An I/O Project: Building a Command Line Program]
```