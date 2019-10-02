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

  run(a);
}




fn run(count: u32) {
  let count_us = count as usize;
  let results_len = catalan_count(count) as usize;
  let mut variants = Vec::with_capacity(results_len);

  gen("(", count_us, 1, &mut variants);

  let mut result = Vec::with_capacity(results_len);

  for i in 0..=count_us {
    let its: Vec<_> = variants.iter().filter(|it| it.2 == i).collect();
    let len = its.len();
    for ind in 0..len {
      for idx in 0..len {
        let mut start = its[ind].0.clone();
        start.push_str(&its[idx].1);
        result.push(start);
      }
    }
  }

  // println!("{:?} \n {}", result, result.len());
  println!("\n {}", result.len());
}



fn gen(
  start: &str,
  total: usize,
  opening: usize,
  variants: &mut Vec<(String, String, usize)>,
) {
  let len = start.len();
  let new_s_len = len + 1;
  let is_latest = new_s_len == total;
  let closing_len = len - opening;

  let mut new_s1 = String::from(start);
  new_s1.push(O_BRACKET);
  let opening_plus = opening + 1;
  if is_latest {
    let reversed = reverse(&new_s1);
    variants.push((new_s1, reversed, opening_plus - closing_len));
  } else {
    gen(&new_s1, total, opening_plus, variants);
  }

  if opening - closing_len > 0 {
    let mut new_s2 = String::from(start);
    new_s2.push(C_BRACKET);
    let closing_plus = closing_len + 1;
    if is_latest {
      let reversed = reverse(&new_s2);
      variants.push((new_s2, reversed, opening - closing_plus));
    } else {
      gen(&new_s2, total, opening, variants);
    }
  }
}


fn reverse(s: &str) -> String {
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
