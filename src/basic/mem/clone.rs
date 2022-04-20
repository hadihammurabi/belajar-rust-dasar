pub fn clone() {
  let x = String::from("hello");
  let y = x.clone();
  clone_print(x, y);
}

fn clone_print(x: String, y: String) {
  println!("x: {} & y: {}", x, y);
}
