use std::io;
use std::collections::HashMap;

pub fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  let input_number: u32 = input.trim().parse().unwrap();

  let answer;
  let mut dx_offset: i32 = 1;
  let mut dy_offset: i32 = 1;
  let mut coord = (0, 0);
  let mut use_dx = true;
  let mut flag_counter = 0;
  let mut dict = HashMap::new();
  dict.insert(coord, 1);

  loop {
    let current_value: u32 = *(dict.get(&coord).unwrap());
    if current_value > input_number {
      answer = current_value;
      break;
    }

    flag_counter += 1;
    if use_dx {
      coord.0 = coord.0 + dx_offset.signum();
      
      if flag_counter >= dx_offset.abs() {
        flag_counter = 0;
        use_dx = !use_dx;
        dx_offset = -dx_offset.signum() * (dx_offset.abs() + 1);
      }
    } else {
      coord.1 = coord.1 + dy_offset.signum();
      
      if flag_counter >= dy_offset.abs() {
        flag_counter = 0;
        use_dx = !use_dx;
        dy_offset = -dy_offset.signum() * (dy_offset.abs() + 1);
      }
    }

    let mut new_value = 0;
    for &(dx, dy) in [(0, 1), (0, -1), (1, 1), (1, 0), (1, -1), (-1, 1), (-1, 0), (-1, -1)].iter() {
      let mut new_coord = (coord.0 + dx, coord.1 + dy);
      new_value += dict.get(&new_coord).unwrap_or(&0);
    }
    // println!("new_value {} {} {} {} {}", dx_offset, dy_offset, coord.0, coord.1, new_value);
    dict.insert(coord, new_value);

  }


  println!("{}", answer);
}
