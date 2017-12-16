pub fn main() {
  let mut answer = 0;
  let gen_a_start = 722;
  let gen_b_start = 354;
  // let gen_a_start = 65;
  // let gen_b_start = 8921;
  let mut index = 0;
  let max = 40000000;

  let gen_a_factor = 16807;
  let gen_b_factor = 48271;

  let modulus = 2147483647;

  let mut gen_a: u64 = gen_a_start;
  let mut gen_b: u64 = gen_b_start;

  while index < max {
    gen_a = (gen_a * gen_a_factor) % modulus;
    gen_b = (gen_b * gen_b_factor) % modulus;
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
