
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn canhold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}




pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn failing_test() {
//         let result = add(2, 2);
//         assert_eq!(result, 6);
//     }
// }



pub struct Guess {
    value: i32,
}


impl Guess {
    pub fn new(value: i32) -> Guess {
        if value <1 || value > 100{
            panic!("Guess value must be between 1 and 100, got {}", value)
        }
        Guess{value}
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test] 
  #[should_panic(expected = "Guess value must be between 1 and 100, got -2")]
  fn greater_than_100(){
    Guess::new(-2);
  }

  fn it_works() -> Result<(), String> {
    if 2+3 ==4 {
        Ok(())
    } else {
        Err(String::from("Two plus two does not equal four "))
    }
  }



  #[test]
  fn larger_can_hold_smaller() {
       let larger = Rectangle{
        width:30,
        height: 30
       };

       let smaller = Rectangle{
        width:6,
        height: 10
       };

       assert!(larger.canhold(&smaller));
    
  }

  #[test]
  fn smaller_cannot_hold_larger() {
       let larger = Rectangle{
        width:30,
        height: 30
       };

       let smaller = Rectangle{
        width:6,
        height: 10
       };

       assert!(!smaller.canhold(&larger));
    
  }


  #[test]
  fn not_eq() {
       let larger = Rectangle{
        width:30,
        height: 30
       };

       let smaller = Rectangle{
        width:6,
        height: 10
       };

       assert_ne!(30, smaller.area());
    
  }

  #[test]
  fn greeting_contains_name(){
    let result = greeting("Carol");
    assert!(result.contains("Carol"));
  }

  #[test]
  fn greeting_contains_name_should_fail(){
    let res = greeting("Carol");
    assert!(res.contains("Tayo"), "Greetings name does not contiain Carol, value was {}", res);
  }
}
