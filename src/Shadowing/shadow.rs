//====================== Rust Playground(Guess game) =============================
fn main() {
    let mut x = "Sunkanmi";
    println!("The value of X is, {}", x);
    x = "New value";
    println!("The value of X is, {}", x);

    const SUBSCRIBER: u32 = 100000;
    println!("SUBSCRIBER, {}", SUBSCRIBER);
    // shadowing
    let y = "Sunkanmi";
    println!("The value of X is, {}", y);
    let y = 60;
    println!("The value of X is, {}", y);



}
