macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

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

use std::collections::HashSet;

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const DIR_CHARS: [char; 4] = ['U', 'D', 'L', 'R'];

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        robots: [(usize, usize); m],
        v: [chars; n],
        h: [chars; n - 1],
    }

    let mut visited = HashSet::new();
    for &(i, j) in &robots {
        visited.insert((i, j));
    }

    let mut button_config = vec![vec!['S'; m]; k];
    for b in 0..k.min(4) {
        for r in 0..m {
            button_config[b][r] = DIR_CHARS[b];
        }
    }

    let mut operations = Vec::new();
    let mut robot_positions = robots.clone();
    
    for _ in 0..1800 {
        if visited.len() >= (n * n) {
            break;
        }
        
        let button = operations.len() % 4;
        
        let mut new_positions = robot_positions.clone();
        for r in 0..m {
            let (ci, cj) = robot_positions[r];
            let action = button_config[button][r];
            
            let (ni, nj) = match action {
                'U' if ci > 0 && !is_wall_h(&h, ci - 1, cj) => (ci - 1, cj),
                'D' if ci < n - 1 && !is_wall_h(&h, ci, cj) => (ci + 1, cj),
                'L' if cj > 0 && !is_wall_v(&v, ci, cj - 1) => (ci, cj - 1),
                'R' if cj < n - 1 && !is_wall_v(&v, ci, cj) => (ci, cj + 1),
                _ => (ci, cj),
            };
            
            new_positions[r] = (ni, nj);
            visited.insert((ni, nj));
        }
        
        robot_positions = new_positions;
        operations.push(button);
    }

    for row in &button_config {
        println!("{}", row.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
    }
    
    for op in operations {
        println!("{}", op);
    }
}

fn is_wall_v(v: &[Vec<char>], i: usize, j: usize) -> bool {
    v[i][j] == '1'
}

fn is_wall_h(h: &[Vec<char>], i: usize, j: usize) -> bool {
    h[i][j] == '1'
}
