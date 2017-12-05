use std::io;

pub fn main() {
  let mut count = 0;
  let mut current_index: i32 = 0;
  let mut vec: Vec<i32> = Vec::new();

  loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "END" {
      // TODO: Replace with EOF character
      break;
    }
    vec.push(input.trim().parse().unwrap());
  }

  let max_index: i32 = (vec.len() - 1) as i32;

  loop {
    if current_index < 0 || current_index > max_index {
      break;
    }
    let current_value = vec[current_index as usize];
    // println!(
    //   "current_index {}, current_value {}",
    //   current_index,
    //   current_value
    // );
    vec[current_index as usize] += 1;
    current_index += current_value;
    count += 1;
  }


  println!("{}", count);
}
