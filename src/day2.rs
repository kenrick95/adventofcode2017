use std::io;

pub fn main() {
  let mut sum = 0;

  while (|| {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "-1" { // TODO: Replace with EOF character
      return false;
    }

    let mut largest = -1;
    let mut smallest = 999999;

    for num_str in input.trim().split_whitespace() {
      let num = num_str.parse().unwrap();
      if num < smallest {
        smallest = num;
      }
      if num > largest {
        largest = num;
      }
    }
    sum += largest - smallest;

    return true;

  })() {
  }

  println!("{}", sum);
}
