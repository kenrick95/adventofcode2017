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
    // Rotate 180deg
    // Rotate 270deg

    // Flip x + rotate 90
    // Flip x + rotate 180
    // Flip x + rotate 270

    // Flip y + rotate 90
    // Flip y + rotate 180
    // Flip y + rotate 270
  }
  println!("{}", rules.len());

  // Note: # --> 1
  //       . --> 0

  let mut current_map = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]];

  let max = 5;
  let mut iteration = 0;

  while iteration < max {
    if current_map.len() % 2 == 0 {
      // break into 2x2

      // get slice, match with rules, replace slice
    } else {
      // break into 3x3

      // get slice, match with rules, replace slice
    }
    iteration += 1;
  }

  let mut answer = 0;
  // sum current_map
  for row in current_map {
    for col in row {
      answer += col;
    }
  }

  println!("{}", answer);
}
