use rand::{Rng, Crypto, ErrorKind::Transient};
use front_of_house;
// use std::io::{self, Write};

use std::io::*;

// use std::fmt;
// use std::io::Result as IoResult;



// fn function() -> fmt::Result{
//   Ok(())
// }

// fn function2() -> IoResult<()> {
//   Ok(())
// }







// use allows you to bring a path into scope
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
  let secret_number = rand::thread_rng().gen_range(1,101);

  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}

// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant(){
//   // Note that all variant of an enum is public. Enums will not be useful if variants are private
//   let order1 = back_of_house::Appetizer::Soup;
//   let order2 = back_of_house::Appetizer::Salad;
// }




// mod back_of_house {

//     pub struct Breakfast {
//        pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant(){
//   let mut meal = back_of_house::Breakfast::summer("Lagos");

//   meal.toast = String::from("Bread");
// }

// fn serve_order(){}

// mod back_of_house{
//     fn fix_incorrect_order(){
//       cook_order();
//       super::serve_order()
//     }

//     fn cook_order(){}
// }

// mod front_of_house{
//   pub mod hosting {
//    pub  fn add_to_waitlist(){}
//     fn seat_at_table(){}
//   }

//   mod serving {
//     fn take_order(){}
//     fn serve_order(){}
//     fn take_payment(){}
//   }

// }

// pub fn eat_at_restaurant(){
//   //Absolute path
//   crate::front_of_house::hosting::add_to_waitlist();

//   //Relative path
//   front_of_house::hosting::add_to_waitlist();
// }
