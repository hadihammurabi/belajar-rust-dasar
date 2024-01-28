#![allow(dead_code)]
mod vector;
mod hashmap;
mod array;

pub fn main() {
  println!("");
  println!("--==:: VECTOR ::==--");
  vector::basic();

  println!("");
  println!("--==:: HASHMAP ::==--");
  hashmap::basic();

  println!("");
  println!("--==:: ARRAY ::==--");
  array::main();
}
