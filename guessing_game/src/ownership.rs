fn main(){

    // let c = dangle();

    // let s1 = String::from("Hello");
    // let s2 = s1.clone();
    // println!("{} wprld", s1)

    // let s =  String::from("Hello world");
    // takes_ownership(s);
    // println!("{}", s)

    // let x = 5;
    // makes_copy(x);
    // println!("{}", x);

    // let x = gives_ownership(String::from("Boyode"));

    // let mut s1 = String::from("Hello");
    // let r1 = &s1;
    // let r2 = &s1;

    // // This will show an error immuttable references does not expect
    // // underlining data to change and having a mutable reference breaks this rule.
    // // let r3 = &mut s1;


    // // note that the scope of a reference starts when we declare it and ends when it
    // // is last used. So s1 reference is dropped here
    // println!("{}, {}", r1,r2);
    
    // // this is valid because we dropped the scope of s1 reference above
    // let r3 = &mut s1;



    // change(&mut s1);
    // We did not pass string as value here because we don't want to move it,
    // rather we want to borrow it without taking ownership so we pass a referene to the string.
    // let string_length =get_length(&s1);
    // println!("{}, {}", s1, string_length)


    // rules of references
    // 1. At a given point you can have either one mutable reference or any number
    // of immutable reference
    //
    // 2. Ref must always be valid
    //


     
 
}

// fn takes_ownership(string: String){
//    println!("{}", string);
// }

// fn gives_ownership(string: String)-> String{
//     string
//  }

// fn makes_copy(num: i32){
//     println!("{}", num);
//  }

// fn get_length(s: &String)-> usize{
   
//   let length = s.len();
//   length
// }

// fn change(s: &mut String){
//      s.push_str("Sunkanmi");
  
//   }

// use lifetime - later
// fn dangle()-> &String{
//     let s = String::from("Hello");
//     &s
// }

