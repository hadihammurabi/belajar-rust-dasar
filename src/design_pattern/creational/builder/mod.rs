pub mod car;
pub mod car_builder;

pub fn main() {
  let c1 = car::Car::new().big().red().build();
  println!("{:?}", c1);
}