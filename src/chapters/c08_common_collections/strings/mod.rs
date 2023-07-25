pub fn main() {
    creating_a_string();
    updating_a_string();
    slicing_a_string();
    iterating_a_string();
}

fn creating_a_string() {
    let s = String::new();
    println!("{}", s);

    let data = "initial contents 2";
    let s = data.to_string();
    println!("{}", s);

    let s = "initial contents 3".to_string();
    println!("{}", s);

    let s = String::from("initial contents 4");
    println!("{}", s);

    let hello = String::from("السلام عليكم");
    println!("{}", hello);

    let hello = String::from("Dobrý den");
    println!("{}", hello);

    let hello = String::from("Hello");
    println!("{}", hello);

    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);

    let hello = String::from("नमस्ते");
    println!("{}", hello);

    let hello = String::from("こんにちは");
    println!("{}", hello);

    let hello = String::from("안녕하세요");
    println!("{}", hello);

    let hello = String::from("你好");
    println!("{}", hello);

    let hello = String::from("Olá");
    println!("{}", hello);

    let hello = String::from("Здравствуйте");
    println!("{}", hello);

    let hello = String::from("Hola");
    println!("{}", hello);
}

fn updating_a_string() {
    let s1 = "foo";
    let s2 = "bar";
    let mut s = String::from(s1);
    s.push_str(s2);
    println!("{} + {} => {}", s1, s2, s);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // format! is basically the same as println! but without printing
    println!("{}", s);
}

fn slicing_a_string() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}"); // This is inly two letters long, despite getting the first 4 bytes, because each letter is two bytes
                     // IMO this is unhelpful and I'd rather have this slice by char or grapheme but they know better than I do
}

fn iterating_a_string() {
    for c in "Зд".chars() {
        print!("{c}");
    }
    println!("");

    for b in "Зд".bytes() {
        print!("{b} ");
    }
    println!();
}
