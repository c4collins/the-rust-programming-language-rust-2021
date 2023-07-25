#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn get_rectangles() -> (Rectangle, Rectangle) {
        let larger: Rectangle = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller: Rectangle = Rectangle {
            width: 5,
            height: 1,
        };
        (larger, smaller)
    }

    #[test]
    #[should_panic]
    fn another_test() {
        panic!("This will fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let (larger, smaller) = get_rectangles();
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let (larger, smaller) = get_rectangles();
        assert!(!smaller.can_hold(&larger));
    }
}
