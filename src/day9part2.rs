use std::io;

pub fn main() {
  let mut garbage_count = 0;

  // TODO: Read input from file
  loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "" {
      break;
    }


    let mut char_pointer = input.trim().chars();
    let mut current_level = 0;
    let mut is_garbage = false; // if true, ignore chars until next '>'
    let mut is_cancel = false; // if true, cancel 1 next char (this has higher priority than is_garbade)

    loop {
      let character = match char_pointer.next() {
        Some(x) => x,
        None => break
      };

      if is_cancel {
        is_cancel = false;
        continue;
      }

      if character == '!' {
        is_cancel = true;
        continue;
      }
      
      if character == '>' {
        is_garbage = false;
        continue;
      }
      if is_garbage {
        garbage_count += 1;
        continue;
      }
      if character == '<' {
        is_garbage = true;
        continue;
      }
      if character == '{' {
        current_level += 1;
        continue;
      }
      if character == '}' {
        current_level -= 1;
        continue;
      }
    }

  }

  println!("{}", garbage_count);
}
