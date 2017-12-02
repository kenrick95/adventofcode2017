use std::io;

pub fn main() {
  let mut sum = 0;

  while (|| {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "-1" { // TODO: Replace with EOF character
      return false;
    }

    let length = input.trim().split_whitespace().count();
    let mut i = 0;

    while (i < length - 1) {
      let mut j = i + 1;
      while (j < length) {
        let num1: u32 = input.trim().split_whitespace().nth(i).unwrap().parse().unwrap();
        let num2: u32 = input.trim().split_whitespace().nth(j).unwrap().parse().unwrap();

        if num1 % num2 == 0 {
          sum += num1 / num2;
        } else if num2 % num1 == 0 {
          sum += num2 / num1;
        }
        j += 1;
      }
      i += 1;
    }
    println!("{}", sum);

    return true;

  })() {
  }

  println!("{}", sum);
}
