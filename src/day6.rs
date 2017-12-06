use std::io;
use std::collections::HashMap;

fn get_index(vec: &Vec<u32>) -> (u32, u32) {
  let mut max_index = 0;
  let mut max_value = (*vec)[0];
  for (i, val) in vec.into_iter().enumerate() {
    if *val > max_value {
      max_index = i;
      max_value = *val;
    }
  }
  return (max_index as u32, max_value);
}

fn get_next(vec: &Vec<u32>) -> Vec<u32> {
  // get index of max value, lowest index
  let (idx_to_distribute, value_to_distribute) = get_index(&vec);

  let mut some_vec = vec.clone();

  some_vec[idx_to_distribute as usize] = 0;

  let mut left = value_to_distribute;
  let mut i = idx_to_distribute as usize + 1;
  // TODO: Optimize this, this is too naive
  loop {
    if i >= vec.len() {
      i = 0;
    }
    if left == 0 {
      break;
    }
    some_vec[i] += 1;
    left -= 1;
    i += 1;
  }

  for x in &some_vec {
    print!("{} ", x);
  }
  println!("...");

  return some_vec;
}

fn get_count(vec: Vec<u32>, hash_map: &mut HashMap<Vec<u32>, u32>) -> u32 {
  if hash_map.contains_key(&vec) {
    return 0;
  }

  let new_vec = get_next(&vec);
  hash_map.insert(vec, 1);

  return 1 + get_count(new_vec.clone(), hash_map);
}

pub fn main() {
  let mut vec: Vec<u32> = Vec::new();
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  for num_str in input.trim().split_whitespace() {
    vec.push(num_str.parse().unwrap());
  }
  let mut hash_map = HashMap::new();

  let count = get_count(vec.clone(), &mut hash_map);

  println!("{}", count);
}
