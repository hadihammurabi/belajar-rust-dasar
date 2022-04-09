pub fn copy() {
  let x = 10;
  let y = x;
  copy_print(x, y);
}

fn copy_print(x: i32, y: i32) {
  println!("x: {} & y: {}", x, y);
}
