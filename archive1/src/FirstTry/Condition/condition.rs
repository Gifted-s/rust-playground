 //====================== Rust Playground(Conditions) =============================
fn main() {
    let condition = true;
    let name = if condition { "Adewumi" } else { "Sunkanmi" }; // interesting -  inline condition check
    println!("Name is {} ", name);
    let num = 10;
    if num < 30 {
        println!("Less than 30");
    } else if num > 15 {
        println!("Greater than 15 ")
    } else {
        println!("Within range")
    }
}
