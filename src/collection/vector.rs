#![allow(dead_code)]

pub fn basic() {
  let mut scores = vec![80, 70, 85, 90, 75];
  scores.push(95);
  scores.pop();
  println!("{:?}", scores);

  let mut total = 0;
  for score in &scores {
    total += score;
  }
  let average = total / &scores.len();
  println!("Average: {}", average);
}
