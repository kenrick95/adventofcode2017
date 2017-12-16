use std::io;
use std::collections::HashMap;

fn hash(input: String) -> Vec<u32> {
  let mut current_position = 0;
  let mut current_skip = 0;
  let mut vec: Vec<u32> = Vec::new();
  let mut number = 0;
  while number < 256 {
    vec.push(number);
    number += 1;
  }

  let mut round = 0;
  while round < 64 {
    let mut lengths = input.trim().as_bytes().to_vec();
    lengths.push(17);
    lengths.push(31);
    lengths.push(73);
    lengths.push(47);
    lengths.push(23);

    for length in lengths {
      let mut collected = 0;
      let mut temp_position = current_position;

      // Collect vec to reverse
      let mut reversed_vec = Vec::new();
      while collected < length {
        let vec_value = vec[temp_position as usize];

        reversed_vec.push(vec_value);

        temp_position += 1;
        collected += 1;

        if temp_position >= vec.len() as u32 {
          temp_position = 0;
        }
      }
      reversed_vec.reverse();

      // Apply vec
      let mut reversed_vec_iter = reversed_vec.into_iter();
      let mut applied = 0;
      temp_position = current_position;
      while applied < length {
        vec[temp_position as usize] = reversed_vec_iter.next().unwrap();

        temp_position += 1;
        applied += 1;

        if temp_position >= vec.len() as u32 {
          temp_position = 0;
        }
      }

      current_position = (current_position + length as u32 + current_skip) % vec.len() as u32;
      current_skip += 1;
    }


    round += 1;
  }

  let mut reduced_vec = Vec::new();
  for chunk in vec.chunks(16) {
    let mut xor_result = 0;
    for elem in chunk {
      xor_result ^= *elem;
    }
    reduced_vec.push(xor_result);
  }

  return reduced_vec;
}

fn ff(
  i: i32,
  j: i32,
  last_max: i32,
  map: &HashMap<(u32, u32), u8>,
  colored_map: &mut HashMap<(u32, u32), i32>,
) -> bool {
  // amgozmfv
  if i < 0 || i >= 128 || j < 0 || j >= 128 {
    return false;
  }
  // println!("i {}, j {}", i, j);
  let current_position = (i as u32, j as u32);
  let current_bit = *map.get(&current_position).unwrap();
  // println!("bit {}", current_bit);
  let current_color = *colored_map.get(&current_position).unwrap();
  // println!("color {}", current_color);
  if current_color != -1 || current_bit == 0 {
    return false;
  }
  colored_map.insert(current_position, last_max);
  ff(i - 1, j, last_max, map, colored_map);
  ff(i + 1, j, last_max, map, colored_map);
  ff(i, j - 1, last_max, map, colored_map);
  ff(i, j + 1, last_max, map, colored_map);

  return true;
}

pub fn main() {
  let mut map = HashMap::new();

  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  let mut answer = 0;
  let mut row_counter = 0;
  while row_counter < 128 {
    let row_input = String::from(input.trim().clone()) + "-" + &row_counter.to_string().clone();
    // println!("{}", row_input);
    let reduced_vec = hash(row_input);
    let mut row_bits = Vec::new();
    // for elem in reduced_vec.clone() {
    //   print!("{:02x}", elem);
    // }
    // println!("");

    for elem in reduced_vec {
      let hex_str = format!("{:02x}", elem);
      for char_byte in hex_str.into_bytes() {
        let mut byte_bits = Vec::new();
        let byte;
        if char_byte <= 57 {
          // 0-9
          byte = char_byte - 48;
        } else {
          // a-f
          byte = char_byte - 97 + 10;
        }
        let mut thing = 0;
        while thing < 4 {
          let bit = (byte >> thing) & 1;
          // print!("{}", bit);
          thing += 1;
          byte_bits.push(bit);
        }
        byte_bits.reverse();
        for bit in byte_bits {
          row_bits.push(bit);
        }
        // print!(";{}, ", byte);
      }
    }
    let mut column_index = 0;
    for bit in row_bits {
      // print!("{}", bit);
      map.insert((row_counter, column_index), bit);

      column_index += 1;
    }

    // println!("");
    row_counter += 1;
  }

  // flood fill, find how many groups
  let mut i: u32 = 0;
  let mut j: u32;
  let mut colored_map = HashMap::new();

  while i < 128 {
    j = 0;
    while j < 128 {
      colored_map.insert((i, j), -1);
      j += 1;
    }
    i += 1;
  }

  i = 0;
  let mut last_max = 0;
  while i < 128 {
    j = 0;
    while j < 128 {
      let res = ff(i as i32, j as i32, last_max, &map, &mut colored_map);
      if res {
        answer += 1;
        last_max += 1;
      }
      j += 1;
    }
    i += 1;
  }


  println!("{}", answer);
}
