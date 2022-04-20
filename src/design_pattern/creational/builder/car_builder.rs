#![allow(dead_code)]

use crate::design_pattern::creational::builder::car::Car;

pub struct CarBuilder {
  pub seats: i32,
  pub color: String,
}

impl CarBuilder {
  fn set_seats(&mut self, seats: i32) -> &mut Self {
    self.seats = seats;
    self
  }
  pub fn big(&mut self) -> &mut Self {
    self.set_seats(6)
  }
  pub fn medium(&mut self) -> &mut Self {
    self.set_seats(4)
  }
  pub fn small(&mut self) -> &mut Self {
    self.set_seats(2)
  }
  
  fn set_color(&mut self, color: &str) -> &mut Self {
    self.color = color.to_owned();
    self
  }
  pub fn red(&mut self) -> &mut Self {
    self.set_color("red")
  }
  pub fn yellow(&mut self) -> &mut Self {
    self.set_color("yellow")
  }
  pub fn silver(&mut self) -> &mut Self {
    self.set_color("silver")
  }
  pub fn black(&mut self) -> &mut Self {
    self.set_color("black")
  }
  pub fn white(&mut self) -> &mut Self {
    self.set_color("white")
  }

  pub fn build(&mut self) -> Car {
    Car{
      seats: self.seats,
      color: self.color.to_owned(),
    }
  }
}
