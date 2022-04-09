#![allow(dead_code)]

use std::collections::HashMap;

pub fn basic() {
  let mut student: HashMap<&str, &str> = HashMap::new();
  student.insert("name", "Alex Under");
  student.insert("grade", "VIII");
  println!("{:?}", student);
}
