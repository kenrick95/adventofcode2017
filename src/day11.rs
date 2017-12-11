use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn main() {
  let mut input = String::new();
  let mut hash_map = HashMap::new();

  let mut f = File::open("input.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  let steps = input.trim().split(",");

  for step in steps {
    let counter = hash_map.entry(step).or_insert(0);
    *counter += 1;
  }
  for (k, v) in hash_map.iter() {
    println!("{} {}", k, v);
  }

  // ne + s --> se
  // se + n --> ne
  // nw + s --> sw
  // sw + n --> nw
  // ne + nw --> n
  // sw + se --> s

  // s + n --> 0
  // se + nw --> 0
  // sw + ne --> 0

  
  let n_count = *hash_map.get("n").unwrap();
  let s_count = *hash_map.get("s").unwrap();
  if n_count > s_count {
    hash_map.insert("n", n_count - s_count);
    hash_map.insert("s", 0);
  } else {
    hash_map.insert("n", 0);
    hash_map.insert("s", s_count - n_count);
  }

  let ne_count = *hash_map.get("ne").unwrap();
  let sw_count = *hash_map.get("sw").unwrap();
  if ne_count > sw_count {
    hash_map.insert("ne", ne_count - sw_count);
    hash_map.insert("sw", 0);
  } else {
    hash_map.insert("ne", 0);
    hash_map.insert("sw", sw_count - ne_count);
  }

  let nw_count = *hash_map.get("nw").unwrap();
  let se_count = *hash_map.get("se").unwrap();
  if nw_count > se_count {
    hash_map.insert("nw", nw_count - se_count);
    hash_map.insert("se", 0);
  } else {
    hash_map.insert("nw", 0);
    hash_map.insert("se", se_count - nw_count);
  }

  println!("");

  for (k, v) in hash_map.iter() {
    println!("{} {}", k, v);
  }

  // s 43
  // ne 256
  // se 440
  // nw 0
  // n 0
  // sw 0
  // Manual analysis:
  // ne + s --> se
  // so
  // ne: 256 - 43
  // s: 0
  // se: 440 + 43
  // total: 696

}
