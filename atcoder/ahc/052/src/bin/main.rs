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

use std::collections::{VecDeque, HashSet};

const ACTIONS: [char; 5] = ['U', 'D', 'L', 'R', 'S'];
const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        robots: [(usize, usize); m],
        v: [chars; n],
        h: [chars; n - 1],
    }

    // Find connected regions separated by walls
    let regions = find_regions(n, &v, &h);
    eprintln!("Found {} regions", regions.len());
    
    // Create button configurations for different movement patterns
    let mut button_config = vec![vec!['S'; m]; k];
    
    // Basic directions for all robots
    for dir in 0..4.min(k) {
        for r in 0..m {
            button_config[dir][r] = ACTIONS[dir];
        }
    }
    
    // Mixed patterns for remaining buttons
    if k > 4 {
        // Half robots move right, half left
        for r in 0..m {
            button_config[4][r] = if r < m/2 { 'R' } else { 'L' };
        }
    }
    
    if k > 5 {
        // Half robots move down, half up
        for r in 0..m {
            button_config[5][r] = if r < m/2 { 'D' } else { 'U' };
        }
    }
    
    if k > 6 {
        // Diagonal-like movement
        for r in 0..m {
            button_config[6][r] = match r % 4 {
                0 => 'U',
                1 => 'R',
                2 => 'D',
                _ => 'L',
            };
        }
    }
    
    // Some robots stay, others move
    for b in 7..k {
        for r in 0..m {
            button_config[b][r] = if r % 3 == 0 { 'S' } else { ACTIONS[(r + b) % 4] };
        }
    }
    
    // Generate operations using greedy approach
    let operations = generate_greedy_operations(n, m, k, &robots, &v, &h, &button_config, &regions);
    
    // Output
    for row in &button_config {
        println!("{}", row.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
    }
    
    for op in operations {
        println!("{}", op);
    }
}

fn find_regions(n: usize, v: &[Vec<char>], h: &[Vec<char>]) -> Vec<Vec<(usize, usize)>> {
    let mut visited = vec![vec![false; n]; n];
    let mut regions = Vec::new();
    
    for i in 0..n {
        for j in 0..n {
            if !visited[i][j] {
                let region = bfs_region(i, j, n, &v, &h, &mut visited);
                if !region.is_empty() {
                    regions.push(region);
                }
            }
        }
    }
    
    regions
}

fn bfs_region(
    start_i: usize,
    start_j: usize,
    n: usize,
    v: &[Vec<char>],
    h: &[Vec<char>],
    visited: &mut Vec<Vec<bool>>
) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut region = Vec::new();
    
    queue.push_back((start_i, start_j));
    visited[start_i][start_j] = true;
    
    while let Some((i, j)) = queue.pop_front() {
        region.push((i, j));
        
        // Check all 4 directions
        for d in 0..4 {
            let ni = (i as i32 + DIR[d].0) as usize;
            let nj = (j as i32 + DIR[d].1) as usize;
            
            if ni < n && nj < n && !visited[ni][nj] && can_move_between(i, j, ni, nj, n, v, h) {
                visited[ni][nj] = true;
                queue.push_back((ni, nj));
            }
        }
    }
    
    region
}

fn can_move_between(
    i1: usize, j1: usize,
    i2: usize, j2: usize,
    n: usize,
    v: &[Vec<char>],
    h: &[Vec<char>]
) -> bool {
    if i1 == i2 {
        // Horizontal move
        if j1 < j2 {
            j1 < n - 1 && v[i1][j1] != '1'
        } else {
            j2 < n - 1 && v[i1][j2] != '1'
        }
    } else {
        // Vertical move
        if i1 < i2 {
            i1 < n - 1 && h[i1][j1] != '1'
        } else {
            i2 < n - 1 && h[i2][j1] != '1'
        }
    }
}

fn generate_greedy_operations(
    n: usize,
    m: usize,
    k: usize,
    robots: &[(usize, usize)],
    v: &[Vec<char>],
    h: &[Vec<char>],
    button_config: &[Vec<char>],
    _regions: &[Vec<(usize, usize)>]
) -> Vec<usize> {
    let mut operations = Vec::new();
    let mut visited = vec![vec![false; n]; n];
    let mut robot_positions = robots.to_vec();
    
    // Mark initial positions
    for &(i, j) in &robot_positions {
        visited[i][j] = true;
    }
    
    let max_ops = 1800;
    
    // New systematic approach: priority-based exploration
    while operations.len() < max_ops && count_visited(&visited) < n * n {
        let current_coverage = count_visited(&visited);
        let coverage_ratio = current_coverage as f64 / (n * n) as f64;
        
        let best_button = find_best_systematic_button(
            &robot_positions,
            &visited,
            n,
            k,
            v,
            h,
            button_config,
            coverage_ratio,
            operations.len()
        );
        
        // Apply the button press
        operations.push(best_button);
        let mut new_positions = robot_positions.clone();
        
        for r in 0..m {
            let (ci, cj) = robot_positions[r];
            let action = button_config[best_button][r];
            
            let (ni, nj) = match action {
                'U' if ci > 0 && can_move_between(ci, cj, ci - 1, cj, n, v, h) => (ci - 1, cj),
                'D' if ci < n - 1 && can_move_between(ci, cj, ci + 1, cj, n, v, h) => (ci + 1, cj),
                'L' if cj > 0 && can_move_between(ci, cj, ci, cj - 1, n, v, h) => (ci, cj - 1),
                'R' if cj < n - 1 && can_move_between(ci, cj, ci, cj + 1, n, v, h) => (ci, cj + 1),
                _ => (ci, cj),
            };
            
            new_positions[r] = (ni, nj);
            visited[ni][nj] = true;
        }
        
        robot_positions = new_positions;
    }
    
    eprintln!("Coverage: {}/{}, Operations: {}", count_visited(&visited), n * n, operations.len());
    operations
}

fn find_best_systematic_button(
    robot_positions: &[(usize, usize)],
    visited: &[Vec<bool>],
    n: usize,
    k: usize,
    v: &[Vec<char>],
    h: &[Vec<char>],
    button_config: &[Vec<char>],
    coverage_ratio: f64,
    step: usize
) -> usize {
    let mut best_button = 0;
    let mut best_score = -10000.0;
    
    for button in 0..k {
        let score = evaluate_systematic_button(
            button,
            robot_positions,
            visited,
            n,
            v,
            h,
            button_config,
            coverage_ratio,
            step
        );
        
        if score > best_score {
            best_score = score;
            best_button = button;
        }
    }
    
    best_button
}

fn evaluate_systematic_button(
    button: usize,
    robot_positions: &[(usize, usize)],
    visited: &[Vec<bool>],
    n: usize,
    v: &[Vec<char>],
    h: &[Vec<char>],
    button_config: &[Vec<char>],
    coverage_ratio: f64,
    step: usize
) -> f64 {
    let mut score = 0.0;
    let mut new_cells = 0;
    let mut total_movement = 0;
    
    // Simulate one step
    for r in 0..robot_positions.len() {
        let (ci, cj) = robot_positions[r];
        let action = button_config[button][r];
        
        let (ni, nj) = match action {
            'U' if ci > 0 && can_move_between(ci, cj, ci - 1, cj, n, v, h) => (ci - 1, cj),
            'D' if ci < n - 1 && can_move_between(ci, cj, ci + 1, cj, n, v, h) => (ci + 1, cj),
            'L' if cj > 0 && can_move_between(ci, cj, ci, cj - 1, n, v, h) => (ci, cj - 1),
            'R' if cj < n - 1 && can_move_between(ci, cj, ci, cj + 1, n, v, h) => (ci, cj + 1),
            _ => (ci, cj),
        };
        
        // Count new cells
        if !visited[ni][nj] {
            new_cells += 1;
            score += 100.0; // High reward for new cells
        }
        
        // Count actual movement
        if (ni, nj) != (ci, cj) {
            total_movement += 1;
        }
        
        // Exploration potential - reward being near unvisited areas
        let mut adjacent_unvisited = 0;
        for &(di, dj) in &[(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
            let ai = (ni as i32 + di) as usize;
            let aj = (nj as i32 + dj) as usize;
            if ai < n && aj < n && !visited[ai][aj] {
                adjacent_unvisited += 1;
            }
        }
        score += adjacent_unvisited as f64 * 5.0;
        
        // Penalty for staying in already well-explored areas
        if visited[ni][nj] {
            let mut nearby_visited = 0;
            for &(di, dj) in &[(-2i32, 0i32), (2, 0), (0, -2), (0, 2), (-1, -1), (-1, 1), (1, -1), (1, 1)] {
                let ai = (ni as i32 + di) as usize;
                let aj = (nj as i32 + dj) as usize;
                if ai < n && aj < n && visited[ai][aj] {
                    nearby_visited += 1;
                }
            }
            if nearby_visited > 4 {
                score -= 10.0; // Penalty for crowded areas
            }
        }
    }
    
    // Different strategies based on coverage
    if coverage_ratio < 0.3 {
        // Early phase: reward spreading out
        score += total_movement as f64 * 2.0;
    } else if coverage_ratio < 0.7 {
        // Middle phase: balanced exploration
        score += new_cells as f64 * 50.0;
        if total_movement == 0 {
            score -= 50.0; // Penalty for not moving
        }
    } else {
        // Late phase: focus on uncovered areas
        if new_cells > 0 {
            score += new_cells as f64 * 500.0; // Much higher reward for new cells
        } else {
            score -= 100.0; // Heavy penalty for not finding new cells
        }
        
        // Find all unvisited cells and calculate strategy
        let mut unvisited_cells = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if !visited[i][j] {
                    unvisited_cells.push((i, j));
                }
            }
        }
        
        if !unvisited_cells.is_empty() {
            // Calculate how this button moves robots toward unvisited areas
            let mut total_improvement = 0.0;
            
            for r in 0..robot_positions.len() {
                let (ci, cj) = robot_positions[r];
                let action = button_config[button][r];
                let (ni, nj) = match action {
                    'U' if ci > 0 && can_move_between(ci, cj, ci - 1, cj, n, v, h) => (ci - 1, cj),
                    'D' if ci < n - 1 && can_move_between(ci, cj, ci + 1, cj, n, v, h) => (ci + 1, cj),
                    'L' if cj > 0 && can_move_between(ci, cj, ci, cj - 1, n, v, h) => (ci, cj - 1),
                    'R' if cj < n - 1 && can_move_between(ci, cj, ci, cj + 1, n, v, h) => (ci, cj + 1),
                    _ => (ci, cj),
                };
                
                // Calculate improvement toward closest unvisited cell
                let old_min_dist = unvisited_cells.iter()
                    .map(|&(ui, uj)| ((ci as i32 - ui as i32).abs() + (cj as i32 - uj as i32).abs()) as usize)
                    .min().unwrap_or(n * n);
                
                let new_min_dist = unvisited_cells.iter()
                    .map(|&(ui, uj)| ((ni as i32 - ui as i32).abs() + (nj as i32 - uj as i32).abs()) as usize)
                    .min().unwrap_or(n * n);
                
                if new_min_dist < old_min_dist {
                    total_improvement += (old_min_dist - new_min_dist) as f64 * 10.0;
                } else if new_min_dist > old_min_dist {
                    total_improvement -= (new_min_dist - old_min_dist) as f64 * 5.0;
                }
                
                // Bonus for being very close to unvisited cells
                if new_min_dist <= 1 {
                    total_improvement += 50.0;
                } else if new_min_dist <= 2 {
                    total_improvement += 20.0;
                }
            }
            
            score += total_improvement;
        }
    }
    
    // Add some randomness to prevent getting stuck
    let random_factor = (step * 13 + button * 17) % 100;
    score += (random_factor as f64 - 50.0) * 0.1;
    
    score
}

fn evaluate_button_advanced(
    button: usize,
    robot_positions: &[(usize, usize)],
    visited: &[Vec<bool>],
    n: usize,
    v: &[Vec<char>],
    h: &[Vec<char>],
    button_config: &[Vec<char>],
    phase: usize
) -> i32 {
    let mut score = 0;
    let m = robot_positions.len();
    
    for r in 0..m {
        let (ci, cj) = robot_positions[r];
        let action = button_config[button][r];
        
        let (ni, nj) = match action {
            'U' if ci > 0 && can_move_between(ci, cj, ci - 1, cj, n, v, h) => (ci - 1, cj),
            'D' if ci < n - 1 && can_move_between(ci, cj, ci + 1, cj, n, v, h) => (ci + 1, cj),
            'L' if cj > 0 && can_move_between(ci, cj, ci, cj - 1, n, v, h) => (ci, cj - 1),
            'R' if cj < n - 1 && can_move_between(ci, cj, ci, cj + 1, n, v, h) => (ci, cj + 1),
            _ => (ci, cj),
        };
        
        // High score for visiting new cells
        if !visited[ni][nj] {
            score += 100;
        }
        
        // Look ahead for potential new visits
        let potential_visits = count_potential_visits(ni, nj, n, visited, v, h, 2);
        score += potential_visits * 10;
        
        // Phase-based scoring with more variety
        match phase % 6 {
            0 => {
                // Phase 0: Edge exploration
                if ni == 0 || ni == n-1 || nj == 0 || nj == n-1 {
                    score += 15;
                }
            },
            1 => {
                // Phase 1: Center exploration  
                let center_dist = ((ni as i32 - n as i32 / 2).abs() + (nj as i32 - n as i32 / 2).abs()) as i32;
                score += (n as i32 - center_dist) * 3;
            },
            2 => {
                // Phase 2: Corner focus
                let corner_dist = ni.min(n-1-ni) + nj.min(n-1-nj);
                if corner_dist < 8 {
                    score += 25;
                }
            },
            3 => {
                // Phase 3: Diagonal movement preference
                if (ni + nj) % 2 == 0 {
                    score += 10;
                }
            },
            4 => {
                // Phase 4: Random exploration boost
                score += ((ni * 17 + nj * 23) % 20) as i32;
            },
            _ => {
                // Phase 5: Scattered exploration
                let unvisited_neighbors = count_unvisited_neighbors(ni, nj, n, visited);
                score += (unvisited_neighbors * 20) as i32;
            }
        }
    }
    
    score
}

fn count_potential_visits(
    i: usize,
    j: usize,
    n: usize,
    visited: &[Vec<bool>],
    v: &[Vec<char>],
    h: &[Vec<char>],
    depth: usize
) -> i32 {
    if depth == 0 {
        return 0;
    }
    
    let mut count = 0;
    for &(di, dj) in &DIR {
        let ni = (i as i32 + di) as usize;
        let nj = (j as i32 + dj) as usize;
        
        if ni < n && nj < n && can_move_between(i, j, ni, nj, n, v, h) {
            if !visited[ni][nj] {
                count += 1;
            }
            count += count_potential_visits(ni, nj, n, visited, v, h, depth - 1);
        }
    }
    
    count
}



fn simulate_move(i: usize, j: usize, action: char, steps: usize, n: usize) -> (usize, usize) {
    let mut ci = i;
    let mut cj = j;
    
    for _ in 0..steps {
        let (ni, nj) = match action {
            'U' if ci > 0 => (ci - 1, cj),
            'D' if ci < n - 1 => (ci + 1, cj),
            'L' if cj > 0 => (ci, cj - 1),
            'R' if cj < n - 1 => (ci, cj + 1),
            _ => (ci, cj),
        };
        ci = ni;
        cj = nj;
    }
    
    (ci, cj)
}

fn evaluate_button(
    button: usize,
    robot_positions: &[(usize, usize)],
    visited: &[Vec<bool>],
    n: usize,
    v: &[Vec<char>],
    h: &[Vec<char>],
    button_config: &[Vec<char>]
) -> usize {
    let mut score = 0;
    let m = robot_positions.len();
    
    for r in 0..m {
        let (ci, cj) = robot_positions[r];
        let action = button_config[button][r];
        
        let (ni, nj) = match action {
            'U' if ci > 0 && can_move_between(ci, cj, ci - 1, cj, n, v, h) => (ci - 1, cj),
            'D' if ci < n - 1 && can_move_between(ci, cj, ci + 1, cj, n, v, h) => (ci + 1, cj),
            'L' if cj > 0 && can_move_between(ci, cj, ci, cj - 1, n, v, h) => (ci, cj - 1),
            'R' if cj < n - 1 && can_move_between(ci, cj, ci, cj + 1, n, v, h) => (ci, cj + 1),
            _ => (ci, cj),
        };
        
        // Score based on visiting new cells
        if !visited[ni][nj] {
            score += 10;
        }
        
        // Bonus for moving towards unvisited areas
        let unvisited_neighbors = count_unvisited_neighbors(ni, nj, n, visited);
        score += unvisited_neighbors;
    }
    
    score
}

fn find_exploration_button(
    robot_positions: &[(usize, usize)],
    visited: &[Vec<bool>],
    n: usize,
    k: usize,
    button_config: &[Vec<char>]
) -> usize {
    // Find robots that are stuck and try to move them
    let mut best_button = 0;
    let mut max_potential = 0;
    
    for button in 0..k {
        let mut potential = 0;
        for r in 0..robot_positions.len() {
            let (i, j) = robot_positions[r];
            // Check if this robot is near unvisited areas
            if count_unvisited_neighbors(i, j, n, visited) > 0 {
                potential += 1;
            }
        }
        
        if potential > max_potential {
            max_potential = potential;
            best_button = button;
        }
    }
    
    best_button
}

fn count_unvisited_neighbors(i: usize, j: usize, n: usize, visited: &[Vec<bool>]) -> usize {
    let mut count = 0;
    
    for &(di, dj) in &DIR {
        let ni = (i as i32 + di) as usize;
        let nj = (j as i32 + dj) as usize;
        
        if ni < n && nj < n && !visited[ni][nj] {
            count += 1;
        }
    }
    
    count
}

fn count_visited(visited: &[Vec<bool>]) -> usize {
    visited.iter().flatten().filter(|&&v| v).count()
}

fn find_exhaustive_button(
    robot_positions: &[(usize, usize)],
    visited: &[Vec<bool>],
    n: usize,
    k: usize,
    v: &[Vec<char>],
    h: &[Vec<char>],
    button_config: &[Vec<char>]
) -> usize {
    let mut best_button = 0;
    let mut best_score = 0;
    
    // Try all buttons and find the one that covers the most unvisited cells
    for button in 0..k {
        let mut temp_visited = visited.to_vec();
        let mut temp_positions = robot_positions.to_vec();
        let mut score = 0;
        
        // Simulate multiple steps with this button
        for _ in 0..5 {
            let mut moved = false;
            for r in 0..temp_positions.len() {
                let (ci, cj) = temp_positions[r];
                let action = button_config[button][r];
                
                let (ni, nj) = match action {
                    'U' if ci > 0 && can_move_between(ci, cj, ci - 1, cj, n, v, h) => (ci - 1, cj),
                    'D' if ci < n - 1 && can_move_between(ci, cj, ci + 1, cj, n, v, h) => (ci + 1, cj),
                    'L' if cj > 0 && can_move_between(ci, cj, ci, cj - 1, n, v, h) => (ci, cj - 1),
                    'R' if cj < n - 1 && can_move_between(ci, cj, ci, cj + 1, n, v, h) => (ci, cj + 1),
                    _ => (ci, cj),
                };
                
                if (ni, nj) != (ci, cj) {
                    moved = true;
                }
                
                temp_positions[r] = (ni, nj);
                
                if !temp_visited[ni][nj] {
                    temp_visited[ni][nj] = true;
                    score += 10;
                }
            }
            
            if !moved {
                break;
            }
        }
        
        if score > best_score {
            best_score = score;
            best_button = button;
        }
    }
    
    // If no button produces new coverage, return a random exploration button
    if best_score == 0 {
        find_exploration_button(robot_positions, visited, n, k, button_config)
    } else {
        best_button
    }
}

fn find_aggressive_button(
    robot_positions: &[(usize, usize)],
    visited: &[Vec<bool>],
    n: usize,
    k: usize,
    v: &[Vec<char>],
    h: &[Vec<char>],
    button_config: &[Vec<char>]
) -> usize {
    let mut best_button = 0;
    let mut best_score = 0;
    
    for button in 0..k {
        let mut score = 0;
        
        for r in 0..robot_positions.len() {
            let (ci, cj) = robot_positions[r];
            let action = button_config[button][r];
            
            let (ni, nj) = match action {
                'U' if ci > 0 && can_move_between(ci, cj, ci - 1, cj, n, v, h) => (ci - 1, cj),
                'D' if ci < n - 1 && can_move_between(ci, cj, ci + 1, cj, n, v, h) => (ci + 1, cj),
                'L' if cj > 0 && can_move_between(ci, cj, ci, cj - 1, n, v, h) => (ci, cj - 1),
                'R' if cj < n - 1 && can_move_between(ci, cj, ci, cj + 1, n, v, h) => (ci, cj + 1),
                _ => (ci, cj),
            };
            
            // High score for new cells
            if !visited[ni][nj] {
                score += 20;
            }
            
            // Score for being near unvisited areas
            let unvisited_neighbors = count_unvisited_neighbors(ni, nj, n, visited);
            score += unvisited_neighbors * 2;
            
            // Penalty for staying still when there are unvisited neighbors
            if (ni, nj) == (ci, cj) && count_unvisited_neighbors(ci, cj, n, visited) > 0 {
                score -= 5;
            }
        }
        
        if score > best_score {
            best_score = score;
            best_button = button;
        }
    }
    
    best_button
}

fn find_distant_exploration_button(
    robot_positions: &[(usize, usize)],
    visited: &[Vec<bool>],
    n: usize,
    k: usize,
    v: &[Vec<char>],
    h: &[Vec<char>],
    button_config: &[Vec<char>]
) -> usize {
    // Find unvisited cells that are farthest from current robot positions
    let mut unvisited_cells = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if !visited[i][j] {
                unvisited_cells.push((i, j));
            }
        }
    }
    
    if unvisited_cells.is_empty() {
        return 0;
    }
    
    let mut best_button = 0;
    let mut best_score = 0;
    
    for button in 0..k {
        let mut score = 0;
        
        for r in 0..robot_positions.len() {
            let (ci, cj) = robot_positions[r];
            let action = button_config[button][r];
            
            let (ni, nj) = match action {
                'U' if ci > 0 && can_move_between(ci, cj, ci - 1, cj, n, v, h) => (ci - 1, cj),
                'D' if ci < n - 1 && can_move_between(ci, cj, ci + 1, cj, n, v, h) => (ci + 1, cj),
                'L' if cj > 0 && can_move_between(ci, cj, ci, cj - 1, n, v, h) => (ci, cj - 1),
                'R' if cj < n - 1 && can_move_between(ci, cj, ci, cj + 1, n, v, h) => (ci, cj + 1),
                _ => (ci, cj),
            };
            
            if !visited[ni][nj] {
                score += 15;
            }
            
            // Calculate minimum distance to unvisited cells
            let mut min_dist = n * n;
            for &(ui, uj) in &unvisited_cells {
                let dist = ((ni as i32 - ui as i32).abs() + (nj as i32 - uj as i32).abs()) as usize;
                if dist < min_dist {
                    min_dist = dist;
                }
            }
            
            // Score inversely proportional to distance to nearest unvisited cell
            if min_dist > 0 {
                score += (n * 2) / (min_dist + 1);
            }
        }
        
        if score > best_score {
            best_score = score;
            best_button = button;
        }
    }
    
    best_button
}