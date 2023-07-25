pub fn main() {
    let hw = "Hello, world !";
    let first_word = get_first_word(&hw);
    println!("{first_word}");
    let second_word = get_second_word(&hw);
    println!("{second_word}");
}

fn get_first_word(input: &str) -> &str {
    let bytes = input.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }
    &input[..]
}

fn get_second_word(input: &str) -> &str {
    let bytes = input.as_bytes();
    let first_word = get_first_word(&input);
    let first_word_len = first_word.len() + 1;
    for (i, &item) in bytes.iter().enumerate() {
        if i > first_word_len && item == b' ' {
            return &input[first_word_len..i];
        }
    }
    &input[first_word_len..]
}
