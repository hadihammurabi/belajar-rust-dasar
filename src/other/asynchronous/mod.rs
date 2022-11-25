use async_std::task;

pub fn main() {
  task::spawn(async {
    println!("first")
  });

  println!("second")
}
