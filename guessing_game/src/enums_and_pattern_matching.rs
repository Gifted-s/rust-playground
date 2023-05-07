


// Enums
// Pattern matching
// Optional enum
// If let syntax

enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddressKind,
    address: String,
}

// enum Message{
//     Quit,
//     Move{x:i32, y:i32},// annonymous struct
//     Write(String),
//     ChangeColor(i32,i32,i32)
// }
fn main() {
    //   let four = IpAddressKind::V4;
    //   let six  = IpAddressKind::V6;

    //   let localhost = IpAddr{
    //     kind: IpAddressKind::V4,
    //     address: String::from("127.0.0.1")
    //   }
    //  let localhost = IpAddrKind::V4(String::from("127.0.0.1"));

    // Option variant
    //  let x:i8 =5;

    //  let y: Option<i8>= None;

    //  let sum = x+y.unwrap_or(0);
    // let num = value_in_cents(Coin::Quarter(UsState::Alabama));
    // println!("Number {}", num);

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    // If let syntax
    // Good :(
    let some_value = Some(3);
    match some_value {
        Some(i) => println!("Three is here {}", 4),
        _ => (),
    }
    // Excellent :) but not that readable :( - tradeoff
    if let Some(3) = some_value {
        println!("Three")
    }

}

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     Arizona,
//     Arkansas,
//     California,
//     //..
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Dime => 1,
//         Coin::Nickel => 5,
//         Coin::Penny => 4,
//         Coin::Quarter(state)=>{
//             println!("State quarter from {:?}!", state);
//             7
//         }
//     }
// }
// fn route(ip_kind: IpAddressKind) {}

// fn plus_one(x:Option<i32>)-> Option<i32>{
//     match x {
//         Some(i)=> Some(i+1),
//         _ => None,
//     }
// }
