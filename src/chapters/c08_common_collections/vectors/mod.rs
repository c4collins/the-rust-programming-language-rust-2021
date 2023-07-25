pub fn main() {
    let _empty_vector: Vec<i32> = new_empty_vector();
    let _macro_vector = vector_from_macro();

    let mut normal_vector = new_empty_vector();
    normal_vector.push(5);
    normal_vector.push(6);
    normal_vector.push(7);
    normal_vector.push(8);
    println!("{:?}", normal_vector);

    // Get with index
    let third = &normal_vector[2];
    println!("The third element is {third}");

    // Get with .get()
    let third = normal_vector.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is not third element"),
    }

    for i in &mut normal_vector {
        println!("{i}");
        *i += 50;
    }

    for i in &normal_vector {
        println!("{i}");
    }

    enum_to_store_multiple_types();
}

fn new_empty_vector<T>() -> Vec<T> {
    Vec::new()
}

fn vector_from_macro() -> Vec<i32> {
    vec![1, 2, 3]
}

fn enum_to_store_multiple_types() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("This row contains: {:?}", row);
}
