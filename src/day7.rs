use std::io;
use std::collections::HashMap;

// need hashmap<entry_name, Entry>
struct Entry {
  value: u32,
  parent_name: String,
  children_names: Vec<String>,
}

pub fn main() {
  let mut hash_map: HashMap<String, Entry> = HashMap::new();
  loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let splits: Vec<&str> = input.split("->").collect();

    // Parse LHS, get node name & value
    let lhs = splits[0];
  

    if splits.len() > 0 {

        // Parse RHS, children names
        let rhs = splits[1];
        rhs.split(",");
    }

  }

  // find root: start randomly, traverse parent_name to top

  println!("{}", 0);
}
