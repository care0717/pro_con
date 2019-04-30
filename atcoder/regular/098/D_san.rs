#[allow(dead_code)]
fn getline() -> String {
    let mut res = String::new();
    std::io::stdin().read_line(&mut res).ok();
    res
}

#[allow(unused_macros)]
macro_rules! readl {
    ($t: ty) => {
        {
            let s = getline();
            s.trim().parse::<$t>().unwrap()
        }
    };
    ($( $t: ty),+ ) => {
        {
            let s = getline();
            let mut iter = s.trim().split(' ');
            ($(iter.next().unwrap().parse::<$t>().unwrap(),)*)
        }
    };
}

#[allow(unused_macros)]
macro_rules! readlvec {
    ($t: ty) => {
        {
            let s = getline();
            let iter = s.trim().split(' ');
            iter.map(|x| x.parse().unwrap()).collect::<Vec<$t>>()
        }
    }
}

#[allow(unused_macros)]
macro_rules! debug { ($x: expr) => { println!("{}: {:?}", stringify!($x), $x) } }
// macro_rules! debug { ($x: expr) => () }

#[allow(dead_code)]
fn show<T>(iter: T) -> String
where
    T: Iterator,
    T::Item: std::fmt::Display
{
    let mut res = iter.fold(String::new(), |sum, e| sum + &format!("{} ", e));
    res.pop();
    res
}

#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::collections::btree_map::Entry::{Occupied, Vacant};

fn naive(n: usize, a: Vec<i32>) -> usize {
    let mut ans = 0;
    for l in 0..n {
        let mut sum = 0;
        let mut xor = 0;
        for r in l..n {
            sum += a[r];
            xor ^= a[r];
            if sum == xor {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    use std::io::Write;
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    macro_rules! printf { ($($arg:tt)*) => (write!(out, $($arg)*).unwrap()); }
    macro_rules! printfln { () => (writeln!(out).unwrap()); ($($arg:tt)*) => (writeln!(out, $($arg)*).unwrap()); }
    
    let n = readl!(usize);
    let a = readlvec!(i32);
    let mut l = 0;
    let mut r = 0;
    let mut sum = 0;
    let mut xor = 0;
    let mut ans = 0;
    let mut roopc = 0;
    while l < n {
       roopc += 1;
        while r < n && sum+a[r] == xor^a[r] {
            sum += a[r];
            xor ^= a[r];
            r += 1;
             roopc += 1;
        }
        // debug!((l, r));
        ans += r-l;
        printfln!("{}", l);
        printfln!("{}", r);
        
        if l == r {
            l += 1;
            r += 1;
            sum = 0;
            xor = 0;
        } else {
            sum -= a[l];
            xor ^= a[l];
            l += 1;
        }
    }
    printfln!("{}", roopc);
    printfln!("{}", ans);

}
