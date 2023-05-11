use std::fs::{self, File};
// use std::fs::File;
// use std::io::ErrorKind;
fn main() {



// fn main(){
//     let home: IpAddr = "127.0.0.1".parse().unwrap();
// }

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value <1 || value >100{
            panic!("Guess value must be between 1 and 100, got {}.", value)
        }
        Guess{value}
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


// fn main() Result <(), Box<dyn Error>>{
//    let f = File::open("hello.txt")?;
//    Ok(())
// }


// Error propagation

// fn read_username_from_file() -> Result<String, io::Error>{
    //##########################################################
    //let mut s = String::new();
    // try to open the file, if it is successful, move to string otherwise return the error
    // In addition, if you cannot read to string then return error
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)
    //########################################################## can we make this simpler?, yes


    // let mut f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e)  => return Err(e),
    // }

    // let mut s = String::new();

    // match f.read_to_string(&mut s){
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    // f.read_to_string(&mut s)?;
    // Ok(s)
}
    


    // let f = File::open("hello.txt").expect("Error opening file");



    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", other_error)
    // };





    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // //Using closure - Looks rusty :)
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem crating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the fole")
    //     }
    // });
    // enum Result<T, E>{
    //     Ok(T),
    //     Err(E),
    // }
// }

// fn a(){
//     b();
// }

// fn b(){
//    c(21)
// }

// fn c(num :i32){
//    if num==22{
//     panic!("Dont pass 22!");
//    }
// }

// // RUST_BACKTRACE=1 cargo run
