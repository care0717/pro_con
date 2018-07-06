use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char) 
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
  let _a: f64 = read();
  let a: u32 = _a as u32;
  let b = 10f64.powi(_a.log10() as i32) as u32;
  if a == b {
    println!("{}", 10);
  } else {
    println!("{}", sum_digit_num(a));
  }
}

fn sum_digit_num(a: u32) -> u32{
  let mut b = a/10*10;
  let mut diff = a-b;
  let mut res = diff;
  let mut x = a/10;
  loop {
    b = x/10*10;
    diff = x-b;
    res += diff;
    if b == 0 { break;}
    x = x/10;
  }
  return res
}
