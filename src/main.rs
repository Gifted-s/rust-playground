// Dec 29, 2022
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;


//====================== Rust Playground(Guess game) =============================
fn main() {
    println!("Guess a number");
 
    loop {
        println!("Please input your guess");
        
        let secret = rand::thread_rng().gen_range(1, 101);
        println!("The secret number is: {} ", secret); 
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to accept input");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue 
         };
        match guess.cmp(&secret) {
            Ordering::Greater => println!("{}","Too big".red()),
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Equal => {
                println!("{}","You win".green());
                break;
            },
        }

        println!("you guessed: {} ", guess);
    }
}
