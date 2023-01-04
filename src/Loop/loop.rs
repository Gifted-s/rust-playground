//====================== Rust Playground(Loops) =============================
fn main() {
    let mut counter = 1;
    // Freaking awesome - you can assign the result of a loop to a variable - Just thinking of the compiler Lol :)
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter;
        }
    };

    println!("Result  {}", result);

    // WHILE LOOP
    let mut num = 5;
    while num != 0 {
        println!("Holy Loard {}", num);
        num -= 1;
    }
    println!("Done");

    // FOR LOOP
    let nums = [200, 600, 300, 100];
    for num in nums.iter() {
        println!("num: {}", num);
    }
    for num in 1..5 {
        // Amazing :)
        println!("num: {}", num);
    }
}
