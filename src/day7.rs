use std::io;
use std::collections::HashMap;

// need hashmap<entry_name, Entry>
struct Entry {
  value: u32,
  children_names: Vec<String>,
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

  let mut something = String::new();

  for input in inputs {
    let splits: Vec<&str> = input.trim().split("->").collect();

    // Parse LHS, get node name & value
    let lhs = String::from(splits[0]);
    let things: Vec<&str> = lhs.split(" (").collect();
    let name = String::from(things[0]);
    let value = things[1].trim().trim_right_matches(")").parse().unwrap();
    let mut children_names = Vec::new();

    if something == "" {
      something = name.clone();
    }

    print!("name {}, value {}, ", name, value);

    if splits.len() > 1 {
      // Parse RHS, children names
      let rhs = splits[1];
      for name in rhs.split(",") {
        children_names.push(String::from(name.trim()));
      }
    }

    print!("children names: ");
    for child_name in &children_names {
      print!("{}, ", child_name);
      parent_map.insert(child_name.clone(), name.clone());
    }
    println!("..");

    let entry = Entry {
      value: value,
      children_names: children_names,
    };

    hash_map.insert(name, entry);
  }

  // find root: start randomly, traverse parent_name to top
  while parent_map.contains_key(&something) {
    something = parent_map.get(&something).unwrap().clone();
  }


  println!("{}", something);
}
