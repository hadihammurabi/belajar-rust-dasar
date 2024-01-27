pub fn reference() {
  let x = String::from("hello");
  let y = &x;
  reference_print(&x, y);

  let mut s = String::from("mut this");

  {
    let a = &s;
    let b = &s;
    reference_print(a, b);
  }

  s = String::from("mutated");
  reference_print(&s, &s);
}

fn reference_print(x: &String, y: &String) {
  println!("x: {x}, y: {y}");
}