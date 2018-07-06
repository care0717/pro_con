fn main() {
  let mut buf = String::new();

  // 標準入力から全部bufに読み込む
  std::io::stdin().read_to_string(&mut buf).unwrap();

  // 読み込んだStringを空白で分解する
  let mut iter = buf.split_whitespace();

  let n: usize = iter.next().unwrap().parse().unwrap();
  let a: usize = iter.next().unwrap().parse().unwrap();
  let b: usize = iter.next().unwrap().parse().unwrap();
  let k: usize = iter.next().unwrap().parse().unwrap();
  let c = a+b
}
