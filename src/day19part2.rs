use std::fs::File;
use std::io::Read;

fn is_safe(row: i32, col: i32, map: &Vec<Vec<char>>) -> bool {
  return row >= 0 && row < map.len() as i32 && col >= 0 && col <= map[0].len() as i32
    && map[row as usize][col as usize] != ' ';
}

pub fn main() {
  let mut input = String::new();
  let mut collected: Vec<char> = Vec::new();

  let mut f = File::open("day19.in.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  let inputs: Vec<&str> = input.lines().collect();

  let mut map = Vec::new();
  for line in inputs {
    let mut line_vec = Vec::new();
    for ch in String::from(line).chars() {
      line_vec.push(ch);
    }
    map.push(line_vec);
  }

  // Find starting point
  let mut row = 0;
  let mut col = 0;
  for (index, content) in map[0].iter().enumerate() {
    if *content != ' ' {
      col = index;
      break;
    }
  }

  let mut dx = 0;
  let mut dy = 1;
  let mut counter = 1; // for initial "step" from outside to first cell

  loop {
    let val = map[row][col];
    // println!("{} {}, {}", row, col, val);
    if val != ' ' && val != '|' && val != '+' && val != '-' {
      collected.push(val);
    }

    let hrow = row as i32;
    let hcol = col as i32;

    let mut next_row = hrow + dy;
    let mut next_col = hcol + dx;

    // if not stuck, continue the direction
    if !is_safe(next_row, next_col, &map) {
      if is_safe(hrow + 1, hcol, &map) && dy != -1 {
        // println!("down");
        dx = 0;
        dy = 1;
        next_row = hrow + 1;
        next_col = hcol;
      } else if is_safe(hrow, hcol - 1, &map) && dx != 1 {
        // println!("left");
        dx = -1;
        dy = 0;
        next_row = hrow;
        next_col = hcol - 1;
      } else if is_safe(hrow, hcol + 1, &map) && dx != -1 {
        // println!("right");
        dx = 1;
        dy = 0;
        next_row = hrow;
        next_col = hcol + 1;
      } else if is_safe(hrow - 1, hcol, &map) && dy != 1 {
        // println!("up");
        dx = 0;
        dy = -1;
        next_row = hrow - 1;
        next_col = hcol;
      } else {
        println!("...");
        break;
      }
    }

    counter += 1;

    row = next_row as usize;
    col = next_col as usize;
  }

  for letter in collected {
    print!("{}", letter);
  }
  println!("");

  println!("counter {}", counter);
}
