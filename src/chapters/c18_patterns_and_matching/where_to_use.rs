pub fn run() {
    matching();
    if_let();
    while_let();
    for_loops();
    let_statements();
    function_parameters();
}

// Matches are nice because they're exhaustive
fn matching() {
    let x = Some(1);
    let y = match x {
        None => None,
        Some(i) => Some(i + 1),
        // _ => None, // It's not needed here but this is a catch-all pattern
    };
    println!("x: {}, y: {}", x.unwrap(), y.unwrap());
}

// mixing if let, else if let, and normal if/else you can get quite a set of logic
// but the compiler doesn't verify for completeness in your logic
fn if_let() {
    let favourite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favourite_color {
        println!("Using your favourit color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loops() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn let_statements() {
    let x = 5;
    let (a, b, c) = (1, 2, 3);
    // let (h, i) = (4, 5, 6, 7, 8, 9, 10);
    println!("{}, {}, {}, {}", x, a, b, c);
}

fn function_parameters() {
    fn foo(x: i32) {
        println!("x is {} in foo()", x);
    }
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current coordiantes: ({}, {})", x, y);
    }

    foo(32);
    let point = (3, 5);
    print_coordinates(&point);
}
