#![allow(dead_code)]

mod thread;
mod channel;

pub fn main() {
  thread::run(4);
  channel::run();
}