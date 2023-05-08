use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
fn main() {

let text = "Hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace(){
    let count = map.entry(word).or_insert(0);
    *count +=1;
}

println!("{:?}", map);


//    let mut scores = HashMap::new();

//    scores.insert(String::from("Blue"), 10);
//    scores.insert(String::from("Blue"), 30);// this will overwrite

//    // this does not overwrite the exisiting entry. If the entry exist return it, otherwise return default value
//    scores.entry(String::from("Yellow")).or_insert(30);
//    scores.entry(String::from("Yellow")).or_insert(40);





    // let blue = String::from("Blue");
    // let yellow = String::from("Yellow");

    // let mut scores = HashMap::new();

    // scores.insert(blue, 30); // Note: this takes ownership of the key string  blue
    // scores.insert(yellow, 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);

    // for (key, value) in &score {
    //     println!("{} : {}", key, value);
    // }


    // so this will fail
    // println!("{}", blue);

    //    let hello: String = String::from("Hello");
    //    let c: char = hello[0];

    // let hello: String = String::from("ناماستي");

    // for b in "ناماستي".bytes() {
    //     println!("{}", b);
    // }

    // for c in "ناماستي".chars() {
    //     println!("{}", c);
    // }

    // for g in "ناماستي".graphemes(true) {
    //     println!("{}", g);
    // }

    // Strings are stored as a collection of UTF-8 encoded bytes

    // let s1 = String::new;
    // let s2 = "Initial String";
    // let s3 = s2.to_string();
    // let s4 = String::from("Initial Contents");

    // // Truly UTF-8
    // let hello = String::from("Hello");
    // let hello = String::from("¿Como está?");

    // Append to string
    // let mut s = String::from("foo");
    // s.push_str("bar");
    // s.push('!');

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("World");

    // let s3 = s1 + &s2;
    // println!("{}", s1);

    // enum SpreadSheetCell{
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }
    // // It is true that a vector can only store data of thesame type but we can store an enum in a vector and that can have multiple types
    // let row = vec![
    //     SpreadSheetCell::Int(32),
    //     SpreadSheetCell::Text(String::from("blue")),
    //     SpreadSheetCell::Float(10,20),
    // ];

    // match &row[1]{
    //     SpreadSheetCell::Int(i)=> println!("{}", i),
    //     _ => println!("Not Integer !")
    // }

    // let mut v = vec![1,2,3,4,5];

    // for i in &mut v{
    //     *i+=30;
    //     println!("{}", i);
    // }

    // let a = [1, 2, 3, 4, 5];
    // let mut v: Vec<i32> = Vec::new();

    // v.push(1);
    // v.push(2);
    // v.push(3);

    // let v2 = vec![1, 2, 3];
    // let third = &v[2];
    // println!("The third element is {}", third);

    // match v.get(20){
    //     Some (v)=> println!("{}", v),
    //     None => println!("There is no third element")
    // }
}
