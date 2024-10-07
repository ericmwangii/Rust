//ownership rules
//1. Each value In Rust has an owner
//2. There can only be one owner at a time
//3. When the owner goes out of scope, the value will be dropped.

fn main() {
   //variable scope
   //s not valid here
   let _s = "hello";  //valid here


   let mut t = String::from("hello");

   t.push_str(", world!");  //append string literal

   println!("{t}");  //hello, world!


   //variables and data interacting with Move

   let s1 = String::from("Hi");
   //let s2 = s1;  //moves the value
   let s2 = s1.clone(); //->deep copy

   println!("s1 = {s1}, s2 = {s2} world!");






}
