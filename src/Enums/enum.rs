enum ipAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Delete(String),
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn say() {
        println!("Rusty devloper yuck ");
    }
}

fn main() {
    let localhost = ipAddrKind::V4(String::from("127.0.0.1"));

    let message = Message::Delete(String::from("127.0.0.1"));
}
