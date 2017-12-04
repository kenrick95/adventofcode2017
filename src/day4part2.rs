use std::io;
use std::collections::HashMap;
use std::iter::FromIterator;

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
      // Sort the phrase as key to hashmap
      // https://stackoverflow.com/a/42729940/917957
      let mut key: Vec<char> = phrase.chars().collect();
      key.sort_by(|a, b| b.cmp(a));
      let key_str = String::from_iter(key);

      if (dict.contains_key(&key_str)) {
        valid = false;
        break;
      }
      dict.insert(key_str, 1);
    }

    if (valid) {
      count += 1;
    }

    return true;

  })() {
  }

  println!("{}", count);
}