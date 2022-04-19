pub fn public_void() {
  println!("public_void: completed");
}

pub fn public_ret() -> i32 {
  println!("public_ret: completed");
  return 10;
}

pub fn public_param(a: i32, b: i32) -> i32 {
  println!("public_param: completed");
  return a + b;
}
