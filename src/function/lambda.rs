pub fn public_anonymous(a: i32, b: i32) {
  let adder = || {
    a + b
  };

  println!("public_anonymous: completed");
  adder();
}
