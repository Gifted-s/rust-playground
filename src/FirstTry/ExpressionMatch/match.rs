fn main() {
    let x = 5;
    // let y: Option<i32> = Some(10);
    let y: Option<i32> = None;
    let sum = x + y.unwrap_or(0);

    let centvalue = valueincent(Coin::Quatar(UsState::Alaska));
    println!("Cent Value {}", centvalue);
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    //..
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quatar(UsState),
}

fn valueincent(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 30,
        Coin::Nickel => 50,
        Coin::Dime => 60,
        Coin::Quatar(state) => {
            println!("Coin type {:?}!", state);
            4
        }
    }
}
