use std::io;
use std::collections::HashMap;

pub fn main() {
  let mut count = 0;

  while (|| {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "-1" { // TODO: Replace with EOF character
      return false;
    }

    let mut dict = HashMap::new();
    let mut valid = true;

    for phrase in input.trim().split_whitespace() {
      if (dict.contains_key(phrase)) {
        valid = false;
        break;
      }
      dict.insert(phrase, 1);
    }

    if (valid) {
      count += 1;
    }

    return true;

  })() {
  }

  println!("{}", count);
}