use std::io;

pub fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input);

  let mut prev_digit = input.trim().chars().last().unwrap();
  let mut sum = 0;

  for digit in input.trim().chars() {
    if prev_digit == digit {
      sum += digit.to_digit(10).unwrap();
    }
    prev_digit = digit;
  }

  println!("{}", sum);
}
