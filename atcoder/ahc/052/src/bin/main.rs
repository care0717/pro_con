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

const MAX_OPS: usize = 1800;
const TIME_LIMIT: f64 = 1.8;
const DIJ: [(i32, i32); 5] = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];

#[derive(Clone)]
struct State {
    button_config: Vec<Vec<u8>>,  // Use u8 instead of char for faster comparison
    operations: Vec<usize>,
    score: i64,
}

// Precompute wall checks
struct Walls {
    can_go: Vec<Vec<[bool; 4]>>,  // [up, down, left, right] for each cell
}

impl Walls {
    fn new(n: usize, v: &[Vec<char>], h: &[Vec<char>]) -> Self {
        let mut can_go = vec![vec![[false; 4]; n]; n];
        
        for i in 0..n {
            for j in 0..n {
                // Up
                can_go[i][j][0] = i > 0 && (i == 1 || h[i - 1][j] != '1');
                // Down
                can_go[i][j][1] = i < n - 1 && h[i][j] != '1';
                // Left
                can_go[i][j][2] = j > 0 && (j == 1 || v[i][j - 1] != '1');
                // Right
                can_go[i][j][3] = j < n - 1 && v[i][j] != '1';
            }
        }
        
        Walls { can_go }
    }
    
    #[inline(always)]
    fn can_move(&self, i: usize, j: usize, dir: u8) -> bool {
        if dir == 4 {
            return true;
        }
        self.can_go[i][j][dir as usize]
    }
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

    let walls = Walls::new(n, &v, &h);
    let mut rng = thread_rng();
    
    // Initialize with better initial solution
    let mut best_state = generate_initial_solution(&mut rng, n, m, k);
    best_state.score = evaluate(&best_state, n, m, &robots, &walls);
    
    // Simulated Annealing
    let mut current_state = best_state.clone();
    let mut temperature = 50.0;
    let mut counter = 0;
    
    while start.elapsed().as_secs_f64() < TIME_LIMIT {
        counter += 1;
        
        // Update temperature based on time
        let progress = start.elapsed().as_secs_f64() / TIME_LIMIT;
        temperature = 50.0 * (1.0 - progress).powf(2.0);
        
        let mut new_state = current_state.clone();
        
        // Random neighbor with biased selection
        let r = rng.gen::<f64>();
        if r < 0.4 {
            // Change a button configuration
            let button = rng.gen_range(0..k);
            let robot = rng.gen_range(0..m);
            new_state.button_config[button][robot] = rng.gen_range(0..5);
        } else if r < 0.7 {
            // Change an operation
            if !new_state.operations.is_empty() {
                let idx = rng.gen_range(0..new_state.operations.len());
                new_state.operations[idx] = rng.gen_range(0..k);
            }
        } else if r < 0.85 {
            // Add operation
            if new_state.operations.len() < MAX_OPS {
                let pos = if new_state.operations.is_empty() {
                    0
                } else {
                    rng.gen_range(0..=new_state.operations.len())
                };
                new_state.operations.insert(pos, rng.gen_range(0..k));
            }
        } else {
            // Remove operation
            if new_state.operations.len() > 20 {
                let idx = rng.gen_range(0..new_state.operations.len());
                new_state.operations.remove(idx);
            }
        }
        
        new_state.score = evaluate(&new_state, n, m, &robots, &walls);
        
        let delta = new_state.score - current_state.score;
        if delta > 0 || rng.gen_bool((delta as f64 / temperature).exp().min(1.0)) {
            current_state = new_state;
            if current_state.score > best_state.score {
                best_state = current_state.clone();
            }
        }
    }
    
    eprintln!("counter: {}, score: {}", counter, best_state.score);
    
    // Output
    const ACTIONS: [char; 5] = ['U', 'D', 'L', 'R', 'S'];
    for row in &best_state.button_config {
        println!("{}", row.iter().map(|&c| ACTIONS[c as usize].to_string()).collect::<Vec<_>>().join(" "));
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
) -> State {
    let mut button_config = vec![vec![4u8; m]; k];
    
    // Set up Roomba-like pattern buttons
    // Button 0: All move right
    for r in 0..m {
        button_config[0][r] = 3;  // Right
    }
    
    // Button 1: All move left
    if k > 1 {
        for r in 0..m {
            button_config[1][r] = 2;  // Left
        }
    }
    
    // Button 2: All move down
    if k > 2 {
        for r in 0..m {
            button_config[2][r] = 1;  // Down
        }
    }
    
    // Button 3: All move up
    if k > 3 {
        for r in 0..m {
            button_config[3][r] = 0;  // Up
        }
    }
    
    // Mixed patterns for remaining buttons
    if k > 4 {
        // Split robots for different movements
        for r in 0..m {
            button_config[4][r] = if r % 2 == 0 { 3 } else { 2 };  // R/L alternating
        }
    }
    
    if k > 5 {
        for r in 0..m {
            button_config[5][r] = if r < m/3 { 1 } else if r < 2*m/3 { 0 } else { 4 };  // D/U/S
        }
    }
    
    // Random for the rest
    for b in 6..k {
        for r in 0..m {
            button_config[b][r] = rng.gen_range(0..5);
        }
    }
    
    // Generate Roomba-like sweep pattern operations
    let mut operations = Vec::new();
    
    // Start with a systematic sweep pattern
    for row in 0..3 {  // Do 3 rows of sweeping
        // Move right across the grid
        for _ in 0..29 {
            operations.push(0);  // Right
        }
        
        // Move down one step
        operations.push(2);  // Down
        
        // Move left across the grid
        for _ in 0..29 {
            operations.push(1);  // Left
        }
        
        // Move down one step
        if row < 2 {
            operations.push(2);  // Down
        }
    }
    
    // Add some random movements to cover remaining areas
    while operations.len() < MAX_OPS.min(180) {
        // Occasionally do vertical movements
        if rng.gen_bool(0.1) {
            operations.push(if rng.gen_bool(0.5) { 2 } else { 3 });  // Down or Up
        } else {
            // Mostly horizontal sweeping
            operations.push(if rng.gen_bool(0.5) { 0 } else { 1 });  // Right or Left
        }
    }
    
    State {
        button_config,
        operations,
        score: 0,
    }
}

#[inline(always)]
fn evaluate(
    state: &State,
    n: usize,
    m: usize,
    robots: &[(usize, usize)],
    walls: &Walls,
) -> i64 {
    let mut visited = vec![0u64; (n * n + 63) / 64];  // Bitset for faster operations
    let mut robot_positions = [0usize; 10];  // Fixed size array for robots
    
    // Pack robot positions into single usize (row * n + col)
    for i in 0..m {
        let pos = robots[i].0 * n + robots[i].1;
        robot_positions[i] = pos;
        visited[pos / 64] |= 1u64 << (pos % 64);
    }
    
    // Simulate operations
    for &button in &state.operations {
        for r in 0..m {
            let pos = robot_positions[r];
            let i = pos / n;
            let j = pos % n;
            let dir = state.button_config[button][r];
            
            if dir < 4 && walls.can_move(i, j, dir) {
                let new_pos = match dir {
                    0 => pos - n,  // Up
                    1 => pos + n,  // Down
                    2 => pos - 1,  // Left
                    3 => pos + 1,  // Right
                    _ => pos,
                };
                robot_positions[r] = new_pos;
                visited[new_pos / 64] |= 1u64 << (new_pos % 64);
            }
        }
    }
    
    // Count visited cells using bit operations
    let mut visited_count = 0;
    for chunk in visited {
        visited_count += chunk.count_ones();
    }
    
    if visited_count == (n * n) as u32 {
        (3 * n * n - state.operations.len()) as i64
    } else {
        visited_count as i64
    }
}