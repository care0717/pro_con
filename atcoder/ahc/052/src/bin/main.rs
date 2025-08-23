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

use rand::prelude::*;
use std::time::Instant;

const MAX_OPS: usize = 200;
const TIME_LIMIT: f64 = 1.8;
const ACTIONS: [char; 5] = ['U', 'D', 'L', 'R', 'S'];
const DIJ: [(i32, i32); 5] = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];

#[derive(Clone)]
struct State {
    button_config: Vec<Vec<char>>,
    operations: Vec<usize>,
    score: i64,
}

fn main() {
    let start = Instant::now();
    
    input! {
        n: usize,
        m: usize,
        k: usize,
        robots: [(usize, usize); m],
        v: [chars; n],
        h: [chars; n - 1],
    }

    let mut rng = thread_rng();
    
    // Initialize with random solution
    let mut best_state = generate_initial_solution(&mut rng, n, m, k, &robots, &v, &h);
    best_state.score = evaluate(&best_state, n, m, &robots, &v, &h);
    
    // Simulated Annealing
    let mut current_state = best_state.clone();
    let mut temperature = 100.0;
    let cooling_rate = 0.999;
    let mut counter = 0;
    
    while start.elapsed().as_secs_f64() < TIME_LIMIT {
        let mut new_state = current_state.clone();
        counter += 1;
        
        // Random neighbor
        match rng.gen_range(0..3) {
            0 => {
                // Change a button configuration
                let button = rng.gen_range(0..k);
                let robot = rng.gen_range(0..m);
                new_state.button_config[button][robot] = ACTIONS[rng.gen_range(0..5)];
            }
            1 => {
                // Change an operation
                if !new_state.operations.is_empty() {
                    let idx = rng.gen_range(0..new_state.operations.len());
                    new_state.operations[idx] = rng.gen_range(0..k);
                }
            }
            _ => {
                // Add or remove operation
                if new_state.operations.len() < MAX_OPS && rng.gen_bool(0.5) {
                    new_state.operations.push(rng.gen_range(0..k));
                } else if new_state.operations.len() > 10 {
                    new_state.operations.pop();
                }
            }
        }
        
        new_state.score = evaluate(&new_state, n, m, &robots, &v, &h);
        
        let delta = new_state.score - current_state.score;
        if delta > 0 || rng.gen_bool((delta as f64 / temperature).exp()) {
            current_state = new_state;
            if current_state.score > best_state.score {
                best_state = current_state.clone();
            }
        }
        
        temperature *= cooling_rate;
    }
    eprintln!("counter: {}", counter);
    
    // Output
    for row in &best_state.button_config {
        println!("{}", row.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
    }
    
    for op in &best_state.operations {
        println!("{}", op);
    }
}

fn generate_initial_solution(
    rng: &mut ThreadRng,
    n: usize,
    m: usize,
    k: usize,
    robots: &[(usize, usize)],
    v: &[Vec<char>],
    h: &[Vec<char>],
) -> State {
    let mut button_config = vec![vec!['S'; m]; k];
    
    // Set up basic movement patterns
    for b in 0..k.min(4) {
        for r in 0..m {
            button_config[b][r] = ACTIONS[b];
        }
    }
    
    // Mixed patterns for remaining buttons
    for b in 4..k {
        for r in 0..m {
            button_config[b][r] = ACTIONS[rng.gen_range(0..5)];
        }
    }
    
    // Generate operations
    let mut operations = Vec::new();
    for _ in 0..MAX_OPS {
        operations.push(rng.gen_range(0..k));
    }
    
    State {
        button_config,
        operations,
        score: 0,
    }
}

fn evaluate(
    state: &State,
    n: usize,
    m: usize,
    robots: &[(usize, usize)],
    v: &[Vec<char>],
    h: &[Vec<char>],
) -> i64 {
    let mut visited = vec![vec![false; n]; n];
    let mut robot_positions = robots.to_vec();
    
    // Mark initial positions
    for &(i, j) in &robot_positions {
        visited[i][j] = true;
    }
    
    // Simulate operations
    for &button in &state.operations {
        let mut new_positions = robot_positions.clone();
        
        for r in 0..m {
            let (ci, cj) = robot_positions[r];
            let action = state.button_config[button][r];
            
            let dir_idx = ACTIONS.iter().position(|&c| c == action).unwrap_or(4);
            let (di, dj) = DIJ[dir_idx];
            
            let ni = (ci as i32 + di) as usize;
            let nj = (cj as i32 + dj) as usize;
            
            if can_move(n, ci, cj, ni, nj, v, h) {
                new_positions[r] = (ni, nj);
                visited[ni][nj] = true;
            }
        }
        
        robot_positions = new_positions;
    }
    
    // Calculate score
    let mut unvisited = 0;
    for i in 0..n {
        for j in 0..n {
            if !visited[i][j] {
                unvisited += 1;
            }
        }
    }
    
    if unvisited == 0 {
        (3 * n * n - state.operations.len()) as i64
    } else {
        (n * n - unvisited) as i64
    }
}

fn can_move(
    n: usize,
    ci: usize,
    cj: usize,
    ni: usize,
    nj: usize,
    v: &[Vec<char>],
    h: &[Vec<char>],
) -> bool {
    if ni >= n || nj >= n {
        return false;
    }
    
    if ci == ni {
        // Horizontal move
        if cj < nj {
            // Moving right
            if cj < n - 1 {
                v[ci][cj] != '1'
            } else {
                false
            }
        } else if cj > nj {
            // Moving left
            if nj < n - 1 {
                v[ci][nj] != '1'
            } else {
                false
            }
        } else {
            true // Stay
        }
    } else if cj == nj {
        // Vertical move
        if ci < ni {
            // Moving down
            if ci < n - 1 {
                h[ci][cj] != '1'
            } else {
                false
            }
        } else if ci > ni {
            // Moving up
            if ni < n - 1 {
                h[ni][cj] != '1'
            } else {
                false
            }
        } else {
            true // Stay
        }
    } else {
        true // Stay
    }
}