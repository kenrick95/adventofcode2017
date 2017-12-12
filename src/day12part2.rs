use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn find(x: u32, parent: &mut HashMap<u32, u32>, rank: &mut HashMap<u32, u32>) -> u32 {
  let mut par_value = *parent.get(&x).unwrap();
  if par_value != x {
    par_value = find(par_value, parent, rank);
    parent.insert(x, par_value);
  }
  return par_value;
}

fn make_set(x: u32, parent: &mut HashMap<u32, u32>, rank: &mut HashMap<u32, u32>) {
  if !parent.contains_key(&x) {
    parent.insert(x, x);
    rank.insert(x, 0);
  }
}

fn union(x: u32, y: u32, parent: &mut HashMap<u32, u32>, rank: &mut HashMap<u32, u32>) {
  let x_root = find(x, parent, rank);
  let y_root = find(y, parent, rank);

  if x_root == y_root {
    return;
  }

  let x_root_rank = *rank.get(&x_root).unwrap();
  let y_root_rank = *rank.get(&y_root).unwrap();

  if x_root_rank < y_root_rank {
    parent.insert(x_root, y_root);
  } else if x_root_rank > y_root_rank {
    parent.insert(y_root, x_root);
  } else {
    parent.insert(y_root, x_root);
    rank.insert(x_root, x_root_rank + 1);
  }
}

pub fn main() {
  let mut parent = HashMap::new();
  let mut rank = HashMap::new();
  let mut input = String::new();

  let mut f = File::open("day12.in.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  let mut i = 0;
  while i < 2000 {
    make_set(i, &mut parent, &mut rank);
    i += 1;
  }

  for line in input.lines() {
    let pieces: Vec<&str> = line.split("<->").collect();
    let lhs = String::from(pieces[0]);

    let head: u32 = lhs.trim().parse().unwrap();

    let rhs = String::from(pieces[1]);

    let children: Vec<&str> = rhs.split(",").collect();

    for child_str in children {
      let child: u32 = child_str.trim().parse().unwrap();
      union(head, child, &mut parent, &mut rank);
    }
  }

  let mut other_parent = parent.clone();
  let mut set = HashSet::new();

  for key in parent.keys() {
    let root = find(*key, &mut other_parent, &mut rank);
    set.insert(root);
  }

  println!("{}", set.len());
}
