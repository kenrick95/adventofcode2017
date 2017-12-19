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
  if history.len() > 0 {
    registers.insert(a, history.pop().unwrap());
    return true;
  }
  return false;
}


pub fn main() {
  let mut input = String::new();
  let mut registers = vec![HashMap::new(), HashMap::new()];
  let mut message_queue = vec![Vec::new(), Vec::new()];

  // PID 0
  // all registers are 0 (including "p" = 0; "p" means PID)
  // PID 1
  // all registers are 0, except "p" having the value 1
  registers[0].insert(String::from("p"), 0 as i64);
  registers[1].insert(String::from("p"), 1 as i64);

  let mut current_pid = 0;
  let mut deadlock_counter = 0;
  let mut deadlock_counter_inst = 0;

  let mut snd_counter = vec![0, 0];

  let mut f = File::open("day18.in.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  let instructions: Vec<&str> = input.lines().collect();
  let instruction_len = instructions.len();

  let mut thread_loop_counter = 0;

  let mut current_index = vec![0, 0];
  loop {
    if thread_loop_counter % 100 == 0 {
      println!("thread loop {}, pid {}", thread_loop_counter, current_pid);
    }
    let mut next_pid = 0;
    if current_pid == 0 {
      next_pid = 1;
    }

    if deadlock_counter == 2 || deadlock_counter_inst == 2 {
      break;
    }

    let ref mut current_pid_index = current_index[current_pid];
    let ref mut current_pid_register = registers[current_pid];
    let ref mut current_pid_snd_counter = snd_counter[current_pid];
    if *current_pid_index < 0 || *current_pid_index >= instruction_len as i64 {
      deadlock_counter_inst += 1;
      continue;
    }
    deadlock_counter_inst = 0;

    while *current_pid_index >= 0 && *current_pid_index < instruction_len as i64 {
      let instruction = instructions[*current_pid_index as usize];

      // println!("{} {}", current_pid_index, instruction);

      // parse
      let chunks: Vec<&str> = instruction.trim().split(" ").collect();

      let operator = String::from(chunks[0]);

      if operator == "snd" {
        op_snd(String::from(chunks[1]), current_pid_register, &mut message_queue[next_pid]);
        *current_pid_snd_counter += 1;
      } else if operator == "set" {
        op_set(
          String::from(chunks[1]),
          String::from(chunks[2]),
          current_pid_register,
        );
      } else if operator == "add" {
        op_add(
          String::from(chunks[1]),
          String::from(chunks[2]),
          current_pid_register,
        );
      } else if operator == "mul" {
        op_mul(
          String::from(chunks[1]),
          String::from(chunks[2]),
          current_pid_register,
        );
      } else if operator == "mod" {
        op_mod(
          String::from(chunks[1]),
          String::from(chunks[2]),
          current_pid_register,
        );
      } else if operator == "rcv" {
        let success = op_rcv(String::from(chunks[1]), current_pid_register, &mut message_queue[current_pid]);
        if !success {
          // break and let other "thread" run
          deadlock_counter += 1;
          break;
        }
      } else if operator == "jgz" {
        let reg_value = get_register_or_value(String::from(chunks[1]), current_pid_register);
        if reg_value > 0 {
          let jump_value = get_register_or_value(String::from(chunks[2]), current_pid_register);
          *current_pid_index = *current_pid_index + jump_value;
        } else {
          *current_pid_index += 1;
        }
      }
      if operator != "jgz" {
        *current_pid_index += 1;
      }
      deadlock_counter = 0;
    }
    // println!("-----");
    // for (key, value) in current_pid_register {
    //   println!("reg {}, value {}", key, value);
    // }
    // println!("deadlock_counter {}", deadlock_counter);
    // println!("-----");
    // println!("");

    current_pid = next_pid;
    thread_loop_counter += 1;
  }

  println!("answer {}", snd_counter[1]);

  // for (key, value) in registers {
  //   println!("reg {}, value {}", key, value);
  // }
}
