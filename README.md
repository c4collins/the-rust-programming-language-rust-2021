# The Rust Programming Language (for Rust 2021)

I'm working through The Rust Programming Language (for Rust 2021) and this is my code.

I've gone through the book before, a long while ago, and haven't don't enough Rust since, so I'm starting over having learned a lot about other programming things in the interim.

https://doc.rust-lang.org/book/title-page.html

### Timeline: 

- 08 June 2023 - Initial Commit
- 08 June 2023 - Chapter 1 Complete
- 11 June 2023 - Chapter 2 Complete
- 03 July 2023 - Chapter 3 Complete
- 04 July 2023 - Chapter 4 Complete
- 15 July 2023 - Chapter 5 Complete
- 16 July 2023 - Chapter 6 Complete
- 16 July 2023 - Chapter 7 Complete
- 17 July 2023 - Chapter 8 Complete
- 19 July 2023 - Chapter 9 Complete
- 22 July 2023 - Chapter 10 Complete
- 23 July 2023 - Chapter 11 Complete

## Usage

- in general: 

    `cargo run --bin minigrep`

- with a script:  

    `./run.sh chapter_02-programming_a_guessing_game/guessing_game/`

    These were intended to be mostly references for the commands for various cases (especially the options in test.sh) but they're useful and usable.

    [NOTE: I'd like to rewrite this all as one Rust program (maybe after Ch.12) but we'll see how it goes.]

    - `build.sh` will do a development build & then run the project
        - `rustc.sh` is the same thing but without cargo
    - `run.sh` will only build if necessary
    - `build-release.sh` will do a release build
    - `check.sh` will do a check without a build step
    - `test.sh` will format and test

