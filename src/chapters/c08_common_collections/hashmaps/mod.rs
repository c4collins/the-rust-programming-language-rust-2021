use std::collections::HashMap;

pub fn main() {
    let scores = creating_a_hashmap();
    accessing_a_hashmap(scores);
    let scores = creating_a_hashmap();
    updating_a_hashmap(scores);
}

fn creating_a_hashmap() -> HashMap<String, u32> {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    scores
}

fn accessing_a_hashmap(scores: HashMap<String, u32>) {
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Team {} has a score of {}", team_name, score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn updating_a_hashmap(scores: HashMap<String, u32>) {
    let overwritten_scores = overwriting_a_hashmap_value(scores);
    let updated_scores = adding_a_key_if_missing(overwritten_scores);
    println!("{:?}", updated_scores);
    updating_a_value_using_old_value()
}

fn overwriting_a_hashmap_value(mut scores: HashMap<String, u32>) -> HashMap<String, u32> {
    scores.insert(String::from("Blue"), 25);
    scores
}

fn adding_a_key_if_missing(mut scores: HashMap<String, u32>) -> HashMap<String, u32> {
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(75);
    scores
}

fn updating_a_value_using_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
