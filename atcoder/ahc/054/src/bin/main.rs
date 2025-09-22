use std::collections::{HashSet, VecDeque};

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
) -> Vec<(usize, usize)> {
    let original_bss = bss.clone();
    let n = bss.len() as i64;
    let mut treant_placements = Vec::new();
    let mut tmp_ti = ti as i64;
    let mut tmp_tj = tj as i64;
    while tmp_ti < n - 1 && tmp_tj < n - 1 && tmp_ti >= 0 && tmp_tj >= 0 {
        if bss[tmp_ti as usize][tmp_tj as usize] == '.'
            && !confirmed.contains(&(tmp_ti as usize, tmp_tj as usize))
            && !exist_around_tree(&original_bss, tmp_ti as usize, tmp_tj as usize)
        {
            if (tmp_ti == n - 2 || tmp_tj == n - 2)
                && bss[(tmp_ti + direction) as usize][(tmp_tj + direction) as usize] == 'T'
            {
                return treant_placements;
            }
            bss[tmp_ti as usize][tmp_tj as usize] = 'T';
            treant_placements.push((tmp_ti as usize, tmp_tj as usize));
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
) -> Vec<(usize, usize)> {
    let n: usize = bss.len();

    let mut best_bss = bss.clone();
    let mut best_treant_placements = Vec::new();
    for candidate in surround_placements {
        let mut treant_placements = Vec::new();
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
                    treant_placements.push((ni, nj));
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

    return best_treant_placements;
}

fn init_tree(
    bss: &mut Vec<Vec<char>>,
    confirmed: &HashSet<(usize, usize)>,
    start_pos: usize,
    direction: i64,
) -> Vec<(usize, usize)> {
    let n = bss.len();
    let mut treant_placements = Vec::new();
    // 3つ飛ばしで斜めに線を引く
    for i in (0..n).step_by(3) {
        treant_placements.extend(put_diagonal(bss, &confirmed, 0, i + start_pos, direction));
    }
    for i in (0..n).step_by(3) {
        treant_placements.extend(put_diagonal(
            bss,
            &confirmed,
            i + 3 - start_pos,
            0,
            direction,
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
) -> Vec<(usize, usize)> {
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
    let surround_placements: Vec<Vec<(i64, i64)>> = if tj >= n / 2 {
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

    let mut best_bss: Vec<Vec<char>> = bss.clone();
    let mut best_placements: Vec<(usize, usize)> = Vec::new();
    let mut best_score = 0;

    let ti_mod = 3;
    let tj_mod: usize = 5;
    let mut tmp_bss: Vec<Vec<char>> = bss.clone();
    let mut must_reach: Vec<(usize, usize)> = vec![(ti, tj)];
    let mut tmp_ti = (ti + ((n - 1 - ti) / ti_mod) * ti_mod) as i64;
    while tmp_ti >= 0 {
        let mut tmp_tj = tj % tj_mod;
        while tmp_tj < n {
            if bss[tmp_ti as usize][tmp_tj] == '.' {
                must_reach.push((tmp_ti as usize, tmp_tj));
            }
            tmp_tj += tj_mod;
        }
        tmp_ti -= ti_mod as i64;
    }
    let mut tmp_placements = surround_flower(
        &mut tmp_bss,
        &confirmed,
        (ti, tj),
        &must_reach,
        &surround_placements,
    );
    let mut tmp_ti = (ti + ((n - 1 - ti) / ti_mod) * ti_mod) as i64;
    while tmp_ti >= 0 {
        let mut tmp_tj = tj % tj_mod;
        while tmp_tj < n {
            if tmp_tj == tj && (tmp_ti == (ti + ti_mod) as i64 || tmp_ti == (ti - ti_mod) as i64) {
                tmp_tj += tj_mod;
                continue;
            }
            if bss[tmp_ti as usize][tmp_tj] == '.' {
                let placements = surround_flower(
                    &mut tmp_bss,
                    &confirmed,
                    (tmp_ti as usize, tmp_tj),
                    &must_reach,
                    &surround_placements,
                );
                tmp_placements.extend(placements);
            }
            tmp_tj += tj_mod;
        }
        tmp_ti -= ti_mod as i64;
    }
    let score = simulate(&tmp_bss, pi, pj, (ti, tj), q.clone());
    if score > best_score {
        best_score = score;
        best_placements = tmp_placements;
        best_bss = tmp_bss;
    }

    best_placements.extend(put_tree_around_no_tree(&mut best_bss, &confirmed, ti, tj));
    let mut iteration = 0;
    let mut empty_placements = HashSet::new();
    for i in 0..bss.len() {
        for j in 0..bss.len() {
            if best_bss[i][j] == '.' && !confirmed.contains(&(i, j)) && !(i == ti && j == tj) {
                empty_placements.insert((i, j));
            }
        }
    }
    while start_time.elapsed() < std::time::Duration::from_millis(1900) {
        let mut new_bss = best_bss.clone();
        let mut new_placements = best_placements.clone();
        let mut new_empty_placements = empty_placements.clone();

        // 近傍操作: トレントを1つ追加/削除/移動
        let operation = rng.gen_range(0..2);
        match operation {
            // 0 => {
            //     // 追加: 空きマスにトレントを追加
            //     if !new_empty_placements.is_empty() {
            //         let pos = new_empty_placements
            //             .iter()
            //             .nth(rng.gen_range(0..new_empty_placements.len()))
            //             .unwrap()
            //             .clone();
            //         new_empty_placements.remove(&pos);
            //         new_bss[pos.0][pos.1] = 'T';
            //         new_placements.push(pos.clone());
            //     }
            // }
            0 => {
                // 削除: 既存のトレントを削除
                if !new_placements.is_empty() {
                    let idx = rng.gen_range(0..new_placements.len());
                    let pos = new_placements.remove(idx);
                    new_bss[pos.0][pos.1] = '.';
                }
            }
            1 => {
                // 移動: トレントを別の場所に移動
                if !new_placements.is_empty() {
                    let idx = rng.gen_range(0..new_placements.len());
                    let old_pos: (usize, usize) = new_placements[idx];
                    new_bss[old_pos.0][old_pos.1] = '.';
                    if !new_empty_placements.is_empty() {
                        let new_pos = new_empty_placements
                            .iter()
                            .nth(rng.gen_range(0..new_empty_placements.len()))
                            .unwrap()
                            .clone();
                        new_empty_placements.remove(&new_pos);
                        new_bss[new_pos.0][new_pos.1] = 'T';
                        new_placements[idx] = new_pos.clone();
                    } else {
                        new_bss[old_pos.0][old_pos.1] = 'T'; // 元に戻す
                    }
                }
            }
            _ => {}
        }

        // 新しい配置でシミュレーション
        let new_score = simulate(&new_bss, pi, pj, (ti, tj), q.clone());

        if new_score > best_score {
            empty_placements = new_empty_placements;
            best_placements = new_placements;
            best_bss = new_bss;
            best_score = new_score;
        }
        iteration += 1;
    }
    eprintln!("iteration: {}, {:?}", iteration, start_time.elapsed());
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
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<char> = line.trim().chars().collect();
        bss.push(row);
    }

    let mut turn = 0;
    loop {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.trim().split_whitespace();
        let pi: usize = parts.next().unwrap().parse().unwrap();
        let pj: usize = parts.next().unwrap().parse().unwrap();

        let line = lines.next().unwrap().unwrap();
        let mut parts = line.trim().split_whitespace();
        let num_confirmed: usize = parts.next().unwrap().parse().unwrap();

        let mut confirmed = HashSet::new();
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
            let best_placements = solve(start_time, &bss, &confirmed, pi, pj, ti, tj);
            print!("{}", best_placements.len());
            for (x, y) in best_placements {
                print!(" {} {}", x, y);
            }
            println!();
        } else {
            println!("0");
        }
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
