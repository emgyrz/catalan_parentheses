#!/usr/bin/node

const variants = []
const result = []
let total = 0

function main() {
  const arg = parseInt( process.argv[2] );
  console.time('bench')
  total = arg
  run(arg);
  console.timeEnd('bench')
}

main()

// fn to_str(arr: &[char; 16], count: usize) -> String {
//   let mut s = String::with_capacity(count);
//   for i in 0..count {
//     let b = arr[i];
//     s.push(b);
//   }
//   s
// }

function reverse(s) {
  return s.split("").reverse().map(s => s === '(' ? ')' : '(').join("")
}



function run(count) {

  gen("(", 1);

  for (let i = 0; i <= count; i++) {
    let its = variants.filter(it => it[2] === i);
    let len = its.length;
    for (let ind = 0; ind < len; ind++) {
      for (let idx = 0; idx < len; idx++) {
        result.push(its[ind][0] + its[idx][1]);
      }
    }
  }

  console.log(result.length)
}


function gen(
  start,
  opening,
) {
  const len = start.length
  let new_s_len = len + 1;
  let is_latest = new_s_len === total;
  let closing_len = len - opening;

  let new_s1 = start + '(';
  let opening_plus = opening + 1;
  if (is_latest ) {
    variants.push([new_s1, reverse(new_s1), opening_plus - closing_len]);
  } else {
    gen(new_s1, opening_plus);
  }

  if ((opening - closing_len) > 0 ) {
    let new_s2 = start + ')'
    let closing_plus = closing_len + 1;
    if ( is_latest ) {
      variants.push([new_s2, reverse(new_s2), opening - closing_plus])
    } else {
      gen(new_s2, opening);
    }
  }
}

