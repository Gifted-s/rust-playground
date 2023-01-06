fn main() {
    let x = 5;

    movei(x);
    println!("name is {}", x); // move does not happen for simple primitive types
                               // let name = String::from("hello");
                               // let name2 = name;
                               // println!("{} String here", name);

    // let name = String::from("hello");
    // moves(name);
    // println!("{} String here", name); // a move has happend here an error will be diplayed

    // Not recommended but you can take and give ownership
    let mut s1 = String::from("Sunkanmi");
    let length = length(&s1);
    change(&mut s1);
    let s2 = ownership(s1);
    println!("{} String here", s2);

    // reference recommended

    /*
        this will throw an error because you can't have two mutable reference to a single variable to prevent DATA RACE (think of two threads
        with shared access to read or modify a single resources without control  )
    */
    // let mut s = String::from("test");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // works(all reference immutable)
    // let s = String::from("test");
    // let r1 = &s;
    // let r2 = &s;
    // println!("{}, {}", r1, r2);

    // this will fail because RUST does not allow you to have both mutable and immutable reference of thesame variable together(think of you must be specific, don't confuse the compiler :) )
    // let mut s = String::from("test");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;

    // println!("{}, {}", r1, r2);

    // Note that the scope of a reference begins when its first introduced and ends the last time its used so this code will work
    // let mut s = String::from("test");
    // let r1 = &s;
    // let r2 = &s;
    // println!("{}, {}", r1, r2);
    // let r3 = &mut s;
    // println!("{} ", r3);

    // DANGLING POINTERS
    let reference = dangle();
}

fn dangle() -> &String {
    let s = String::from("Scifi");
    &s
}

fn moves(name: String) {
    println!("name is {}", name);
}
fn movei(x: i32) {
    println!("name is {}", x);
}

fn ownership(s1: String) -> String {
    s1
}

fn length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("hello");
}
