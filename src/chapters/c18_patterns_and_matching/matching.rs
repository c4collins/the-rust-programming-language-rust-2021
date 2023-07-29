pub fn run() {
    matching_literals();
    matching_named_variables();
    matching_multiple_patterns();
    matching_range_values();
    destructuring_structs();
    destructuring_enums();
    destructuring_structs_and_tuples();
    ignoring_entire_value();
    ignoring_partial_value();
    ignoring_unused_variable();
    ignoring_remaining_values();
    match_guards();
    at_bindings();
}

fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("matched, y = {}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {}", x, y);
}

fn matching_multiple_patterns() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything else"),
    }
}

fn matching_range_values() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("Everything else"),
    }

    let y = 'y';

    match y {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("Something else entirely"),
    }
}

fn destructuring_structs() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // Set the variables to whatever you want
    assert_eq!(0, a);
    assert_eq!(7, b);
    println!("{} = {}, {} = {}", a, p.x, b, p.y);

    let Point { x, y } = p; // But there is a shorthand version if you aren't changing the var name
    assert_eq!(0, x);
    assert_eq!(7, y);
    println!("{} = {}, {} = {}", x, p.x, y, p.y);

    match p {
        // Or skip making variables and match the struct directly
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis (x, y)"),
    }
}

fn destructuring_enums() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let messages = [
        Message::ChangeColor(Color::Hsv(0, 160, 240)),
        Message::Write(String::from("test")),
        Message::Move { x: 0, y: 44 },
        Message::ChangeColor(Color::Rgb(56, 68, 215)),
        Message::Quit,
    ];

    for msg in messages {
        match msg {
            Message::Quit => println!("The Quit variant has no data to destructure"),
            Message::Move { x, y } => println!("Move in the directions; x: {x}, y: {y}"),
            Message::Write(text) => println!("Text message: {text}"),
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to: RGB red:{r}, green:{g}, blue:{b}")
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to: HSV hue:{h}, saturation:{s}, value:{v}")
            }
        }
    }
}

// You can do some really messed up destructuring
fn destructuring_structs_and_tuples() {
    struct Point {
        x: i32,
        y: i32,
    }
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!("feet:{feet}, inches:{inches}, x:{x}, y:{y}");
}

fn ignoring_entire_value() {
    fn foo(_: i32, y: i32) {
        // If you were implementing this function it wouldn't make a lot of sense to do this
        // But if you're implementing a function using someoen else's signature and don't need every value they defined
        // This will keep your brain clean and your compiler happy
        println!("I only use y here {y} so I don't need the other value");
    }

    foo(1, 6);
    foo(9, 0);
}

fn ignoring_partial_value() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite and existing customized value"),
        _ => setting_value = new_setting_value,
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {first}, {third}, {fifth}"),
    }
}

fn ignoring_unused_variable() {
    let x = 10; // No warning because it's used
    let _y = 20; // No warning even though it's unused
    let z = 30; // Will get a warning because it's unused
    println!("x = {x}");

    let s = Some(String::from("Hello!"));
    // if let Some(_s) = s { // Subtle different but this version MOVEs the s variable
    if let Some(_) = s {
        // This version doesn't move the s variable, so it can be used later
        println! {"found a string"};
    }

    println!("{:?}", s);
}

fn ignoring_remaining_values() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point { x: 1, y: 2, z: 3 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32, 64);
    match numbers {
        (first, .., last) => println!("numbers range from {}-{}", first, last),
    }
}

fn match_guards() {
    // When using match guards exhaustiveness is not guaranteed
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    let x = 4;
    let y = false;

    match x {
        // [(4|5|6) if y], not [4|5|(6 if y)]
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn at_bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
