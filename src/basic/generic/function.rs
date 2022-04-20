use std::cmp::PartialOrd;

pub fn function() {
  let numbers = vec![3,2,4,2,3,1,3,2,3,2,4,5,6,5,3,5,3];
  let largest = get_largest(numbers);
  println!("Largest: {}", largest);
}

fn get_largest<T: PartialOrd + Copy>(numbers: Vec<T>) -> T {
  let mut largest = numbers[0];
  for number in numbers {
    if number > largest {
      largest = number;
    }
  }
  largest
}
