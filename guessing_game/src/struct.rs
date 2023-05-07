
// struct  User {
//     name : String,
//     email: String,
//     age:u32,
//     active: bool,
// }

// // For display trait
// #[derive(Debug)]
// struct  Rect {
//     w:u32,
//     h:u32,
// }

// impl Rect {
//     fn area(&self)->u32{
//       self.h * self.w
//     }

//     fn can_hold(&self, other: &Rect)-> bool{
//         self.w > other.w && self.h>other.h
//     }
// }

// impl Rect{
//     fn square(size: u32)-> Rect{
//         Rect {
//             h: size,
//             w: size
//         }
//     }
// }





// fn main(){
// let rect = Rect{
//     w:32,
//     h:32,
// };

// let rect2 = Rect{
//     w:43,
//     h:65,
// };


// let rect3 = Rect::square(32);



// println!("rect is {:#?}", rect);
// println!("Can hold {}", rect2.can_hold(&rect));
// println!("Square oh Host {:#?}", rect3);
// //Tuple struct
// //Tuple struct are useful when you want a tuple to be of  different types and have a different name
// // struct Color(i32, i32,i32);
// // struct Point (i32, i32,i32);

// // let red = Color{32,43,54};
// // let pos = Point{32,34,23};

// //     let mut user = User{
// //         name:String::from("Adewumi"),
// //         email:String::from("sun@gmail.com"),
// //         age:32,
// //         active:true
// //     };

// //    let name = user.name;
// //    user.name = String::from("Boyod");
// //    let user2  = build_user(String::from("Boyod2"),
// //    String::from("sun@gmail.com2") );


// //    let user3  =  User{
// //     name:String::from("Adewumi"),
// //     email:String::from("sun@gmail.com"),
// //     ..user2


// //    println!("{}", user.name);
// }


// // fn build_user(name:String, email: String)->User{
// //    //ield init shorthand syntax 
// //    User {
// //      name,
// //      email,
// //      age:30,
// //      active:true
// //    }
// // }

