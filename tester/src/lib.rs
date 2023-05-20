

#[cfg(test)]
mod tests2 {
    #[test]
    fn it_works(){
        assert_eq!(2 + 2 ,4)
    }


    #[test]
    #[ignore]
    fn expensive_test(){
      // code that takes too long to run
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn prints_and_return_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_test() {
        assert_eq!(10, add_two(8));
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_return_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_return_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two does not equal four "))
        }
    }
    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4)
    }
}
