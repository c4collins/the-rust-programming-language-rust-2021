// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added, (NOTE: I did first syllable)
// so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

use std::collections::HashMap;

const LETTERS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn run() {
    let text = "By signing up you agree to our terms of service and acknowledge that you have read and understand our privacy policy and code of conduct";
    let igpay_atinlay = one_line(String::from(text));
    println!("{text}");
    println!("igpay_atinlay:");
    println!("{igpay_atinlay}");
}

fn one_line(line: String) -> String {
    let words = line.split_whitespace();
    let mut resp: String = String::from("");
    for word in words {
        resp = format!("{} {}", resp, one_word(String::from(word)))
    }
    let resp = resp.trim();
    String::from(resp)
}

fn one_word(word: String) -> String {
    let (letter, index) = get_vowel_index(&word);
    // println!("{} splits at {}/{}", word, letter, index);

    if index < word.len().try_into().unwrap() {
        let split_word: Vec<&str> = word.split(letter).collect();
        // println!("{:?}", split_word);
        let first_syllable = split_word[0];
        let the_rest = split_word[1..].join(&format!("{letter}"));
        // println!("{letter} {the_rest} {first_syllable}");
        if index < u8::MAX {
            return format!("{}{}{}ay", letter, the_rest, first_syllable);
        }
    }

    format!("{}hay", word) // fallback if no vowels are found
}

fn get_vowel_index(word: &String) -> (char, u8) {
    let mut vowel_hash = HashMap::new();

    for (index, char) in word.char_indices() {
        if LETTERS.contains(&char) {
            vowel_hash.entry(char).or_insert(index);
        }
    }

    // println!("{:?}", vowel_hash);

    let mut min_index = 254;
    let mut min_letter = 'b';

    for letter in LETTERS {
        let index = vowel_hash.get(&letter).copied().unwrap_or(usize::MAX);
        // println!("{index} {min_index} {}", index < min_index);
        if index > 0 && index < min_index {
            min_index = index;
            min_letter = letter;
        }
    }

    (min_letter, min_index.try_into().unwrap())
}
