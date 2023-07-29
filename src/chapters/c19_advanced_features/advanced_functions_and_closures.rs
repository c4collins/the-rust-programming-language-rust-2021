pub fn run() {
    pass_function_as_function();
    function_or_closure();

    let closure_fn = returns_closure();
    let x = closure_fn(50);
    println!(
        "Closure returned from function returns {} when passed 50",
        x
    );
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn pass_function_as_function() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

fn function_or_closure() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("{:?}", list_of_strings);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("{:?}", list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let mut list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    list_of_statuses.push(Status::Stop);
    println!("{:?}", list_of_statuses);
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
