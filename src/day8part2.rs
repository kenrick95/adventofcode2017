use regex::Regex;
use std::io;
use std::collections::HashMap;

pub fn main() {
  let mut hash_map: HashMap<String, i32> = HashMap::new();
  let re = Regex::new(r"(.+?) (inc|dec) ([-0-9]+) if (.+?) (>=|>|!=|==|<=|<) ([-0-9]+)").unwrap();
  let mut max_value = 0;

  loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "END" {
      break;
    }

    let caps = re.captures(&input).unwrap();

    let reg = String::from(caps.get(1).unwrap().as_str());
    let operation = String::from(caps.get(2).unwrap().as_str());
    let operation_amount: i32 = String::from(caps.get(3).unwrap().as_str()).parse().unwrap();
    let cmp_reg = String::from(caps.get(4).unwrap().as_str());
    let cmp_operator = String::from(caps.get(5).unwrap().as_str());
    let cmp_value: i32 = String::from(caps.get(6).unwrap().as_str()).parse().unwrap();

    println!(
      "{} {} {} {} {} {}",
      reg,
      operation,
      operation_amount,
      cmp_reg,
      cmp_operator,
      cmp_value
    );

    if !hash_map.contains_key(&reg) {
      hash_map.insert(reg.clone(), 0);
    }
    if !hash_map.contains_key(&cmp_reg) {
      hash_map.insert(cmp_reg.clone(), 0);
    }

    let mut final_delta: i32 = 0;
    let mut do_operation = false;
    let reg_value = *hash_map.get(&reg).unwrap();
    let cmp_reg_value = *hash_map.get(&cmp_reg).unwrap();

    if cmp_operator == "==" {
      if cmp_reg_value == cmp_value {
        do_operation = true;
      }
    } else if cmp_operator == ">=" {
      if cmp_reg_value >= cmp_value {
        do_operation = true;
      }
    } else if cmp_operator == ">" {
      if cmp_reg_value > cmp_value {
        do_operation = true;
      }
    } else if cmp_operator == "<" {
      if cmp_reg_value < cmp_value {
        do_operation = true;
      }
    } else if cmp_operator == "<=" {
      if cmp_reg_value <= cmp_value {
        do_operation = true;
      }
    } else if cmp_operator == "!=" {
      if cmp_reg_value != cmp_value {
        do_operation = true;
      }
    } else {
      println!("Impossible operator: {}", cmp_operator);
    }

    if do_operation {
      if operation == "inc" {
        final_delta = operation_amount;
      } else if operation == "dec" {
        final_delta = -operation_amount;
      } else {
        println!("Impossible operation: {}", operation);
      }
      hash_map.insert(reg.clone(), reg_value + final_delta);

      if reg_value + final_delta >= max_value {
        max_value = reg_value + final_delta;
      }
    }
  }

  println!("{}", max_value);
}
