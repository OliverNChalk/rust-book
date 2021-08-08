#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        return Rectangle { width, height };
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn failing_test() {
    //     panic!("This test panics!");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 25,
            height: 15,
        };
        let smaller = Rectangle {
            width: 10,
            height: 10,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 25,
            height: 15,
        };
        let smaller = Rectangle {
            width: 10,
            height: 10,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn equal_sized_can_hold() {
        let larger = Rectangle {
            width: 25,
            height: 15,
        };

        assert!(larger.can_hold(&larger));
    }
}
