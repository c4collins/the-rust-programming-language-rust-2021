use std::cmp::PartialOrd;

pub fn go() {
    find_the_largest_number_in_a_list();

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The highest char is {}", result);

    // Generics
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The highest char is {}", result);

    let integer_point = Point { x: 1, y: 10 };
    let float_point = Point { x: 1.0, y: 5.1 };
    println!(
        "integer point: {:?}, float point: {:?}",
        integer_point, float_point
    );
    // let point = Point { x: 4, y: 5.0 }; // Can't have different types there
    let multipoint = MultitypePoint { x: 4, y: 5.0 };

    println!(
        "integer point: {:?}, float point: {:?}, multipoint: {:?}",
        integer_point, float_point, multipoint
    );
    println!("ip.x = {}", integer_point.x());
    println!(
        "fp.distance_from_origin = {}",
        float_point.distance_from_origin()
    );
    let other_multitype_point = MultitypePoint { x: "Hello", y: 'c' };
    let p3 = multipoint.mixup(other_multitype_point);
    println! {"p3 = {:?}", p3};
}

fn find_the_largest_number_in_a_list() {
    // Non-generic, non-combined
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
}

fn largest_i32(list: &[i32]) -> &i32 {
    // Non-generic, combined
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    // Non-generic, combined
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct MultitypePoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> MultitypePoint<T, U> {
    fn mixup<X, Y>(self, other: MultitypePoint<X, Y>) -> MultitypePoint<T, Y> {
        MultitypePoint {
            x: self.x,
            y: other.y,
        }
    }
}
