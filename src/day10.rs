use std::io;

pub fn main() {
  let mut vec: Vec<u32> = Vec::new();
  let mut number = 0;
  while number < 256 {
    vec.push(number);
    number += 1;
  }

  let mut current_position = 0;
  let mut current_skip = 0;

  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let mut lengths = input.trim().split(",");

  for length_str in lengths {
    let length: u32 = length_str.trim().parse().unwrap();

    let mut collected = 0;
    let mut temp_position = current_position;

    // Collect vec to reverse
    let mut reversed_vec = Vec::new();
    while collected < length {
      let vec_value = vec[temp_position as usize];

      reversed_vec.push(vec_value);

      temp_position += 1;
      collected += 1;

      if temp_position >= vec.len() as u32 {
        temp_position = 0;
      }
    }
    reversed_vec.reverse();

    // Apply vec
    let mut reversed_vec_iter = reversed_vec.into_iter();
    let mut applied = 0;
    temp_position = current_position;
    while applied < length {
      vec[temp_position as usize] = reversed_vec_iter.next().unwrap();

      temp_position += 1;
      applied += 1;

      if temp_position >= vec.len() as u32 {
        temp_position = 0;
      }
    }

    current_position = (current_position + length + current_skip) % vec.len() as u32;
    current_skip += 1;

    println!("{} {} {}", length, current_position, current_skip);
  }

  let answer = vec[0] * vec[1];
  println!("{}", answer);
}
