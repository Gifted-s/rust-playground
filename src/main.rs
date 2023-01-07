fn main() {
    let x = Some(4);
    let plus_one = plus_one(x);
    println!("Plus one , {:?}", plus_one);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(val) => Some(val + 1),
        // None=>None,
        _ => None,
    }
}
