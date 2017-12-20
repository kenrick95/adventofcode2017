use std::fs::File;
use std::io::Read;
use regex::Regex;

struct Coord {
  x: i32,
  y: i32,
  z: i32,
}

struct Particle {
  p: Coord,
  v: Coord,
  a: Coord,
}

pub fn main() {
  let mut input = String::new();

  let mut f = File::open("day20.in.txt").unwrap();
  f.read_to_string(&mut input).unwrap();

  let inputs: Vec<&str> = input.lines().collect();

  let re = Regex::new(r"p=<([-0-9]+),([-0-9]+),([-0-9]+)>, v=<([-0-9]+),([-0-9]+),([-0-9]+)>, a=<([-0-9]+),([-0-9]+),([-0-9]+)>").unwrap();

  let mut particles = Vec::new();

  for input in inputs {
    let caps = re.captures(&input).unwrap();
    
    let px: i32 = String::from(caps.get(1).unwrap().as_str()).parse().unwrap();
    let py: i32 = String::from(caps.get(2).unwrap().as_str()).parse().unwrap();
    let pz: i32 = String::from(caps.get(3).unwrap().as_str()).parse().unwrap();
    
    let vx: i32 = String::from(caps.get(4).unwrap().as_str()).parse().unwrap();
    let vy: i32 = String::from(caps.get(5).unwrap().as_str()).parse().unwrap();
    let vz: i32 = String::from(caps.get(6).unwrap().as_str()).parse().unwrap();
    
    let ax: i32 = String::from(caps.get(7).unwrap().as_str()).parse().unwrap();
    let ay: i32 = String::from(caps.get(8).unwrap().as_str()).parse().unwrap();
    let az: i32 = String::from(caps.get(9).unwrap().as_str()).parse().unwrap();

    // println!("{} {} {}, {} {} {}, {} {} {}", px, py, pz, vx, vy, vz, ax, ay, az);
    
    particles.push(Particle{
      p: Coord { 
        x: px,
        y: py,
        z: pz,
      },
      v: Coord { 
        x: vx,
        y: vy,
        z: vz,
      },
      a: Coord { 
        x: ax,
        y: ay,
        z: az,
      },
    });
  }

  let mut tick = 0;

  loop {
    tick += 1;
    let mut closest_value = 0;
    let mut closest_index = -1;
    for (index, particle) in particles.iter_mut().enumerate() {
      particle.v.x += particle.a.x;
      particle.v.y += particle.a.y;
      particle.v.z += particle.a.z;
      particle.p.x += particle.v.x;
      particle.p.y += particle.v.y;
      particle.p.z += particle.v.z;

      let dist = particle.p.x.abs() + particle.p.y.abs() + particle.p.z.abs();
      if closest_index == -1 {
        closest_index = index as i32;
        closest_value = dist;
      } else if dist < closest_value {
        closest_index = index as i32;
        closest_value = dist;
      }
    }
    println!("{} {}", closest_index, closest_value);

    if tick == 1000000 {
      break;
    }
  }
}
