//====================== Rust Playground(Compound types) =============================
fn main() {
 
   // tuples 
  let  tup = ("Value 1", 40);
  let (str,num) = tup;
  let value1 = tup.1; // interesting :) 

  // Array -  Static size
  let errorCodes = [400, 401, 404];
  let notFound = errorCodes[2]; 
  let bytes = [0;8]; // create an array of size 8 and fill it with 0s   

  //vectors later
}
