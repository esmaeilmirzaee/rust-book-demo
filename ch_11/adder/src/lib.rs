#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

pub fn add_two(num: i32) -> i32 {
    num + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting didn't work as expected, the return value is {}.",
            result
        );
    }

    #[test]
    fn test_add_two() {
        assert_eq!(6, add_two(4));
    }

    #[test]
    fn test_ne_add_two() {
        assert_ne!(6, add_two(3));
    }

    #[test]
    fn test_can_hold() {
        let r1 = Rectangle {
            width: 8,
            height: 9,
        };
        let r2 = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(r1.can_hold(&r2));
        // the following is not a seperate test
        assert!(!r2.can_hold(&r1));
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        if false {
            panic!("Failed to test");
        }
    }
}
