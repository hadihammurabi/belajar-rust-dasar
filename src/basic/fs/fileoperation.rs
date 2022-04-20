#![allow(dead_code)]

use std::fs;

pub fn read() {
  let contents = match fs::read_to_string("hello.txt") {
    Ok(c) => c,
    Err(err) => panic!("can't read file: {:?}", err),
  };

  println!("{}", contents);
}
