use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];
const ARROUND: [(usize, usize); 8] = [
    (!0, 0),
    (1, 0),
    (0, !0),
    (0, 1),
    (!0, !0),
    (1, !0),
    (!0, 1),
    (1, 1),
];
// 上下左右に木があるかどうか
fn exist_dij_tree(bss: &Vec<Vec<char>>, ti: usize, tj: usize) -> bool {
    for (di, dj) in DIJ {
        let ni = ti + di;
        let nj = tj + dj;
        if ni < bss.len() && nj < bss[0].len() {
            if bss[ni][nj] == 'T' {
                return true;
            }
        }
    }
    false
}

fn select_next_place(
    n: usize,
    confirmed: &HashSet<(usize, usize)>,
    p: (usize, usize),
    t: (usize, usize),
    will_place: &mut HashSet<(usize, usize)>,
    already_placed: &HashSet<(usize, usize)>,
    before_place: Option<(usize, usize)>,
    empty_arround_flower: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    if will_place.is_empty() {
        return Vec::new();
    }

    let (pi, pj) = p;
    let mut next_place = Vec::new();
    // 前回の移動方向から、今回スキャンする方向を決定
    let (scan_vertical, scan_horizontal) = match before_place {
        Some((prev_i, _)) if prev_i == pi => (true, false), // 横移動してきた→縦スキャン
        Some(_) => (false, true),                           // 縦移動してきた→横スキャン
        None => (true, true),                               // 初回→全方向スキャン
    };

    // 共通のスキャン処理
    let mut scan_line = |positions: Vec<(usize, usize)>| {
        for pos in positions {
            if already_placed.contains(&pos) {
                break;
            }
            if !confirmed.contains(&pos) {
                if will_place.contains(&pos) {
                    will_place.remove(&pos);
                    next_place.push(pos);
                    break;
                }
            }
        }
    };
    // 縦方向スキャン
    if scan_vertical {
        scan_line((pi..n).map(|i| (i, pj)).collect());
        scan_line((0..pi).rev().map(|i| (i, pj)).collect());
    }

    // 横方向スキャン
    if scan_horizontal {
        scan_line((pj..n).map(|j| (pi, j)).collect());
        scan_line((0..pj).rev().map(|j| (pi, j)).collect());
    }
    next_place
}

fn make_empty_arround_flower(
    n: usize,
    confirmed: &HashSet<(usize, usize)>,
    already_placed: &HashSet<(usize, usize)>,
    will_place: &HashSet<(usize, usize)>,
    t: (usize, usize),
) -> HashSet<(usize, usize)> {
    let (ti, tj) = t;
    let mut empty_arround_flower = HashSet::new();
    for (di, dj) in ARROUND {
        let mut iter = 0;
        let mut ni = ti;
        let mut nj = tj;
        while ni < n && nj < n && iter < 1 {
            if !confirmed.contains(&(ni, nj)) && !already_placed.contains(&(ni, nj))
            // && !will_place.contains(&(ni, nj))
            {
                empty_arround_flower.insert((ni, nj));
            } else {
                break;
            }
            ni += di;
            nj += dj;
            iter += 1;
        }
    }
    empty_arround_flower.remove(&(ti, tj));
    empty_arround_flower
}

fn make_habitat_place(bss: &Vec<Vec<char>>, t: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut habitat_place = HashSet::new();
    let mut visited = HashSet::new();
    visited.insert(t);
    for (di, dj) in ARROUND {
        let ni = t.0 + di;
        let nj = t.1 + dj;
        visited.insert((ni, nj));
        if ni < bss.len() && nj < bss.len() && bss[ni][nj] == 'T' {
            for (di, dj) in ARROUND {
                let ni2 = ni + di;
                let nj2 = nj + dj;
                if ni2 < bss.len() && nj2 < bss.len() && bss[ni2][nj2] == '.' {
                    habitat_place.insert((ni2, nj2));
                }
            }
            queue.push_back((ni, nj));
        }
    }
    while let Some((i, j)) = queue.pop_front() {
        if visited.contains(&(i, j)) {
            continue;
        }
        visited.insert((i, j));
        for (di, dj) in ARROUND {
            let ni = i + di;
            let nj = j + dj;
            if ni < bss.len() && nj < bss.len() && bss[ni][nj] == 'T' {
                for (di, dj) in ARROUND {
                    let ni2 = ni + di;
                    let nj2 = nj + dj;
                    if ni2 < bss.len() && nj2 < bss.len() && bss[ni2][nj2] == '.' {
                        habitat_place.insert((ni2, nj2));
                    }
                }
                queue.push_back((ni, nj));
            }
        }
    }
    habitat_place
}

fn exist_around_tree(bss: &Vec<Vec<char>>, ti: usize, tj: usize) -> bool {
    for (di, dj) in ARROUND {
        let ni = ti + di;
        let nj = tj + dj;
        if ni < bss.len() && nj < bss[0].len() {
            if bss[ni][nj] == 'T' {
                return true;
            }
        }
    }
    false
}

fn put_diagonal(
    bss: &mut Vec<Vec<char>>,
    confirmed: &HashSet<(usize, usize)>,
    ti: usize,
    tj: usize,
    direction: i64,
    t: (usize, usize),
) -> HashSet<(usize, usize)> {
    let original_bss = bss.clone();
    let n = bss.len() as i64;
    let mut treant_placements = HashSet::new();
    let mut tmp_ti = ti as i64;
    let mut tmp_tj = tj as i64;
    while tmp_ti < n && tmp_tj < n && tmp_ti >= 0 && tmp_tj >= 0 {
        if bss[tmp_ti as usize][tmp_tj as usize] == '.'
            && !confirmed.contains(&(tmp_ti as usize, tmp_tj as usize))
            && !exist_dij_tree(&original_bss, tmp_ti as usize, tmp_tj as usize)
            && (tmp_ti as usize, tmp_tj as usize) != t
        {
            bss[tmp_ti as usize][tmp_tj as usize] = 'T';
            treant_placements.insert((tmp_ti as usize, tmp_tj as usize));
        }
        tmp_ti += direction;
        tmp_tj += 1;
    }

    treant_placements
}

// 周囲に木がないところにひたすら置いていく
fn put_tree_around_no_tree(
    bss: &mut Vec<Vec<char>>,
    confirmed: &HashSet<(usize, usize)>,
    ti: usize,
    tj: usize,
) -> Vec<(usize, usize)> {
    let mut treant_placements = Vec::new();
    for i in 0..bss.len() {
        for j in 0..bss.len() {
            if bss[i][j] == '.'
                && i != ti
                && j != tj
                && !confirmed.contains(&(i, j))
                && !exist_around_tree(bss, i, j)
            {
                bss[i][j] = 'T';
                treant_placements.push((i, j));
            }
        }
    }
    treant_placements
}

fn can_reach_goals(
    bss: &Vec<Vec<char>>,
    start_ti: usize,
    start_tj: usize,
    goals: &[(usize, usize)],
) -> bool {
    let n = bss.len();
    let goal_set: HashSet<_> = goals.iter().cloned().collect();
    let mut remaining_goals = goal_set.clone();

    if remaining_goals.is_empty() {
        return true;
    }

    let mut queue = VecDeque::new();
    queue.push_back((start_ti, start_tj, 0));
    let mut visited = HashSet::new();
    visited.insert((start_ti, start_tj));

    remaining_goals.remove(&(start_ti, start_tj));
    while !queue.is_empty() && !remaining_goals.is_empty() {
        let (i, j, steps) = queue.pop_front().unwrap();

        for (di, dj) in DIJ {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);

            if ni < n && nj < n && bss[ni][nj] != 'T' && visited.insert((ni, nj)) {
                remaining_goals.remove(&(ni, nj));
                queue.push_back((ni, nj, steps + 1));
            }
        }
    }
    remaining_goals.is_empty()
}
fn surround_flower(
    bss: &mut Vec<Vec<char>>,
    confirmed: &HashSet<(usize, usize)>,
    t: (usize, usize),
    must_reach: &Vec<(usize, usize)>,
    surround_placements: &Vec<Vec<(i64, i64)>>,
) -> HashSet<(usize, usize)> {
    let n: usize = bss.len();

    let mut best_bss = bss.clone();
    let mut best_treant_placements = HashSet::new();
    for candidate in surround_placements {
        let mut treant_placements = HashSet::new();
        let mut tmp_bss = bss.clone();
        for (di, dj) in candidate {
            let ni = t.0 as i64 + di;
            let nj = t.1 as i64 + dj;
            if ni < 0 || nj < 0 {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if ni < n && nj < n {
                if tmp_bss[ni][nj] == '.' && !confirmed.contains(&(ni, nj)) {
                    tmp_bss[ni][nj] = 'T';
                    treant_placements.insert((ni, nj));
                }
            }
        }
        if can_reach_goals(&tmp_bss, 0, n / 2, must_reach) {
            best_treant_placements = treant_placements;
            best_bss = tmp_bss;
            break;
        }
    }
    *bss = best_bss;

    best_treant_placements
}
fn not_around_goal(p: (usize, usize), t: (usize, usize)) -> bool {
    for (di, dj) in DIJ {
        let ni = t.0 + di;
        let nj = t.1 + dj;
        if ni == p.0 && nj == p.1 {
            return false;
        }
    }
    true
}
fn delete_three_tree(
    bss: &mut Vec<Vec<char>>,
    placements: &mut Vec<(usize, usize)>,
    t: (usize, usize),
) {
    let n = bss.len();
    for i in 0..n {
        for j in 0..n - 2 {
            if bss[i][j] == 'T'
                && bss[i][j + 1] == 'T'
                && bss[i][j + 2] == 'T'
                && not_around_goal((i, j + 1), t)
                && placements.contains(&(i, j + 1))
            {
                bss[i][j + 1] = '.';
                placements.retain(|(x, y)| *x != i || *y != j + 1);
            }
        }
    }
}
fn init_tree(
    bss: &mut Vec<Vec<char>>,
    confirmed: &HashSet<(usize, usize)>,
    start_pos: usize,
    direction: i64,
    t: (usize, usize),
) -> HashSet<(usize, usize)> {
    let n = bss.len();
    let (ti, tj) = t;
    let mut treant_placements = HashSet::new();
    // 3つ飛ばしで斜めに線を引く
    for i in (0..n).step_by(3) {
        treant_placements.extend(put_diagonal(
            bss,
            &confirmed,
            0,
            i + start_pos,
            direction,
            t,
        ));
    }
    for i in (0..n).step_by(3) {
        treant_placements.extend(put_diagonal(
            bss,
            &confirmed,
            i + 3 - start_pos,
            0,
            direction,
            t,
        ));
    }

    treant_placements
}
fn solve(
    start_time: std::time::Instant,
    bss: &Vec<Vec<char>>,
    confirmed: &HashSet<(usize, usize)>,
    pi: usize,
    pj: usize,
    ti: usize,
    tj: usize,
) -> HashSet<(usize, usize)> {
    // Generate q: all cells except entrance in random order
    use rand::prelude::*;
    let mut rng = thread_rng();
    let mut q = Vec::new();
    let n = bss.len();
    let entrance = (0, n / 2);
    for i in 0..n {
        for j in 0..n {
            if (i, j) != entrance {
                q.push((i, j));
            }
        }
    }
    q.shuffle(&mut rng);

    let surround_candidates: Vec<Vec<(i64, i64)>> = if tj >= n / 2 {
        vec![
            vec![(1, 0), (0, -1), (-1, 0), (-1, 1), (0, 2)], // 横右下
            vec![(0, -1), (-1, 0), (0, 1), (1, -1), (2, 0)], // 縦右下
            vec![(-1, 0), (0, 1), (1, 0), (-1, -1), (0, -2)], // 横左下
            vec![(0, -1), (-1, 0), (0, 1), (1, 1), (2, 0)],  // 縦左下
            vec![(0, 1), (1, 0), (0, -1), (-1, 1), (-2, 0)], // 縦右上
            vec![(1, 0), (0, -1), (-1, 0), (1, 1), (0, 2)],  // 横右上
            vec![(-1, 0), (0, 1), (1, 0), (1, -1), (0, -2)], // 横左上
            vec![(0, 1), (1, 0), (0, -1), (-1, -1), (-2, 0)], // 縦左上
        ]
    } else {
        vec![
            vec![(-1, 0), (0, 1), (1, 0), (-1, -1), (0, -2)], // 横左下
            vec![(0, -1), (-1, 0), (0, 1), (1, 1), (2, 0)],   // 縦左下
            vec![(1, 0), (0, -1), (-1, 0), (-1, 1), (0, 2)],  // 横右下
            vec![(0, -1), (-1, 0), (0, 1), (1, -1), (2, 0)],  // 縦右下
            vec![(-1, 0), (0, 1), (1, 0), (1, -1), (0, -2)],  // 横左上
            vec![(0, 1), (1, 0), (0, -1), (-1, -1), (-2, 0)], // 縦左上
            vec![(0, 1), (1, 0), (0, -1), (-1, 1), (-2, 0)],  // 縦右上
            vec![(1, 0), (0, -1), (-1, 0), (1, 1), (0, 2)],   // 横右上
        ]
    };
    let mut surround_bss: Vec<Vec<char>> = bss.clone();
    let surround_placements = surround_flower(
        &mut surround_bss,
        &confirmed,
        (ti, tj),
        &vec![(ti, tj)],
        &surround_candidates,
    );

    let mut best_placements = HashSet::new();
    let mut best_score = 0;
    let mut best_bss: Vec<Vec<char>> = bss.clone();
    for i in 0..3 {
        let mut tmp_bss = surround_bss.clone();
        let mut treant_placements = init_tree(&mut tmp_bss, &confirmed, i, 1, (ti, tj));
        treant_placements.extend(surround_placements.clone());
        let mut uf = UnionFind::new(n * n);
        for i in 0..n - 1 {
            for j in 0..n - 1 {
                if tmp_bss[i][j] == '.' {
                    if tmp_bss[i + 1][j] == '.' {
                        uf.union(i * n + j, (i + 1) * n + j);
                    }
                    if tmp_bss[i][j + 1] == '.' {
                        uf.union(i * n + j, i * n + j + 1);
                    }
                }
            }
        }
        for i in 0..n {
            for j in 0..n {
                if !treant_placements.contains(&(i, j)) || surround_placements.contains(&(i, j)) {
                    continue;
                }
                if (i > 0
                    && i < n - 1
                    && !uf.is_same((i + 1) * n + j, (i - 1) * n + j)
                    && uf.get_size((i + 1) * n + j) > 1
                    && uf.get_size((i - 1) * n + j) > 1)
                    || (j > 0
                        && j < n - 1
                        && !uf.is_same(i * n + j + 1, i * n + j - 1)
                        && uf.get_size(i * n + j + 1) > 1
                        && uf.get_size(i * n + j - 1) > 1)
                {
                    if i > 0 && i < n - 1 {
                        uf.union((i + 1) * n + j, (i - 1) * n + j);
                    }
                    if j > 0 && j < n - 1 {
                        uf.union(i * n + j + 1, i * n + j - 1);
                    }
                    tmp_bss[i][j] = '.';
                    treant_placements.remove(&(i, j));
                }
            }
        }
        let score = simulate(&tmp_bss, pi, pj, (ti, tj), q.clone());
        if score > best_score {
            best_score = score;
            best_placements = treant_placements;
            best_bss = tmp_bss;
        }
    }
    best_placements
}

pub struct Sim {
    pub n: usize,
    pub b: Vec<char>,
    pub p: (usize, usize),
    pub t: (usize, usize),
    pub target: (usize, usize),
    pub revealed: Vec<bool>,
    pub new_revealed: Vec<(usize, usize)>,
    pub dist: Vec<i32>,
    pub q: Vec<(usize, usize)>,
}

impl Sim {
    pub fn new(n: usize, bss: Vec<Vec<char>>, t: (usize, usize), q: Vec<(usize, usize)>) -> Self {
        let mut revealed = vec![false; n * n];
        revealed[n / 2] = true;
        Self {
            n: n,
            b: bss.iter().flatten().copied().collect(),
            p: (0, n / 2),
            t: t,
            target: (!0, !0),
            revealed,
            new_revealed: vec![(0, n / 2)],
            dist: vec![0; n * n],
            q: q,
        }
    }
    fn change_target(&mut self, target: (usize, usize)) {
        if self.target == target {
            return;
        }
        self.target = target;
        if target == (!0, !0) {
            return;
        }
        let dist = &mut self.dist;
        dist.fill(i32::MAX);
        let mut que = vec![target];
        let mut qs = 0;
        dist[target.0 * self.n + target.1] = 0;
        while qs < que.len() {
            let (i, j) = que[qs];
            qs += 1;
            for (di, dj) in DIJ {
                let i2 = i + di;
                let j2 = j + dj;
                if i2 < self.n
                    && j2 < self.n
                    && dist[i2 * self.n + j2] == i32::MAX
                    && (!self.revealed[i2 * self.n + j2] || self.b[i2 * self.n + j2] == '.')
                {
                    dist[i2 * self.n + j2] = dist[i * self.n + j] + 1;
                    que.push((i2, j2));
                }
            }
        }
    }
    pub fn step(&mut self, xy: &[(usize, usize)]) -> Result<(), String> {
        self.new_revealed.clear();
        if self.p == self.t {
            return Err("Too many outputs".to_owned());
        }
        for &(i, j) in xy {
            if self.revealed[i * self.n + j] {
                return Err(format!("({}, {}) is already revealed", i, j));
            } else if self.b[i * self.n + j] != '.' {
                return Err(format!("({}, {}) is not empty", i, j));
            } else if (i, j) == self.t {
                return Err(format!("({}, {}) contains the flower", i, j));
            }
            self.b[i * self.n + j] = 't';
        }
        let mut changed = false;
        for (di, dj) in DIJ {
            let mut i2: usize = self.p.0;
            let mut j2 = self.p.1;
            while i2 < self.n && j2 < self.n {
                if self.revealed[i2 * self.n + j2].setmax(true) {
                    self.new_revealed.push((i2, j2));
                    if self.b[i2 * self.n + j2] != '.' {
                        changed = true;
                    }
                }
                if self.b[i2 * self.n + j2] != '.' {
                    break;
                }
                i2 += di;
                j2 += dj;
            }
        }
        if changed {
            let target = self.target;
            self.target = (!0, !0);
            self.change_target(target);
        }
        if self.revealed[self.t.0 * self.n + self.t.1] {
            self.change_target(self.t);
        }
        if self.target != (!0, !0) && self.dist[self.p.0 * self.n + self.p.1] == i32::MAX {
            self.target = (!0, !0);
        }
        if self.target == (!0, !0)
            || self.target != self.t && self.revealed[self.target.0 * self.n + self.target.1]
        {
            self.change_target(self.p);
            loop {
                if let Some(target) = self.q.pop() {
                    if !self.revealed[target.0 * self.n + target.1]
                        && self.dist[target.0 * self.n + target.1] != i32::MAX
                    {
                        self.change_target(target);
                        break;
                    }
                } else {
                    return Err(format!("Not reachable"));
                }
            }
        }
        let mut min = i32::MAX;
        let mut next_dir = !0;
        for dir in 0..4 {
            let i2 = self.p.0 + DIJ[dir].0;
            let j2 = self.p.1 + DIJ[dir].1;
            if i2 < self.n && j2 < self.n && min.setmin(self.dist[i2 * self.n + j2]) {
                next_dir = dir;
            }
        }
        assert!(next_dir != !0);
        self.p.0 += DIJ[next_dir].0;
        self.p.1 += DIJ[next_dir].1;
        Ok(())
    }
}

fn simulate(
    bss: &Vec<Vec<char>>,
    start_ti: usize,
    start_tj: usize,
    flower_pos: (usize, usize),
    q: Vec<(usize, usize)>,
) -> usize {
    let mut sim = Sim::new(bss.len(), bss.clone(), flower_pos, q);
    sim.p = (start_ti, start_tj);
    sim.revealed.fill(false);
    sim.revealed[start_ti * bss.len() + start_tj] = true;

    let mut steps = 0;
    loop {
        if sim.p == flower_pos {
            return steps;
        }

        // Simulate one step with no treant placements
        if let Err(_) = sim.step(&[]) {
            return 0; // Cannot continue
        }
        steps += 1;
    }
}

fn main() {
    use std::io::{self, BufRead, Write};
    let start_time = std::time::Instant::now();
    // Read initial input manually
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read N, ti, tj
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.trim().split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let ti: usize = parts.next().unwrap().parse().unwrap();
    let tj: usize = parts.next().unwrap().parse().unwrap();

    // Read forest grid
    let mut bss: Vec<Vec<char>> = Vec::new();
    let mut already_placed = HashSet::new();
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<char> = line.trim().chars().collect();
        for (j, c) in row.clone().iter().enumerate() {
            if *c == 'T' {
                already_placed.insert((i, j));
            }
        }
        bss.push(row);
    }

    let mut turn = 0;
    let mut will_place = HashSet::new();
    let mut empty_arround_flower = HashSet::new();
    let mut before_place = None;
    let mut confirmed: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.trim().split_whitespace();
        let pi: usize = parts.next().unwrap().parse().unwrap();
        let pj: usize = parts.next().unwrap().parse().unwrap();

        let line = lines.next().unwrap().unwrap();
        let mut parts = line.trim().split_whitespace();
        let num_confirmed: usize = parts.next().unwrap().parse().unwrap();

        for _ in 0..num_confirmed {
            let x: usize = parts.next().unwrap().parse().unwrap();
            let y: usize = parts.next().unwrap().parse().unwrap();
            confirmed.insert((x, y));
        }

        // Check if adventurer reached the flower
        if pi == ti && pj == tj {
            break;
        }
        if turn == 0 {
            will_place = solve(start_time, &bss, &confirmed, pi, pj, ti, tj);
        }

        let elapsed_ms = start_time.elapsed().as_millis();
        let next_place = if elapsed_ms < 1900 {
            select_next_place(
                n,
                &confirmed,
                (pi, pj),
                (ti, tj),
                &mut will_place,
                &already_placed,
                before_place,
                &empty_arround_flower,
            )
        } else {
            Vec::new()
        };

        print!("{}", next_place.len());
        for (i, j) in next_place {
            already_placed.insert((i, j));
            print!(" {} {}", i, j);
        }
        println!();
        before_place = Some((pi, pj));
        turn += 1;
        io::stdout().flush().unwrap();
    }
}

fn print_bss(bss: &Vec<Vec<char>>) {
    for row in bss.iter() {
        for &cell in row.iter() {
            eprint!("{}", cell);
        }
        eprintln!();
    }
    eprintln!();
}
pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}
#[derive(Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    /// 新しいUnion-Find構造を作成
    /// n: 要素数
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    /// 要素xが属する集合の代表元を探索（経路圧縮付き）
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// 要素xとyが属する集合を併合
    /// 返り値: 併合が行われたらtrue、既に同じ集合ならfalse
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        // ランクによる併合（低い木を高い木にマージ）
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
                self.size[root_x] += self.size[root_y];
            }
        }
        true
    }

    /// 要素xとyが同じ集合に属しているか判定
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// 要素xが属する集合のサイズを返す
    pub fn get_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    /// 連結成分の数を返す
    pub fn count_groups(&mut self) -> usize {
        (0..self.parent.len())
            .filter(|&i| self.find(i) == i)
            .count()
    }
}
