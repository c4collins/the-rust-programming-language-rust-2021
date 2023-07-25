mod guessing_game_2;
mod panics;

pub fn run() {
    guessing_game_2::main();
    let _ = panics::main();
}
