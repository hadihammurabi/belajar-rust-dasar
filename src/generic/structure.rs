struct Wrap<T> {
  value: T,
}

impl Wrap<u32> {
  pub fn get_value(&self) -> u32 {
    self.value
  }
}

pub fn structure() {
  let w1 = Wrap{
    value: 4,
  };
  let w2 = Wrap{
    value: 5,
  };
  let w3 = Wrap{
    value: 6,
  };

  println!("Value 1: {}", w1.get_value());
  println!("Value 2: {}", w2.get_value());
  println!("Value 3: {}", w3.get_value());
}
