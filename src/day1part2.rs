use std::io;

pub fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input);

  let halfway = input.trim().len() / 2;
  let mut halfway_pointer = input.trim().chars().cycle().skip(halfway);
  let mut prev_digit = halfway_pointer.next().unwrap();
  let mut sum = 0;

  for digit in input.trim().chars() {
    if prev_digit == digit {
      sum += digit.to_digit(10).unwrap();
    }
    prev_digit = halfway_pointer.next().unwrap();
  }

  println!("{}", sum);
}
