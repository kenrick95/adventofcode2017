use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::cmp;

fn reduce(hash_map: &mut HashMap<&str, i32>) -> i32 {
  // s + n --> 0
  // se + nw --> 0
  // sw + ne --> 0

  {
    let n_count = *hash_map.get("n").unwrap();
    let s_count = *hash_map.get("s").unwrap();
    hash_map.insert("n", cmp::max(n_count - s_count, 0));
    hash_map.insert("s", cmp::max(s_count - n_count, 0));
  }
  {
    let ne_count = *hash_map.get("ne").unwrap();
    let sw_count = *hash_map.get("sw").unwrap();
    hash_map.insert("ne", cmp::max(ne_count - sw_count, 0));
    hash_map.insert("sw", cmp::max(sw_count - ne_count, 0));
  }
  {
    let nw_count = *hash_map.get("nw").unwrap();
    let se_count = *hash_map.get("se").unwrap();
    hash_map.insert("nw", cmp::max(nw_count - se_count, 0));
    hash_map.insert("se", cmp::max(se_count - nw_count, 0));
  }
  // println!("1");

  // for (k, v) in hash_map.iter() {
  //   println!("{} {}", k, v);
  // }


  // ne + s --> se
  {
    let ne_count = *hash_map.get("ne").unwrap();
    let s_count = *hash_map.get("s").unwrap();
    let se_count = *hash_map.get("se").unwrap();
    let common_count = cmp::min(ne_count, s_count);
    hash_map.insert("ne", ne_count - common_count);
    hash_map.insert("s", s_count - common_count);
    hash_map.insert("se", se_count + common_count);
  }
  // println!("2");

  // for (k, v) in hash_map.iter() {
  //   println!("{} {}", k, v);
  // }

  // nw + s --> sw
  {
    let nw_count = *hash_map.get("nw").unwrap();
    let s_count = *hash_map.get("s").unwrap();
    let sw_count = *hash_map.get("sw").unwrap();
    let common_count = cmp::min(nw_count, s_count);
    hash_map.insert("nw", nw_count - common_count);
    hash_map.insert("s", s_count - common_count);
    hash_map.insert("sw", sw_count + common_count);
  }
  // println!("3");

  // for (k, v) in hash_map.iter() {
  //   println!("{} {}", k, v);
  // }

  // se + n --> ne
  {
    let se_count = *hash_map.get("se").unwrap();
    let n_count = *hash_map.get("n").unwrap();
    let ne_count = *hash_map.get("ne").unwrap();
    let common_count = cmp::min(se_count, n_count);
    hash_map.insert("se", se_count - common_count);
    hash_map.insert("n", n_count - common_count);
    hash_map.insert("ne", ne_count + common_count);
  }
  // println!("4");

  // for (k, v) in hash_map.iter() {
  //   println!("{} {}", k, v);
  // }


  // sw + n --> nw
  {
    let sw_count = *hash_map.get("sw").unwrap();
    let n_count = *hash_map.get("n").unwrap();
    let nw_count = *hash_map.get("nw").unwrap();
    let common_count = cmp::min(sw_count, n_count);
    hash_map.insert("sw", sw_count - common_count);
    hash_map.insert("n", n_count - common_count);
    hash_map.insert("nw", nw_count + common_count);
  }
  // println!("5");

  // for (k, v) in hash_map.iter() {
  //   println!("{} {}", k, v);
  // }

  // sw + se --> s
  {
    let se_count = *hash_map.get("se").unwrap();
    let sw_count = *hash_map.get("sw").unwrap();
    let s_count = *hash_map.get("s").unwrap();
    let common_count = cmp::min(se_count, sw_count);
    hash_map.insert("se", se_count - common_count);
    hash_map.insert("sw", sw_count - common_count);
    hash_map.insert("s", s_count + common_count);
  }
  // println!("6");

  // for (k, v) in hash_map.iter() {
  //   println!("{} {}", k, v);
  // }

  // ne + nw --> n
  {
    let ne_count = *hash_map.get("ne").unwrap();
    let nw_count = *hash_map.get("nw").unwrap();
    let n_count = *hash_map.get("n").unwrap();
    let common_count = cmp::min(ne_count, nw_count);
    hash_map.insert("ne", ne_count - common_count);
    hash_map.insert("nw", nw_count - common_count);
    hash_map.insert("n", n_count + common_count);
  }

  // println!("7");

  let mut total_value = 0;
  for (k, v) in hash_map.iter() {
    // println!("{} {}", k, v);
    total_value += v;
  }
  // println!("{}", total_value);
  return total_value;
}

pub fn main() {
  let mut input = String::new();
  let mut hash_map = HashMap::new();
  hash_map.insert("nw", 0);
  hash_map.insert("ne", 0);
  hash_map.insert("n", 0);
  hash_map.insert("sw", 0);
  hash_map.insert("se", 0);
  hash_map.insert("s", 0);

  let mut f = File::open("input.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  let steps = input.trim().split(",");

  let mut answer = 0;

  for step in steps {
    {
      let counter = hash_map.entry(step).or_insert(0);
      *counter += 1;
    }

    answer = cmp::max(answer, reduce(&mut hash_map));
  }

  println!("{}", answer);
}
