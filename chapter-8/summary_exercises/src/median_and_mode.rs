// Given a list of integers,
// use a vector and
//    return the median (when sorted, the value in the middle position) and
//     mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

pub fn go(ints: [u8; 31]) {
    let ints = sort_array(ints);
    println!("{:?}", ints);

    min_and_max(&ints);

    let median = median_from_array(&ints);
    println!("The median number is {median}");

    mode_from_array(&ints);
}

fn sort_array(mut ints: [u8; 31]) -> [u8; 31] {
    ints.sort();
    ints
}

fn min_and_max(ints: &[u8; 31]) {
    let min = match ints.iter().min() {
        Some(i) => i,
        None => &u8::MIN,
    };

    let max = match ints.iter().max() {
        Some(i) => i,
        None => &u8::MAX,
    };

    println!("This range goes from {:?} to {:?}", min, max);
}

fn median_from_array(ints: &[u8; 31]) -> u8 {
    ints[ints.len() / 2]
}

fn get_counts(ints: &[u8; 31]) -> HashMap<u8, u8> {
    let mut hash = HashMap::new();

    for &int in ints {
        // println!("{int}");
        let count = hash.entry(int).or_insert(0);
        *count += 1;
    }

    hash
}

fn mode_from_array(ints: &[u8; 31]) {
    let int_counts = get_counts(&ints);
    // println!("{:?}", int_counts);

    let mut largest_count = 0;
    let mut mode_int = 0;
    for (int, count) in int_counts {
        if count > largest_count {
            largest_count = count;
            mode_int = int;
        } else if count == largest_count {
            mode_int = 0;
        }
    }

    if ints.len() < 1 {
        println!("There were no numbers to check");
    } else if mode_int == 0 {
        println!(
            "Multiple numbers are contained in this list {} times",
            largest_count
        );
    } else {
        println!(
            "{} appears the most times in this list ({} times)",
            mode_int, largest_count
        );
    }
}
