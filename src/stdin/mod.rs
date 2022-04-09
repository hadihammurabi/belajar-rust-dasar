use std::io;

fn get_input(prompt: &str) -> String {
  println!("{}: ", prompt);

  let mut value = String::new();
  io::stdin()
    .read_line(&mut value)
    .expect("Failed get input");

  value
}

pub fn main() {
  let data = get_input("Input");
  println!("{}", data);
}
