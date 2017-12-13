use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn main() {
  let mut layer_depth = HashMap::new();
  let mut input = String::new();

  let mut f = File::open("day13.in.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  let mut i = 0;
  while i < 100 {
    layer_depth.insert(i, 0);
    i += 1;
  }

  for line in input.lines() {
    let pieces: Vec<&str> = line.split(": ").collect();
    let lhs = String::from(pieces[0]);

    let layer: u32 = lhs.trim().parse().unwrap();

    let rhs = String::from(pieces[1]);

    let depth: u32 = rhs.trim().parse().unwrap();

    layer_depth.insert(layer, depth);
  }

  let mut answer = 0;
  let mut time: i32 = -1;
  while time + 1 < layer_depth.len() as i32 {
    time += 1;

    let packet_position = time as u32;

    let current_layer_depth = *layer_depth.get(&packet_position).unwrap();
    let mut current_layer_fire_position = -1;
    if current_layer_depth > 0 {
      let len = (current_layer_depth as i32) * 2 - 2;
      // println!("len {}", len);
      current_layer_fire_position = time % len;
      if current_layer_fire_position >= (current_layer_depth as i32) {
        current_layer_fire_position = len - current_layer_fire_position;
      }

      // 4
      // len = current_layer_depth * 2 - 2 = 6
      // 
      
      // 0 1 2 3 2 1 | 0 1 2 3 2 1
      // 0 1 2 3 4 5

    }
    // println!("{} {} {}", packet_position, current_layer_depth, current_layer_fire_position);

    if current_layer_fire_position == 0 {
      answer += packet_position * current_layer_depth;
    }
  }

  println!("{}", answer);
}
