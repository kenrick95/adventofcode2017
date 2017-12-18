use std::io;

pub fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  let num_steps_per_count: usize = input.trim().parse().unwrap();

  let mut counter = 0;
  let max = 2017;

  let mut buffer = vec![0];
  let mut current_index = 0;

  while counter < max {
    counter += 1;
    let current_length = buffer.len();

    let next_index = (current_index + num_steps_per_count) % current_length + 1;

    // println!("insert {} to {}", counter, next_index);

    buffer.insert(next_index, counter);

    // for elem in buffer.clone() {
    //   print!("{} ", elem);
    // }
    // println!();

    if counter == 2017 {
      println!("{}", buffer[(next_index + 1) % (current_length + 1)]);
      break;
    }

    current_index = next_index;
  }
}
