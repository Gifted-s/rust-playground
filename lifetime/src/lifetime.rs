fn main1() {
    // let r ;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r)
    let x = 5;
    let r = &x;
    println!("{}", r);

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1, string2);
    println!("The longest string: {}", result)
}

fn main2() {
    let string1 = String::from("abcd");
    {
        // in this case the lifetime that will be used is that of string2 becasue it has the shortest lifetime
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    
}

fn main3() {
    let string1 = String::from("abcd");
    let result;
    {
        // in this case the lifetime that will be used is that of string2 becasue it has the shortest lifetime
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // result is a dangling reference :) since string2 lifetime was used and it ends in the scope above
    println!("The longest string is {}", result);
}

// &i32 // a reference
// &'a i32 // a reference with an explicit lifetime
// &'a mut i32  // a mutable reference with an explicit lifetime

// Generic annotation lifetime doesn't change the lifetime but create relationship between lifetimes of multiple values
// normally the smallest lifetime will be used
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// the ImportantExcerpt struct will use the part field lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main4(){
    let novel = String::from("Call me ishmael, Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find");
    // i here is vald as long as first_sentence is within scope
    let i = ImportantExcerpt{
        part: first_sentence,
    };

}


fn main5(){
    let novel = String::from("Call me ishmael, Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find");
    // i here is vald as long as first_sentence is within scope
    let i = ImportantExcerpt{
        part: first_sentence,
    };
}




// 1. Each parameter that is a reference gets its own lifetime parameter

// 2. If there is exactly one input lifetime parameter,that lifetime is assigned to all output lifetime parameters;

// 3. If there multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is
// assigned to all output lifetime parameters.


// lifetime of the params called input lifetime
// lifetime of the returned value called output lifetime
fn first_word<'a> (s : &'a str) -> &'a str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
     if item == b' ' {
       return &s[0..i];
     }
    }

    &s[..]
}

impl<'a> ImportantExcerpt<'a>{
    // we don't need to have someting like return_part<'a> because the impl expression above already have this
    fn return_part(&self, announcement: &str) ->&str{
        println!("Attention please: {}", announcement);
        self.part;
    }
}


fn main6(){
    let novel = String::from("Call me Daniel. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not cind string");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}




