use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub fn main() {
  let mut input = String::new();
  let mut map = vec![];

  let mut f = File::open("day22.in.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  let inputs: Vec<&str> = input.lines().collect();

  let input_len = inputs.len();

  let additional_col = 300;

  let mut temp = 0;
  while temp < additional_col {
    let mut row = vec![];
    for i in 0..additional_col {
      row.push(0);
    }
    for i in 0..input_len {
      row.push(0);
    }
    for i in 0..additional_col {
      row.push(0);
    }
    map.push(row);
    temp += 1;
  }

  // Parse input
  for input in inputs {
    let mut row = vec![];
    for i in 0..additional_col {
      row.push(0);
    }
    for cell in String::from(input).chars() {
      if cell == '.' { // clean
        row.push(0);
      } else if cell == '#' { // infected
        row.push(1);
      }
    }
    for i in 0..additional_col {
      row.push(0);
    }
    map.push(row);
  }
  temp = 0;
  while temp < additional_col {
    let mut row = vec![];
    for i in 0..additional_col {
      row.push(0);
    }
    for i in 0..input_len {
      row.push(0);
    }
    for i in 0..additional_col {
      row.push(0);
    }
    map.push(row);
    temp += 1;
  }

  // println!("{:?}", map);

  let mut current_x = map.len() / 2;
  let mut current_y = map.len() / 2;
  // println!("init {} {}", current_x, current_y);
  let mut dx: i32 = 0;
  let mut dy: i32 = -1;

  let mut answer = 0;
  let mut counter = 0;
  let max = 10000;
  while counter < max {
    let current_value = map[current_y][current_x];

    if current_value == 1 { // infected
      // turn right
      if dx == 1 && dy == 0 {
        dx = 0;
        dy = 1;
      } else if dx == -1 && dy == 0 {
        dx = 0;
        dy = -1;
      } else if dx == 0 && dy == 1 {
        dx = -1;
        dy = 0;
      } else if dx == 0 && dy == -1 {
        dx = 1;
        dy = 0;
      }
    } else { // clean
      // turn left
      if dx == 1 && dy == 0 {
        dx = 0;
        dy = -1;
      } else if dx == -1 && dy == 0 {
        dx = 0;
        dy = 1;
      } else if dx == 0 && dy == 1 {
        dx = 1;
        dy = 0;
      } else if dx == 0 && dy == -1 {
        dx = -1;
        dy = 0;
      }
    }

    if current_value == 1 {
      map[current_y][current_x] = 0;
    } else {
      map[current_y][current_x] = 1;
      answer += 1;
    }
    
    let next_x = current_x as i32 + dx;
    let next_y = current_y as i32 + dy;

    // println!("({}, {}), {}, ({} {}), {} {}, answer {}", current_x, current_y, current_value, next_x, next_y, dx, dy, answer);

    current_x = next_x as usize;
    current_y = next_y as usize;

    counter += 1;
  }
  
  println!("{}", answer);
}
