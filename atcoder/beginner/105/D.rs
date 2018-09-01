use std::collections::HashMap;
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn comb(n: i64, r: i64) -> i64 {
    match (n, r) {
        (0, _) | (_, 0) => 1,
        _ => comb(n, r - 1) * (n - r + 1) / r
    }
}


fn main() {
    input!{
        n: usize,
        m: i64,
        v: [i64; n],
    }
    let mut sum: Vec<i64> = Vec::new();
    sum.push(v[0] % m);
    let mut temp = v[0] % m ;
    for i in 0..n-1 {
        temp += v[i+1];
        sum.push(temp % m);
    }
    let mut map: HashMap<i64, i64>  = HashMap::new();

    for x in sum.iter() {
        if !map.contains_key(&x) {
            map.insert(*x, 1);
        } else {
            *map.get_mut(&x).unwrap() += 1;
        }
    }
    let mut res: i64= 0;
    for (key, val) in map.iter() {
        if  *val > 1 {
            res += comb(*val, 2_i64)
        }
    }
    res += *map.get(&0).unwrap_or(&0);
    println!("{}", res);

}