pub fn reference() {
  let x = String::from("hello");
  let y = &x;
  reference_print(&x, y);
}

fn reference_print(x: &String, y: &String) {
  println!("x: {} & y: {}", x, y);
}

