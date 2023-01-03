//====================== Rust Playground(Compound types) =============================
fn main() {
    let sum = function(5, 4);
    println!("Sum is {} ", sum)
}

fn function(x: i32, y: i32) -> i32 {
    x + y // this is automatically returned - how interesting :)
}
