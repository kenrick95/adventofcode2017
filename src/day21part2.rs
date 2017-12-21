use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn flip_x(map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let mut new_map = vec![];
  for row in map.iter().rev() {
    new_map.push(row.clone());
  }
  return new_map;
}

fn flip_y(map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let mut new_map = vec![];
  for row in map {
    let mut new_row = vec![];
    for cell in row.iter().rev() {
      new_row.push(*cell);
    }
    new_map.push(new_row);
  }
  return new_map;
}

fn get_empty_map(map_len: usize) -> Vec<Vec<u8>> {
  let mut new_map = vec![];

  let mut i = 0;
  while i < map_len {
    let mut j = 0;
    let mut map_row = vec![];
    while j < map_len {
      map_row.push(0);
      j += 1;
    }
    new_map.push(map_row);
    i += 1;
  }
  return new_map;
}

fn rotate90(map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let map_len = map.len();
  let mut new_map = get_empty_map(map_len);

  let mut i = 0;
  while i < map_len {
    let mut j = 0;
    while j < map_len {
      new_map[i][j] = map[map_len - j - 1][i];
      j += 1;
    }
    i += 1;
  }

  return new_map;
}

fn rotate180(map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let map_len = map.len();
  let mut new_map = get_empty_map(map_len);

  let mut i = 0;
  while i < map_len {
    let mut j = 0;
    while j < map_len {
      new_map[i][j] = map[map_len - i - 1][map_len - j - 1];
      j += 1;
    }
    i += 1;
  }

  return new_map;
}

fn rotate270(map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let map_len = map.len();
  let mut new_map = get_empty_map(map_len);

  let mut i = 0;
  while i < map_len {
    let mut j = 0;
    while j < map_len {
      new_map[i][j] = map[j][map_len - i - 1];
      j += 1;
    }
    i += 1;
  }

  return new_map;
}

pub fn main() {
  let mut input = String::new();
  let mut rules = HashMap::new();

  let mut f = File::open("day21.in.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  let inputs: Vec<&str> = input.lines().collect();

  // Parse input
  for input in inputs {
    // For each rule, put into `rules`
    let input_string = String::from(input).clone();
    let splits: Vec<&str> = input_string.trim().split(" => ").collect();
    let lhs = String::from(splits[0]);
    let rhs = String::from(splits[1]);

    let mut rule_match = vec![];
    let mut rule_result = vec![];

    let lhs_splits: Vec<&str> = lhs.split("/").collect();
    let rhs_splits: Vec<&str> = rhs.split("/").collect();

    for split in lhs_splits {
      let mut rule_match_row = vec![];
      let split_string = String::from(split);
      for ch in split_string.chars() {
        if ch == '.' {
          rule_match_row.push(0);
        } else if ch == '#' {
          rule_match_row.push(1);
        }
      }
      rule_match.push(rule_match_row);
    }

    for split in rhs_splits {
      let mut rule_result_row = vec![];
      let split_string = String::from(split);
      for ch in split_string.chars() {
        if ch == '.' {
          rule_result_row.push(0);
        } else if ch == '#' {
          rule_result_row.push(1);
        }
      }
      rule_result.push(rule_result_row);
    }
    rules.insert(rule_match.clone(), rule_result.clone());

    // TODO: rotate & flip "rule_match", put into "rules"
    // Flip x-axis
    rules.insert(flip_x(rule_match.clone()), rule_result.clone());
    // Flip y-axis
    rules.insert(flip_y(rule_match.clone()), rule_result.clone());

    // Rotate 90deg
    rules.insert(rotate90(rule_match.clone()), rule_result.clone());
    // Rotate 180deg
    rules.insert(rotate180(rule_match.clone()), rule_result.clone());
    // Rotate 270deg
    rules.insert(rotate270(rule_match.clone()), rule_result.clone());

    // Flip x + rotate 90
    rules.insert(flip_x(rotate90(rule_match.clone())), rule_result.clone());
    rules.insert(rotate90(flip_x(rule_match.clone())), rule_result.clone());
    // Flip x + rotate 180
    rules.insert(flip_x(rotate180(rule_match.clone())), rule_result.clone());
    rules.insert(rotate180(flip_x(rule_match.clone())), rule_result.clone());
    // Flip x + rotate 270
    rules.insert(flip_x(rotate270(rule_match.clone())), rule_result.clone());
    rules.insert(rotate270(flip_x(rule_match.clone())), rule_result.clone());

    // Flip y + rotate 90
    rules.insert(flip_y(rotate90(rule_match.clone())), rule_result.clone());
    rules.insert(rotate90(flip_y(rule_match.clone())), rule_result.clone());
    // Flip y + rotate 180
    rules.insert(flip_y(rotate180(rule_match.clone())), rule_result.clone());
    rules.insert(rotate180(flip_y(rule_match.clone())), rule_result.clone());
    // Flip y + rotate 270
    rules.insert(flip_y(rotate270(rule_match.clone())), rule_result.clone());
    rules.insert(rotate270(flip_y(rule_match.clone())), rule_result.clone());
  }
  println!("{}", rules.len());

  // Note: # --> 1
  //       . --> 0

  let mut current_map = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]];

  let max = 18;
  let mut iteration = 0;

  while iteration < max {
    let mut mutated_slices = vec![];

    let current_map_len = current_map.len();

    if current_map_len % 2 == 0 {
      let num_slices_in_row = current_map_len / 2;
      // break into 2x2 --> 3x3
      let mut i = 0;
      let mut j;
      while i < num_slices_in_row {
        j = 0;
        while j < num_slices_in_row {
          let slice = vec![
            vec![current_map[i * 2][j * 2], current_map[i * 2][j * 2 + 1]],
            vec![
              current_map[i * 2 + 1][j * 2],
              current_map[i * 2 + 1][j * 2 + 1],
            ],
          ];
          let mutated_slice = match rules.get(&slice) {
            Some(val) => val.clone(),
            None => slice,
          };
          mutated_slices.push(mutated_slice);

          j += 1;
        }
        i += 1;
      }

      // println!("{:?}", mutated_slices);
      let mut new_map = get_empty_map(num_slices_in_row * 3);

      // get slice, match with rules, replace slice
      for (index, slice) in mutated_slices.iter().enumerate() {
        let base_row = (index / num_slices_in_row) * 3;
        let base_col = (index % num_slices_in_row) * 3;

        let mut i = 0;
        for row in slice {
          let mut j = 0;
          for cell in row {
            new_map[base_row + i][base_col + j] = *cell;
            j += 1;
          }
          i += 1;
        }
      }

      current_map = new_map;
    } else {
      let num_slices_in_row = current_map_len / 3;
      // break into 3x3 --> 4x4
      let mut i = 0;
      let mut j;
      while i < num_slices_in_row {
        j = 0;
        while j < num_slices_in_row {
          let slice = vec![
            vec![
              current_map[i * 3][j * 3],
              current_map[i * 3][j * 3 + 1],
              current_map[i * 3][j * 3 + 2],
            ],
            vec![
              current_map[i * 3 + 1][j * 3],
              current_map[i * 3 + 1][j * 3 + 1],
              current_map[i * 3 + 1][j * 3 + 2],
            ],
            vec![
              current_map[i * 3 + 2][j * 3],
              current_map[i * 3 + 2][j * 3 + 1],
              current_map[i * 3 + 2][j * 3 + 2],
            ],
          ];
          // println!("slice: {:?}", slice);
          let mutated_slice = match rules.get(&slice) {
            Some(val) => val.clone(),
            None => slice,
          };
          // println!("mutated_slice: {:?}", mutated_slice);
          mutated_slices.push(mutated_slice);

          j += 1;
        }
        i += 1;
      }

      let mut new_map = get_empty_map(num_slices_in_row * 4);

      // println!("{:?}", mutated_slices);
      // get slice, match with rules, replace slice
      for (index, slice) in mutated_slices.iter().enumerate() {
        let base_row = (index / num_slices_in_row) * 4;
        let base_col = (index % num_slices_in_row) * 4;
        let mut i = 0;
        for row in slice {
          let mut j = 0;
          for cell in row {
            new_map[base_row + i][base_col + j] = *cell;
            j += 1;
          }
          i += 1;
        }
      }

      current_map = new_map;
    }
    // println!("iteration {}, {:?}", iteration, current_map);
    iteration += 1;
  }

  let mut answer: u64 = 0;
  // sum current_map
  for row in current_map {
    for col in row {
      answer += col as u64;
    }
  }

  println!("{}", answer);
}
