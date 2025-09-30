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

const ACTIONS: [char; 5] = ['U', 'D', 'L', 'R', 'S'];

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        robots: [(usize, usize); m],
        v: [chars; n],
        h: [chars; n - 1],
    }

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
            button_config[4][r] = if r < m / 2 { 'R' } else { 'L' };
        }
    }

    if k > 5 {
        // Half robots move down, half up
        for r in 0..m {
            button_config[5][r] = if r < m / 2 { 'D' } else { 'U' };
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
            button_config[b][r] = if r % 3 == 0 {
                'S'
            } else {
                ACTIONS[(r + b) % 4]
            };
        }
    }

    // Run multiple attempts with time limit and choose the best result
    let start_time = std::time::Instant::now();
    let time_limit = std::time::Duration::from_millis(1500);

    let mut best_operations = Vec::new();
    let mut best_coverage = 0;
    let mut best_steps = usize::MAX; // For full coverage solutions, prefer fewer steps
    let mut attempt = 0;

    while start_time.elapsed() < time_limit {
        attempt += 1;

        let (operations, visited) =
            generate_greedy_operations(n, m, k, &robots, &v, &h, &button_config, attempt);
        let coverage = visited.len();
        let is_better = if coverage == n * n && best_coverage == n * n {
            // Both achieve full coverage, prefer fewer steps
            operations.len() < best_steps
        } else {
            // Prefer higher coverage
            coverage > best_coverage
        };

        if is_better {
            best_coverage = coverage;
            best_operations = operations;
            best_steps = best_operations.len();
        }
    }

    eprintln!(
        "Final best coverage: {}/{}, Steps: {} from {} attempts",
        best_coverage,
        n * n,
        best_steps,
        attempt
    );

    // Output
    for row in &button_config {
        println!(
            "{}",
            row.iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }

    for op in best_operations {
        println!("{}", op);
    }
}

fn can_move_between(
    i1: usize,
    j1: usize,
    i2: usize,
    j2: usize,
    n: usize,
    v: &[Vec<char>],
    h: &[Vec<char>],
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
    seed: usize,
) -> (Vec<usize>, std::collections::HashSet<usize>) {
    let mut operations = Vec::new();
    let mut visited = std::collections::HashSet::new();
    let mut robot_positions = robots.to_vec();
    let mut stagnation_count = 0;
    let mut last_coverage = 0;

    // Mark initial positions
    for &(i, j) in &robot_positions {
        visited.insert(i * n + j);
    }

    let max_ops = 1800;

    // New systematic approach: priority-based exploration with reset mechanism
    while operations.len() < max_ops && visited.len() < n * n {
        let current_coverage = visited.len();
        let coverage_ratio = current_coverage as f64 / (n * n) as f64;

        // Check for stagnation (no coverage increase for 20 consecutive steps)
        if current_coverage == last_coverage {
            stagnation_count += 1;
        } else {
            stagnation_count = 0;
            last_coverage = current_coverage;
        }

        // Check if we need to perform random reset
        if stagnation_count >= 20 {
            // Choose a random direction: 0=right-down, 1=right-up, 2=left-down, 3=left-up
            let direction = (operations.len() * 31 + stagnation_count * 17 + seed * 43) % 4;

            // Perform directional movements for about 20 steps
            for step in 0..30 {
                if operations.len() >= max_ops {
                    break;
                }

                // Choose button based on direction and step
                let target_button = match direction {
                    0 => {
                        if step % 2 == 0 {
                            3
                        } else {
                            1
                        }
                    } // R, D alternating
                    1 => {
                        if step % 2 == 0 {
                            3
                        } else {
                            0
                        }
                    } // R, U alternating
                    2 => {
                        if step % 2 == 0 {
                            2
                        } else {
                            1
                        }
                    } // L, D alternating
                    _ => {
                        if step % 2 == 0 {
                            2
                        } else {
                            0
                        }
                    } // L, U alternating
                };

                // Use the target button if it exists, otherwise use a fallback
                let chosen_button = if target_button < k {
                    target_button
                } else {
                    step % k
                };
                operations.push(chosen_button);

                let mut new_positions = robot_positions.clone();
                for r in 0..m {
                    let (ci, cj) = robot_positions[r];
                    let action = button_config[chosen_button][r];

                    let (ni, nj) = match action {
                        'U' if ci > 0 && can_move_between(ci, cj, ci - 1, cj, n, v, h) => {
                            (ci - 1, cj)
                        }
                        'D' if ci < n - 1 && can_move_between(ci, cj, ci + 1, cj, n, v, h) => {
                            (ci + 1, cj)
                        }
                        'L' if cj > 0 && can_move_between(ci, cj, ci, cj - 1, n, v, h) => {
                            (ci, cj - 1)
                        }
                        'R' if cj < n - 1 && can_move_between(ci, cj, ci, cj + 1, n, v, h) => {
                            (ci, cj + 1)
                        }
                        _ => (ci, cj),
                    };

                    new_positions[r] = (ni, nj);
                    visited.insert(ni * n + nj);
                }
                robot_positions = new_positions;
            }

            // Reset the stagnation count
            stagnation_count = 0;
            last_coverage = visited.len();
            continue;
        }

        let best_button = find_best_systematic_button(
            &robot_positions,
            &visited,
            n,
            k,
            v,
            h,
            button_config,
            coverage_ratio,
            operations.len(),
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
            visited.insert(ni * n + nj);
        }

        robot_positions = new_positions;
    }

    (operations, visited)
}

fn find_best_systematic_button(
    robot_positions: &[(usize, usize)],
    visited: &std::collections::HashSet<usize>,
    n: usize,
    k: usize,
    v: &[Vec<char>],
    h: &[Vec<char>],
    button_config: &[Vec<char>],
    coverage_ratio: f64,
    step: usize,
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
            step,
        );

        if score > best_score {
            best_score = score;
            best_button = button;
        }
    }

    // Note: stagnation tracking is now handled in main loop based on coverage changes
    best_button
}

fn evaluate_systematic_button(
    button: usize,
    robot_positions: &[(usize, usize)],
    visited: &std::collections::HashSet<usize>,
    n: usize,
    v: &[Vec<char>],
    h: &[Vec<char>],
    button_config: &[Vec<char>],
    coverage_ratio: f64,
    step: usize,
) -> f64 {
    let mut new_cells = 0;
    let mut total_movement = 0;
    let mut new_positions = Vec::new();

    let mut new_cell_score = 0.0;
    let mut adjacent_unvisited_score = 0.0;
    let mut crowding_score = 0.0;
    let mut overlap_score = 0.0;
    let mut diversity_score = 0.0;
    let mut coverage_score = 0.0;

    // Simulate one step and collect new positions
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

        new_positions.push((ni, nj));

        // Count new cells
        if !visited.contains(&(ni * n + nj)) {
            new_cells += 1;
            new_cell_score += 100.0; // High reward for new cells
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
            if ai < n && aj < n && !visited.contains(&(ai * n + aj)) {
                adjacent_unvisited += 1;
            }
        }
        adjacent_unvisited_score += adjacent_unvisited as f64;

        // Penalty for staying in already well-explored areas
        if visited.contains(&(ni * n + nj)) {
            let mut nearby_visited = 0;
            for &(di, dj) in &[
                (-2i32, 0i32),
                (2, 0),
                (0, -2),
                (0, 2),
                (-1, -1),
                (-1, 1),
                (1, -1),
                (1, 1),
            ] {
                let ai = (ni as i32 + di) as usize;
                let aj = (nj as i32 + dj) as usize;
                if ai < n && aj < n && visited.contains(&(ai * n + aj)) {
                    nearby_visited += 1;
                }
            }
            if nearby_visited > 4 {
                crowding_score -= 10.0; // Penalty for crowded areas
            }
        }
    }

    // Moderate penalty for robot overlap - but only when not productive
    let mut overlap_penalty = 0.0;
    for i in 0..new_positions.len() {
        let (i1, j1) = new_positions[i];
        for j in (i + 1)..new_positions.len() {
            let (i2, j2) = new_positions[j];
            if (i1, j1) == (i2, j2) {
                // Same position - heavy penalty only if both robots are not visiting new cells
                let both_on_visited =
                    visited.contains(&(i1 * n + j1)) && visited.contains(&(i2 * n + j2));
                if both_on_visited {
                    overlap_penalty += 100.0;
                } else {
                    overlap_penalty += 30.0; // Lighter penalty if at least one is exploring
                }
            } else {
                let dist = ((i1 as i32 - i2 as i32).abs() + (j1 as i32 - j2 as i32).abs()) as usize;
                if dist == 1 && visited.contains(&(i1 * n + j1)) && visited.contains(&(i2 * n + j2))
                {
                    // Adjacent positions - penalty only if both on visited cells
                    overlap_penalty += 15.0;
                }
            }
        }
    }
    overlap_score -= overlap_penalty;

    // Reward robot diversity in actions
    let mut unique_actions = std::collections::HashSet::new();
    for r in 0..robot_positions.len() {
        let action = button_config[button][r];
        unique_actions.insert(action);
    }
    if unique_actions.len() >= 3 {
        diversity_score += 20.0; // Bonus for diverse movements
    }

    // Different strategies based on coverage
    if coverage_ratio < 0.3 {
        // Early phase: reward spreading out
        coverage_score += total_movement as f64 * 2.0;
    } else if coverage_ratio < 0.7 {
        // Middle phase: balanced exploration
        coverage_score += new_cells as f64 * 50.0;
        if total_movement == 0 {
            coverage_score -= 50.0; // Penalty for not moving
        }
    } else {
        // Late phase: focus on uncovered areas
        if new_cells > 0 {
            coverage_score += new_cells as f64 * 500.0; // Much higher reward for new cells
        } else {
            coverage_score -= 100.0; // Heavy penalty for not finding new cells
        }

        // Find all unvisited cells and calculate strategy
        let mut unvisited_cells = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if !visited.contains(&(i * n + j)) {
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
                    'D' if ci < n - 1 && can_move_between(ci, cj, ci + 1, cj, n, v, h) => {
                        (ci + 1, cj)
                    }
                    'L' if cj > 0 && can_move_between(ci, cj, ci, cj - 1, n, v, h) => (ci, cj - 1),
                    'R' if cj < n - 1 && can_move_between(ci, cj, ci, cj + 1, n, v, h) => {
                        (ci, cj + 1)
                    }
                    _ => (ci, cj),
                };

                // Calculate improvement toward closest unvisited cell
                let old_min_dist = unvisited_cells
                    .iter()
                    .map(|&(ui, uj)| {
                        ((ci as i32 - ui as i32).abs() + (cj as i32 - uj as i32).abs()) as usize
                    })
                    .min()
                    .unwrap_or(n * n);

                let new_min_dist = unvisited_cells
                    .iter()
                    .map(|&(ui, uj)| {
                        ((ni as i32 - ui as i32).abs() + (nj as i32 - uj as i32).abs()) as usize
                    })
                    .min()
                    .unwrap_or(n * n);

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

            coverage_score += total_improvement / 2.0;
        }
    }

    // Add some randomness to prevent getting stuck with different seeds per attempt
    let random_factor = (step * 13 + button * 17 + (step * 7) % 101) % 100;
    let random_factor_score = (random_factor as f64 - 50.0) * 0.1;
    // eprintln!(
    //     "new_cell_score: {} adjacent_unvisited_score: {} crowding_score: {} overlap_score: {} diversity_score: {} coverage_score: {} random_factor_score: {}",
    //     new_cell_score,
    //     adjacent_unvisited_score,
    //     crowding_score,
    //     overlap_score,
    //     diversity_score, coverage_score, random_factor_score
    // );
    new_cell_score
        + adjacent_unvisited_score
        + crowding_score
        + overlap_score
        + diversity_score
        + coverage_score
        + random_factor_score
}
