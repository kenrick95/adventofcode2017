use std::io;

fn get_side(layer: u32) -> u32 {
  return layer * 2 - 1;
}

pub fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  let input_number: u32 = input.trim().parse().unwrap();

  let root: u32 = ((input_number as f64).sqrt().ceil() as u32);

  let mut nearest_odd = root;

  if root % 2 == 0 {
    nearest_odd = nearest_odd + 1;
  }

  let nth_layer = (nearest_odd + 1) / 2;

  let mut answer = nth_layer;

  // so nth_layer - 1 <= answer <= nth_layer * 2 - 2
  // (from 1 to nth_layer, Manhattan distance is nth_layer - 1)
  // (from 1 to corner, Manhattan distance is nth_layer * 2 - 2)

  let side = get_side(nth_layer);

  let head_br = nearest_odd * nearest_odd;
  let head_bl = head_br - side + 1;
  let head_tl = head_bl - side + 1;
  let head_tr = head_tl - side + 1;


  if (head_bl < input_number && input_number <= head_br) {
    println!("bottom");
    let mid = head_br - nth_layer + 1;
    println!("mid {}", mid);
    answer = answer + (((input_number - mid) as i32).abs() as u32);
  } else if (head_tl < input_number && input_number <= head_bl) {
    println!("left");
    let mid = head_bl - nth_layer + 1;
    println!("mid {}", mid);
    answer = answer + (((input_number - mid) as i32).abs() as u32);
  } else if (head_tr < input_number && input_number <= head_tl) {
    println!("top");
    let mid = head_tl - nth_layer + 1;
    println!("mid {}", mid);
    answer = answer + (((input_number - mid) as i32).abs() as u32);
  } else {
    println!("right");
    let mid = head_tr - nth_layer + 1;
    println!("mid {}", mid);
    answer = answer + (((input_number - mid) as i32).abs() as u32);
  }

  println!("{}", answer);

}
