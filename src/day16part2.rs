use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;

fn get_index(fake_index: usize, start_index: usize, programs_len: usize) -> usize {
  if fake_index < start_index {
    return fake_index + programs_len - start_index;
  }
  return fake_index - start_index;
}

/**
 * Thanks to https://www.reddit.com/r/adventofcode/comments/7k572l/2017_day_16_solutions/drboovu/
 *
 * */
pub fn main() {
  // init
  let mut memo = Vec::new();
  let mut programs = VecDeque::from(vec![
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
    'i',
    'j',
    'k',
    'l',
    'm',
    'n',
    'o',
    'p',
  ]);
  let mut start_index = 0 as usize;
  let programs_len = programs.len();

  let mut input = String::new();

  let mut f = File::open("day16.in.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  let mut counter = 0;
  let max = 1000000000;

  while counter < max {
    if counter % 10000 == 0 {
      println!("{}", counter);
    }

    let key = programs.clone();

    let mut found = false;

    for programs_key in &memo {
      // for program in programs_key {
      //   print!("{}", program);
      // }
      // println!("");
      if *programs_key == key {
        found = true;
        break;
      }
    }
    if found {
      break;
    }

    memo.push(key);

    for operation in input.split(",") {
      let op_code = operation.trim().chars().nth(0).unwrap();
      let mut operands_string = String::from(operation.trim());
      operands_string.remove(0);
      if op_code == 's' {
        let operand: usize = String::from(operands_string).parse().unwrap();
        start_index = (start_index + operand) % programs_len;
      } else if op_code == 'x' {
        let operands: Vec<&str> = operands_string.split("/").collect();
        let lhs: usize = String::from(operands[0]).parse().unwrap();
        let rhs: usize = String::from(operands[1]).parse().unwrap();

        programs.swap(
          get_index(lhs, start_index, programs_len),
          get_index(rhs, start_index, programs_len),
        );
      } else if op_code == 'p' {
        let operands: Vec<&str> = operands_string.split("/").collect();
        let lhs: char = (*operands[0]).chars().nth(0).unwrap();
        let rhs: char = (*operands[1]).chars().nth(0).unwrap();
        let mut lhs_index = programs_len;
        let mut rhs_index = programs_len;

        for (index, program) in programs.iter().enumerate() {
          if lhs == *program {
            lhs_index = index;
          } else if rhs == *program {
            rhs_index = index;
          }
          if lhs_index != programs_len && rhs_index != programs_len {
            break;
          }
        }
        // println!("lhs_index {}, rhs_index {}", lhs_index, rhs_index);

        programs.swap(lhs_index, rhs_index);
      }
      // println!("----");
      // for program in programs.iter() {
      //   print!("{}", program);
      // }
      // println!("");
    }


    while start_index > 0 {
      let x = programs.pop_back().unwrap();
      programs.push_front(x);
      start_index -= 1;
    }


    counter += 1;
  }


  // println!("start {}", start_index);
  for program in memo[max % counter].iter() {
    print!("{}", program);
  }
  println!("");
}
