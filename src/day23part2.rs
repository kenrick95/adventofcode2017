/**
 * Thanks to https://www.reddit.com/r/adventofcode/comments/7lms6p/2017_day_23_solutions/ to suggest translating assembly to code.
 * Especially https://www.reddit.com/r/adventofcode/comments/7lms6p/2017_day_23_solutions/drngj9r/ 
 */
pub fn main() {
  let a = 1;
  let mut b = 93;
  let mut c = b;
  if a != 0 {
    b = b * 100 + 100000;
    c = b + 17000;
  }
  let mut f = 1;
  let mut d = 2;
  let mut e = 2;
  let mut g = 0;
  let mut h = 0;

  while b <= c {
    f = 1;
    d = 2;
    while d * d <= b {
      if b % d == 0 {
        f = 0;
        break;
      }
      d += 1;
    }
    if f == 0 {
      h += 1;
    }
    b += 17;
  }

  println!("a {}, b {}, c {}, d {}, e {}, f {}, g {}, h {}", a, b, c, d, e, f, g, h);
}
