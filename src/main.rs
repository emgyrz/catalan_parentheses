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
    .expect("cant parse first argument as usize");

  run(a);
}

fn gen(
  results: &mut Vec<(String, String, usize)>,
  count: usize,
  start: &str,
  opening: usize,
  closing: usize,
) {
  let new_s_len = start.len() + 1;
  let is_latest = new_s_len == count;

  let mut new_s1 = String::from(start);
  new_s1.push(O_BRACKET);
  let opening_plus = opening + 1;
  if is_latest {
    let reversed = push_reversed(&new_s1);
    results.push((new_s1, reversed, opening_plus - closing));
  } else {
    gen(results, count, &new_s1, opening_plus, closing);
  }

  if opening - closing > 0 {
    let mut new_s2 = String::from(start);
    new_s2.push(C_BRACKET);
    let closing_plus = closing + 1;
    if is_latest {
      let reversed = push_reversed(&new_s2);
      results.push((new_s2, reversed, opening - closing_plus));
    } else {
      gen(results, count, &new_s2, opening, closing_plus);
    }
  }
}

fn run(count: u32) {
  let count_us = count as usize;
  let mut results = Vec::with_capacity(catalan_count(count) as usize);

  gen(&mut results, count_us, "(", 1, 0);

  let mut res = Vec::with_capacity(catalan_count(count) as usize);

  for i in 0..=count_us {
    let its: Vec<_> = results.iter().filter(|it| it.2 == i).collect();
    let len = its.len();
    for ind in 0..len {
      for idx in 0..len {
        let mut start = its[ind].0.clone();
        start.push_str(&its[idx].1);
        res.push(start);
      }
    }
  }

  // println!("{:?} \n {}", res, res.len());
  println!("\n {}", res.len());
}

fn push_reversed(s: &str) -> String {
  s.chars()
    .rev()
    .map(|c| if c == '(' { ")" } else { "(" })
    .collect::<Vec<&str>>()
    .join("")
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
