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

  let mut answer = -1;
  loop {
    answer += 1;

    let mut can = true;

    let mut time: i32 = -1;
    while time - answer + 1 < layer_depth.len() as i32 {
      time += 1;

      let packet_position = time - answer;
      // println!("{} {} {}", time, answer, packet_position);
      if packet_position < 0 {
        continue;
      }

      let current_layer_depth = *layer_depth.get(&(packet_position as u32)).unwrap();
      let mut current_layer_fire_position = -1;
      if current_layer_depth > 0 {
        current_layer_fire_position = time % ((current_layer_depth as i32 - 1) * 2);
        // let len = (current_layer_depth as i32) * 2 - 2;
        // // println!("len {}", len);
        // current_layer_fire_position = time % len;
        // if current_layer_fire_position >= (current_layer_depth as i32) {
        //   current_layer_fire_position = len - current_layer_fire_position;
        // }

        // // 4
        // // len = current_layer_depth * 2 - 2 = 6
        // //

        // // 0 1 2 3 2 1 | 0 1 2 3 2 1
        // // 0 1 2 3 4 5
      }
      // println!("{} {} {}", packet_position, current_layer_depth, current_layer_fire_position);

      if current_layer_fire_position == 0 {
        // println!(
        //   "{} {} {} {}",
        //   time,
        //   answer,
        //   packet_position,
        //   current_layer_fire_position
        // );
        // caught;
        can = false;
        break;
      }
    }
    if answer % 1000 == 0 {
      println!("{} {} {}", time, answer, can);
    }
    
    if can {
      break;
    }
  }

  println!("{}", answer);
}
