#![allow(dead_code)]

use std::collections::HashMap;

pub fn basic() {
  let mut student: HashMap<&str, &str> = HashMap::new();
  student.insert("name", "Alex Under");
  student.insert("grade", "VIII");
  println!("{:?}", student);
  
  // NOTE: panic, no entry found for key "age"
  // println!("age: {:?}", student["age"]);

  // if student.contains_key("age") {
  //   println!("age: {:?}", student["age"]);
  // }

  // match student.get("age") {
  //   Some(val) => println!("the age is {}", val),
  //   _ => println!("age not found")
  // }
} 
