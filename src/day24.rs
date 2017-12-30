use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

struct Bridge {
  port_start: u32,
  port_end: u32
}

fn traverse(current_port: u32, used_index: &mut Vec<u32>) -> u32 {
  let mut max_value = 0;
  for (index, bridge); in bridges.iter().enumerate() {
    if bridge.port_start == current_port {
      used_index.push(index);
      let temp = traverse(bridge.port_end, used_index);
      used_index.pop();

      if temp > max_value {
        max_value = temp;
      }
    }
  }
  return max_value;
}

pub fn main() {
  let mut bridges = Vec::new();
  let mut input = String::new();

  let mut f = File::open("day24.in.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  for bridge_str in input.lines() {
    let splits = String::from(bridge_str).trim().split("/").collect();
    let lhs: u32 = String::from(splits[0]).parse().unwrap();
    let rhs: u32 = String::from(splits[1]).parse().unwrap();

    // Make sure lower value is on the left
    if lhs < rhs {
      bridges.push(Bridge {
        port_start: lhs,
        port_end: rhs,
      });
    } else {
      bridges.push(Bridge {
        port_start: rhs,
        port_end: lhs,
      });
    }
  }
  let mut used_index = vec![];
  let answer = traverse(0, &mut used_index);
  println!("{}", answer);

}
