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
