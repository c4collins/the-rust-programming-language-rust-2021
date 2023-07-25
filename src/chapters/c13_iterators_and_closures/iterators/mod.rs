#[derive(PartialEq, Debug, Clone)]
struct Shoe {
    size: u32,
    style: String,
}

pub fn main() {
    println!("Iterators");
    consume_an_iterator();
}

fn get_vec<'a>() -> Vec<u32> {
    vec![1, 2, 3]
}

fn consume_an_iterator() {
    for val in get_vec().iter() {
        println!("Got: {}", val);
    }
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_demonstration() {
        let v1 = get_vec();
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn test_iterator_sum() {
        let total: u32 = get_vec().iter().sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn test_iterators_that_produce_iterators() {
        let v2: Vec<_> = get_vec().iter().map(|x| x * 2).collect();
        assert_eq!(v2, vec![2, 4, 6]);
    }

    #[test]
    fn test_filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let test_against = vec![shoes[0].clone(), shoes[2].clone()];
        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(in_my_size, test_against);
    }
}
