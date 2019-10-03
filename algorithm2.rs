use std::time::{Instant,Duration};

static O_BRACKET: char = '(';
static C_BRACKET: char = ')';


fn main() {
  let count: Vec<String> = std::env::args().collect();
  if count.len() < 2 {
    eprintln!("specify count of parentheses");
    std::process::exit(1);
  }
  let a: u32 = count[1]
    .parse()
    .expect("cant parse first argument as uint");
  let now = Instant::now();
  run(a);
  println!("time: {}", now.elapsed().as_millis());
}


#[derive(Copy,Clone,Default,Debug)]
struct Var {
  arr: [char; 16],
  pointer: usize,
}


impl Var {
  fn add(&mut self, b: char) {
    self.arr[self.pointer] = b;
    self.pointer += 1;
  }

}

fn to_str(arr: &[char; 16], count: usize) -> String {
  let mut s = String::with_capacity(count);
  for i in 0..count {
    let b = arr[i];
    s.push(b);
  }
  s
}

fn reverse(arr: &[char; 16], count: usize) -> [char; 16] {
  let mut reversed = [O_BRACKET; 16];
  for (i,b) in arr[..count].iter().rev().enumerate() {
    reversed[i] = if b == &O_BRACKET {C_BRACKET} else {O_BRACKET};
  }

  reversed
}



fn run(count: u32) {
  let count_us = count as usize;
  let results_len = catalan_count(count) as usize;
  let mut variants = Vec::with_capacity(results_len);
  let mut var = Var::default();
  var.add(O_BRACKET);
  gen(&mut var, count_us, 1, &mut variants);

  let mut result = Vec::with_capacity(results_len);

  for i in 0..=count_us {
    let its: Vec<_> = variants.iter().filter(|it| it.2 == i).collect();
    let len = its.len();
    for ind in 0..len {
      for idx in 0..len {
        let mut s = String::with_capacity(count_us * 2);
        s.push_str(&its[ind].0);
        s.push_str(&its[idx].1);
        result.push(s);
      }
    }
  }

  // println!("{:?} \n {}", result, result.len());
  println!("\n {}", result.len());
}


fn gen(
  start: &mut Var,
  total: usize,
  opening: usize,
  variants: &mut Vec<(String, String, usize)>,
) {

  let new_s_len = start.pointer + 1;
  let is_latest = new_s_len == total;
  let closing_len = start.pointer - opening;

  let mut new_s1 = *start;
  new_s1.add(O_BRACKET);
  let opening_plus = opening + 1;
  if is_latest {
    variants.push((to_str(&new_s1.arr,new_s_len), to_str(&reverse(&new_s1.arr,new_s_len),new_s_len), opening_plus - closing_len));
  } else {
    gen(&mut new_s1, total, opening_plus, variants);
  }

  if opening - closing_len > 0 {
    start.add(C_BRACKET);
    let closing_plus = closing_len + 1;
    if is_latest {
      variants.push((to_str(&start.arr,new_s_len), to_str(&reverse(&start.arr,new_s_len),new_s_len), opening - closing_plus));
    } else {
      gen(start, total, opening, variants);
    }
  }
}


fn catalan_count(n: u32) -> u32 {
  let f = f64::from(n);

  if n >= 2 {
    let x = (2.0 * ((2.0 * f) - 1.0)) / (f + 1.0);
    (x * f64::from(catalan_count(n - 1))) as u32
  } else {
    1
  }
}
