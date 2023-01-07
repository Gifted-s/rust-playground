 fn main(){

    //  let mut s = String::from("Hello world");
    //  let length = firstword(&s); 
    //  s.clear();
    // this is not good because even after clearing the string,  we can still print its length 
    // the only option we have is to keep the "length" in sync with "s" which is also not recommended
    //  println!("First word length is {}", length);
    // let mut s = String::from("Hello World");
    // let hello = &s[0..1];
    // let world = &s[..]; 
    
    // let firstname = firstword(&s);
    // s.clear();
    // println!("First name {}", firstname);
     
} 

 
fn firstword(s: &String)->&str{
   let bytes =  s.as_bytes();
  for (i , &item) in bytes.iter().enumerate(){
     if item == b' '{
        return  &s[..i]
     }
  }; 
  &s[..]
} 
 