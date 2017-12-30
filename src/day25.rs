use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use regex::Regex;

struct Action {
  delta: i32, // +1: right, -1: left
  next_state: String,
}

struct State {
  name: String,
  value: u32,
  action_0: Action,
  action_1: Action,
}

pub fn main() {
  let mut states: HashMap<String, State> = HashMap::new();
  let mut input = String::new();
  let mut start_state = String::new();
  let mut max_counter = 0;

  let mut f = File::open("day25.in.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  // Parse input
  // - Get start state
  let re_start_state = Regex::new(r"Begin in state ([A-Z])\.").unwrap();
  // TODO: use regex

  // - Get max_counter
  let re_max_counter = Regex::new(r"Perform a diagnostic checksum after (\d+) steps.").unwrap();
  // TODO: use regex

  // - Parse State + Action
  let re_state = Regex::new(r"In state ([A-Z]):
  If the current value is (0|1):
    - Write the value (0|1)\.
    - Move one slot to the (left|right)\.
    - Continue with state ([A-Z])\.
  If the current value is (0|1):
    - Write the value (0|1)\.
    - Move one slot to the (left|right)\.
    - Continue with state ([A-Z])\.").unwrap();
  // TODO: use regex

  let mut tape = vec![];
  let mut counter = 0;
  let tape_width = max_counter * 2 + 1;
  while counter < tape_width {
    tape.push(0);
    counter += 1;
  }
  let mut current_tape_index = max_counter;

  
  let mut current_state = states.get(start_state).unwrap();
  counter = 0;
  while counter < max_counter {
    let current_tape_value = tape[current_tape_index];
    let next_state_name;
    let next_tape_index;
    if current_tape_value == 0 {
      tape[current_tape_index] = 1;
      next_tape_index = current_tape_index + current_state.action_0.delta;
      next_state_name = current_state.action_0.next_state;
    } else if current_tape_value == 1 {
      tape[current_tape_index] = 0;
      next_tape_index = current_tape_index + current_state.action_1.delta;
      next_state_name = current_state.action_1.next_state;
    } else {
      // Chaos
    }

    counter += 1;
    current_state = states.get(next_state_name).unwrap();
    current_tape_index = next_tape_index;
  }

  let mut answer = 0;
  counter = 0;
  while counter < tape_width {
    answer += tape[counter];
    counter += 1;
  }
  println!("{}", answer);
}
