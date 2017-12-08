use std::io;
use std::collections::HashMap;

// need hashmap<entry_name, Entry>
struct Entry {
  value: u32,
  children_names: Vec<String>,
}

fn get_value(node: String, hash_map: &HashMap<String, Entry>) -> u32 {
  let entry = hash_map.get(&node).unwrap();
  let mut children_values: Vec<u32> = Vec::new();
  for child in &entry.children_names {
    children_values.push(get_value(child.clone(), hash_map));
  }
  let mut children_sum = 0;
  print!("at node {}: ", node);
  for val in &children_values {
    children_sum += *val;
    print!("{}, ", *val);
  }
  println!("....");

  // The first time we encounter an odd children, then set something?

  if children_values.len() > 1 {
    children_values.sort();
    if children_values[0] != children_values[children_values.len() - 1] {
      // something is odd
      println!("Something is odd!");
      // TODO: Go to the "odd" child, get it's value, get it's children values, do some math
      // ... (oh well, it's pretty difficult, so I manually check it and got AC, lol)
    }
  }

  return entry.value + children_sum;
}

pub fn main() {
  let mut hash_map: HashMap<String, Entry> = HashMap::new();
  let mut parent_map: HashMap<String, String> = HashMap::new();
  let mut inputs = Vec::new();

  loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "END" {
      break;
    }
    inputs.push(input);
  }

  let mut root = String::new();

  for input in inputs {
    let splits: Vec<&str> = input.trim().split("->").collect();

    // Parse LHS, get node name & value
    let lhs = String::from(splits[0]);
    let things: Vec<&str> = lhs.split(" (").collect();
    let name = String::from(things[0]);
    let value = things[1].trim().trim_right_matches(")").parse().unwrap();
    let mut children_names = Vec::new();

    if root == "" {
      root = name.clone();
    }

    // print!("name {}, value {}, ", name, value);

    if splits.len() > 1 {
      // Parse RHS, children names
      let rhs = splits[1];
      for name in rhs.split(",") {
        children_names.push(String::from(name.trim()));
      }
    }

    // print!("children names: ");
    for child_name in &children_names {
      // print!("{}, ", child_name);
      parent_map.insert(child_name.clone(), name.clone());
    }
    // println!("..");

    let entry = Entry {
      value: value,
      children_names: children_names,
    };

    hash_map.insert(name, entry);
  }

  // find root: start randomly, traverse parent_name to top
  while parent_map.contains_key(&root) {
    root = parent_map.get(&root).unwrap().clone();
  }
  println!("root {}", root);

  // get values of each children, traversing from root
  get_value(root.clone(), &hash_map);
  // then at the deepest level with odd subtree, answer = its siblings' value
}
