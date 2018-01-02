use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use regex::Regex;

struct Action {
  delta: i32, // +1: right, -1: left
  next_state: String,
  write: u32,
}

struct State {
  name: String,
  action_0: Action,
  action_1: Action,
}

pub fn main() {
  let mut states: HashMap<String, State> = HashMap::new();
  let mut input = String::new();
  let start_state;
  let max_counter;

  let mut f = File::open("day25.in.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  // Parse input
  // - Get start state
  {
    let re_start_state = Regex::new(r"Begin in state ([A-Z])\.").unwrap();
    let caps = re_start_state.captures(&input).unwrap();
    start_state = String::from(caps.get(1).unwrap().as_str());
    println!("start_state {}", start_state);
  }

  // - Get max_counter
  {
    let re_max_counter = Regex::new(r"Perform a diagnostic checksum after (\d+) steps.").unwrap();
    let caps = re_max_counter.captures(&input).unwrap();
    max_counter = String::from(caps.get(1).unwrap().as_str()).parse().unwrap();
    println!("max_counter {}", max_counter);
  }

  // - Parse State + Action
  {
    let re_state = Regex::new(r"In state ([A-Z]):\s+If the current value is (0|1):\s+- Write the value (0|1)\.\s+- Move one slot to the (left|right)\.\s+- Continue with state ([A-Z])\.\s+If the current value is (0|1):\s+- Write the value (0|1)\.\s+- Move one slot to the (left|right)\.\s+- Continue with state ([A-Z])\.").unwrap();
    
    for caps in re_state.captures_iter(&input) {
      let state_name = String::from(caps.get(1).unwrap().as_str());
      // let state_condition_0: u32 = String::from(caps.get(2).unwrap().as_str()).parse().unwrap();
      let state_action_0_write: u32 = String::from(caps.get(3).unwrap().as_str()).parse().unwrap();
      let state_action_0_move = String::from(caps.get(4).unwrap().as_str());
      let state_action_0_next = String::from(caps.get(5).unwrap().as_str());
      // let state_condition_1: u32 = String::from(caps.get(6).unwrap().as_str()).parse().unwrap();
      let state_action_1_write: u32 = String::from(caps.get(7).unwrap().as_str()).parse().unwrap();
      let state_action_1_move = String::from(caps.get(8).unwrap().as_str());
      let state_action_1_next = String::from(caps.get(9).unwrap().as_str());

      let action_0;
      let action_0_delta;
      if state_action_0_move == "left" {
        action_0_delta = -1;
      } else {
        action_0_delta = 1;
      }
      action_0 = Action {
        delta: action_0_delta,
        write: state_action_0_write,
        next_state: state_action_0_next
      };

      let action_1;
      let action_1_delta;
      if state_action_1_move == "left" {
        action_1_delta = -1;
      } else {
        action_1_delta = 1;
      }
      action_1 = Action {
        delta: action_1_delta,
        write: state_action_1_write,
        next_state: state_action_1_next
      };

      states.insert(state_name.clone(), State {
        name: state_name.clone(),
        action_0: action_0,
        action_1: action_1
      });
    }
  }

  let mut tape = vec![];
  let mut counter = 0;
  let tape_width = max_counter * 2 + 1;
  while counter < tape_width {
    tape.push(0);
    counter += 1;
  }
  let mut current_tape_index = max_counter;

  
  let mut current_state = states.get(&start_state).unwrap();
  counter = 0;
  while counter < max_counter {
    let current_tape_value = tape[current_tape_index];
    // println!("Iteration #{}", counter);
    // println!("current_tape_value: {}", current_tape_value);
    // println!("current_state_name {}", current_state.name);
    let next_state_name;
    let next_tape_index;
    if current_tape_value == 0 {
      tape[current_tape_index] = current_state.action_0.write;
      next_tape_index = (current_tape_index as i32 + &current_state.action_0.delta) as usize;
      next_state_name = &current_state.action_0.next_state;
    } else { // if current_tape_value == 1
      tape[current_tape_index] = current_state.action_1.write;
      next_tape_index = (current_tape_index as i32 + &current_state.action_1.delta) as usize;
      next_state_name = &current_state.action_1.next_state;
    }
    // println!("next_tape_index {}", next_tape_index);
    // println!("next_state_name {}", next_state_name);
    // println!("{:?}", tape);

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
