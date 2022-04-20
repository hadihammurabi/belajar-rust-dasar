#![allow(dead_code)]

use crate::design_pattern::creational::builder::car_builder::CarBuilder;

#[derive(Debug)]
pub struct Car {
  pub seats: i32,
  pub color: String,
}

impl Car {
  pub fn new() -> CarBuilder {
    CarBuilder {
      seats: 0,
      color: "".to_owned(),
    }
  }
}
