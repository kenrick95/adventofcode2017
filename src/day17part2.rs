use std::io;

pub fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  let num_steps_per_count: usize = input.trim().parse().unwrap();

  let mut counter = 0;
  let max = 50000000;//2017;

  // let mut buffer = vec![0];
  let mut current_index = 0;
  let mut current_zero_index = 0;
  let mut current_zero_next = -1;
  let mut current_length = 1;

  // Buffer content is not imporant, and resize is expensive, just keep track of where the zero is; and the value of its neighbour.
  while counter < max {
    counter += 1;

    let next_length = current_length + 1;
    let next_index = (current_index + num_steps_per_count) % current_length + 1;

    // println!("insert {} to {}", counter, next_index);

    if next_index == current_zero_index {
      current_zero_index = (current_zero_index + 1) % next_length;
    } else if next_index == current_zero_index + 1 {
      current_zero_next = counter;
    }

    // println!("> {} {} {}", counter, current_zero_index, current_zero_next);

    // buffer.insert(next_index, counter);

    // for elem in buffer.clone() {
    //   print!("{} ", elem);
    // }
    // println!();

    current_index = next_index;
    current_length = next_length;
  }

  println!("{}", current_zero_next);
}
