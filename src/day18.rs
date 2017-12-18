use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn get_from_register(register: &String, registers: &mut HashMap<String, i64>) -> i64 {
  return match registers.get(register) {
    Some(val) => *val,
    None => 0,
  };
}

fn get_register_or_value(register_or_value: String, registers: &mut HashMap<String, i64>) -> i64 {
  let num = register_or_value.parse::<i64>();
  match num {
    Ok(val) => {
      return val;
    }
    Err(_why) => {
      return get_from_register(&register_or_value, registers);
    }
  }
}

fn op_set(a: String, b: String, registers: &mut HashMap<String, i64>) {
  // "a" is register
  // "b" could be register, could be value
  let b_num = get_register_or_value(b, registers);
  registers.insert(a, b_num);
}

fn op_add(a: String, b: String, registers: &mut HashMap<String, i64>) {
  let a_num = get_from_register(&a, registers);
  let b_num = get_register_or_value(b, registers);
  registers.insert(a, a_num + b_num);
}

fn op_mul(a: String, b: String, registers: &mut HashMap<String, i64>) {
  let a_num = get_from_register(&a, registers);
  let b_num = get_register_or_value(b, registers);
  registers.insert(a, a_num * b_num);
}

fn op_mod(a: String, b: String, registers: &mut HashMap<String, i64>) {
  let a_num = get_from_register(&a, registers);
  let b_num = get_register_or_value(b, registers);
  registers.insert(a, a_num % b_num);
}


fn op_snd(a: String, registers: &mut HashMap<String, i64>, history: &mut Vec<i64>) {
  let a_num = get_from_register(&a, registers);
  history.push(a_num);
}

fn op_rcv(a: String, registers: &mut HashMap<String, i64>, history: &mut Vec<i64>) -> bool {
  let a_num = get_from_register(&a, registers);
  if a_num > 0 {
    println!("hohoho {}", history[history.len() - 1]);
    return true;
  }
  return false;
}


pub fn main() {
  let mut input = String::new();
  let mut registers = HashMap::new();
  let mut history = Vec::new();

  let mut f = File::open("day18.in.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  let instructions: Vec<&str> = input.lines().collect();

  let mut current_index: i64 = 0;
  let instruction_len = instructions.len();

  while current_index >= 0 && current_index < instruction_len as i64 {
    let instruction = instructions[current_index as usize];

    // println!("{} {}", current_index, instruction);

    // parse
    let chunks: Vec<&str> = instruction.trim().split(" ").collect();

    let operator = String::from(chunks[0]);

    if operator == "snd" {
      op_snd(String::from(chunks[1]), &mut registers, &mut history);
    } else if operator == "set" {
      op_set(
        String::from(chunks[1]),
        String::from(chunks[2]),
        &mut registers,
      );
    } else if operator == "add" {
      op_add(
        String::from(chunks[1]),
        String::from(chunks[2]),
        &mut registers,
      );
    } else if operator == "mul" {
      op_mul(
        String::from(chunks[1]),
        String::from(chunks[2]),
        &mut registers,
      );
    } else if operator == "mod" {
      op_mod(
        String::from(chunks[1]),
        String::from(chunks[2]),
        &mut registers,
      );
    } else if operator == "rcv" {
      let should_break = op_rcv(String::from(chunks[1]), &mut registers, &mut history);
      if should_break {
        break;
      }
    } else if operator == "jgz" {
      let reg_value = get_register_or_value(String::from(chunks[1]), &mut registers);
      if reg_value > 0 {
        let jump_value = get_register_or_value(String::from(chunks[2]), &mut registers);
        current_index = current_index + jump_value;
      } else {
        current_index += 1;
      }
    }
    if operator != "jgz" {
      current_index += 1;
    }
    // println!("-----");
    // for (key, value) in registers.clone() {
    //   println!("reg {}, value {}", key, value);
    // }
    // println!("-----");
    // println!("");
  }

  for (key, value) in registers {
    println!("reg {}, value {}", key, value);
  }
}
