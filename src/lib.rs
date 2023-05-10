#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 50,
            height: 40,
        };
        let smaller = Rectangle {
            width: 30,
            height: 20,
        };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 50,
            height: 40,
        };
        let smaller = Rectangle {
            width: 30,
            height: 20,
        };

        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    #[should_panic(expected = "Guess was greather than 10")]
    fn greater_than_10() {
        validate_guess(20);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 3 == 4 {
            Ok(())
        } else {
            Err(String::from("err"))
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn validate_guess(guess: i32) {
    if guess > 10 {
        panic!("Guess was greather than 10")
    }
}
