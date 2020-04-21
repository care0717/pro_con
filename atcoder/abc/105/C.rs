
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
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

fn get_max_index(n: usize) -> i64 {
    return (3.0*n as f64 + 1.0).log(4.0).ceil() as i64
}

fn create_base_minus2_list(index: i64) -> Vec<i64> {
    let mut v = Vec::new();
    for i in 0..index {
        v.push((-2 as i64).pow(i as u32));
    }
    v.reverse();
    return v
}


fn make_binary_string(index_list: Vec<i64>, size: i64) -> String{
    let mut res = "1".to_string();
    let mut count: usize = 0;
    for i in 0..size {
        if index_list.len() > count && index_list[count] == i   {
            res += "1";
            count += 1;
        }else{
            res += "0";
        }
    }
    return res
}
fn main() {
    input!{
        n: usize,
    }

    if n > 0 {
        let max_index = get_max_index(n);
        let size = 2*max_index-2;
        let minus2_list = create_base_minus2_list(size);
        let mut diff = 4_i64.pow(max_index as u32 - 1)-n as i64;
        let mut res = Vec::new();
        'outer: for i in 0..size{
            for j in 0..size{
                if -diff == minus2_list[j as usize] {
                    res.push(j);
                    break 'outer;
                }
            }
            if minus2_list[i as usize]*diff < 0 {
                diff +=  minus2_list[i as usize];
                res.push(i);
            }

            if diff == 0 {
                break;
            }
        }
        println!("{:?}", res);
        println!("{:?}", make_binary_string(res, size));

    }
}