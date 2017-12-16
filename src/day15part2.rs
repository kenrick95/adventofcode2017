
pub fn get_a_value(current_value: u64) -> u64 {
  let modulus = 2147483647;
  let gen_a_factor = 16807;
  let mut temp = current_value;
  while (|| {
    temp = (temp * gen_a_factor) % modulus;
    return temp % 4 != 0;
  })()
  {}

  return temp;
}
pub fn get_b_value(current_value: u64) -> u64 {
  let modulus = 2147483647;
  let gen_b_factor = 48271;
  let mut temp = current_value;
  while (|| {
    temp = (temp * gen_b_factor) % modulus;
    return temp % 8 != 0;
  })()
  {}

  return temp;
}

pub fn main() {
  let mut answer = 0;
  let gen_a_start = 722;
  let gen_b_start = 354;
  // let gen_a_start = 65;
  // let gen_b_start = 8921;
  let mut index = 0;
  let max = 5000000;

  let mut gen_a: u64 = gen_a_start;
  let mut gen_b: u64 = gen_b_start;

  while index < max {
    gen_a = get_a_value(gen_a);
    gen_b = get_b_value(gen_b);
    // compare gen a & gen b lowest 16 bit --> mod 2 ^ 16 (note: 2 ^ 16 has 17 bits)
    // println!("{} {} {}", index, gen_a, gen_b);
    if gen_a % 65536 == gen_b % 65536 {
      // println!("match {}, {}, {} {}", index, gen_a, gen_b, gen_a % 65536);
      answer += 1;
    }
    index += 1;
  }

  println!("{}", answer);
}
