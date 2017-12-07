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

  let size = vec.len() as u32;

  let mut i = idx_to_distribute as usize + 1;
  let inc = value_to_distribute / size;

  let inc_bigger = inc + 1;
  let mut bigger_num = value_to_distribute % size;

  loop {
    if i >= size as usize {
      i = 0;
    }
    if bigger_num > 0 {
      bigger_num -= 1;
      some_vec[i] += inc_bigger;
    } else {
      some_vec[i] += inc;
    }
    if i == idx_to_distribute as usize {
      break;
    }
    
    i += 1;
  }

  // for x in &some_vec {
  //   print!("{} ", x);
  // }
  // println!("...");

  return some_vec;
}

pub fn main() {
  let mut vec: Vec<u32> = Vec::new();
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  for num_str in input.trim().split_whitespace() {
    vec.push(num_str.parse().unwrap());
  }
  let mut hash_map = HashMap::new();

  let mut count = 0;
  while !hash_map.contains_key(&vec) {
    count += 1;
    hash_map.insert(vec.clone(), 1);
    vec = get_next(&vec);
  }


  println!("{}", count);
}
