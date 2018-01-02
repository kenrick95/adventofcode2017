use std::fs::File;
use std::io::Read;

struct Bridge {
  port_start: u32,
  port_end: u32
}

fn traverse(current_port: u32, used_index: &mut Vec<usize>, bridges: &Vec<Bridge>) -> (u32, usize) {
  let mut max_value = (0, 0);
  // println!("{:?}, {:?}", current_port, used_index);
  for (index, bridge) in bridges.iter().enumerate() {
    if !used_index.contains(&index) {
      let mut temp;
      if bridge.port_start == current_port {
        used_index.push(index);
        temp = traverse(bridge.port_end, used_index, bridges);
        temp.0 += bridge.port_start;
        used_index.pop();
      } else if bridge.port_end == current_port {
        used_index.push(index);
        temp = traverse(bridge.port_start, used_index, bridges);
        temp.0 += bridge.port_end;
        used_index.pop();
      } else {
        temp = (0, 0);
      }

      if temp.1 > max_value.1 {
        max_value = temp;
      } else if temp.1 == max_value.1 && temp.0 > max_value.0 {
        max_value = temp;
      }
    }
  }
  return (max_value.0 + current_port, max_value.1 + 1);
}

pub fn main() {
  let mut bridges = Vec::new();
  let mut input = String::new();

  let mut f = File::open("day24.in.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  for bridge_str in input.lines() {
    let bridge_string = String::from(bridge_str).clone();
    let splits: Vec<&str> = bridge_string.trim().split("/").collect();
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
  let answer = traverse(0, &mut used_index, &bridges);
  println!("{:?}", answer);

}
