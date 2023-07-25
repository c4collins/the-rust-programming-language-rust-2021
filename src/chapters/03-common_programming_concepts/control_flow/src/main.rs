fn main() {
    if_expressions();
    list_all_roots(if_let_expression(true));
    loops();
}

fn if_expressions() {
    let number = 7;
    if number < 5 {
        println!("The condition was true");
    } else {
        println!("The condition was false");
    }

    if number != 3 {
        println!("The number is not 3");
    }
}

fn list_all_roots(max: usize) {
    println!("These are the roots of {max}: ");
    let mut target = max;
    while target > 1 {
        if max % target == 0 {
            print!("{target}, ");
        }
        target -= 1;
    }
    println!("");
}

fn if_let_expression(test: bool) -> usize {
    let number = if test { 61234234 } else { 92342354 };
    number
}

fn loops() {
    loop_loops();
    while_loops();
    for_loops();
}

fn loop_loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    counter = 0;
    'counting_up: loop {
        println!("count = {counter}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        counter += 1;
    }
    println!("End count = {counter}");
}

fn while_loops() {
    loop {
        if true {
            break;
        } else {
            println!("Do some thing else");
        }
    }

    // okay, but

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loops() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is {element}.");
    }

    for element in (1..4).rev() {
        println!("{element}!");
    }
    println!("LIFTOFF!");
}
