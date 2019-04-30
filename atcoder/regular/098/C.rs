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
  let _n: u32 = read();
  let _s: String = read();
  let e_num = _s.chars().filter(|&c| c == 'E').count();
  let mut right_e_num = e_num;
  let mut left_w_num = 0;
  let s: Vec<char> = _s.chars().collect();
  let mut res = Vec::new();
  for i in s {
    if i == 'E'{
      right_e_num -= 1;
    } 
    res.push(left_w_num + right_e_num);
    if i=='W' {
      left_w_num += 1;
    }
  }
  println!("{}", res.iter().min().unwrap());
  
}
