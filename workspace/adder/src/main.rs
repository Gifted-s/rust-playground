use add_one;

fn main() {
    let num = 10;
    println!(
        "House, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
    println!("Hello, world!");
}
