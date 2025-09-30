#![allow(unused_variables)]
use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    (a.0 as i64 - b.0 as i64).abs() as usize + (a.1 as i64 - b.1 as i64).abs() as usize
}

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
fn rotate_offset(di: i64, dj: i64, degrees: i64) -> (i64, i64) {
    match degrees % 360 {
        0 => (di, dj),
        1 => (-dj, di),  // 時計回り90度
        2 => (-di, -dj), // 180度
        3 => (dj, -di),  // 反時計回り90度
        _ => {
            panic!("Warning: Only 90-degree multiples are supported. Using 0 degrees.");
        }
    }
}

fn rotate_offsets(offsets: &Vec<(i64, i64)>, degrees: i64) -> Vec<(i64, i64)> {
    let mut rotated_offsets = Vec::new();
    for &(di, dj) in offsets {
        // 差分を回転
        let (rotated_di, rotated_dj) = rotate_offset(di, dj, degrees);
        rotated_offsets.push((rotated_di, rotated_dj));
    }
    rotated_offsets
}

fn put_jar(
    n: usize,
    already_placed: &HashSet<(usize, usize)>,
    ti: usize,
    tj: usize,
) -> HashSet<(usize, usize)> {
    let first_place: Vec<(i64, i64)> = vec![(0, -1), (0, 1), (-1, 0), (2, 0)];
    let half_secound_place: Vec<(i64, i64)> = vec![
        (-3, -1),
        (-2, -2),
        (-1, -3),
        (0, -3),
        (1, -3),
        (2, -2),
        (3, -2),
        (4, -1),
        (4, 0),
    ];
    let mut secound_place = half_secound_place.clone();
    for (di, dj) in half_secound_place {
        secound_place.push((di, -dj))
    }
    'looptop: for degrees in 0..4 {
        let rotated_first_place = rotate_offsets(&first_place, degrees);
        let rotated_secound_place = rotate_offsets(&secound_place, degrees);
        let mut jar_placements = HashSet::new();
        for (di, dj) in rotated_first_place {
            let ni: i64 = ti as i64 + di;
            let nj = tj as i64 + dj;
            for (di, dj) in ARROUND {
                let ni2 = ni as usize + di;
                let nj2 = nj as usize + dj;
                if already_placed.contains(&(ni2 as usize, nj2 as usize)) {
                    break 'looptop;
                }
            }
            if ni >= 0
                && nj >= 0
                && ni < n as i64
                && nj < n as i64
                && !already_placed.contains(&(ni as usize, nj as usize))
            {
                jar_placements.insert((ni as usize, nj as usize));
            }
        }
        for (di, dj) in rotated_secound_place {
            let ni = ti as i64 + di;
            let nj = tj as i64 + dj;
            if ni >= 0
                && nj >= 0
                && ni < n as i64
                && nj < n as i64
                && !already_placed.contains(&(ni as usize, nj as usize))
            {
                jar_placements.insert((ni as usize, nj as usize));
            }
        }
        let mut tmp_already_placed = already_placed.clone();
        tmp_already_placed.extend(jar_placements.clone());
        if can_reach_goals(n, &tmp_already_placed, 0, n / 2, &vec![(ti, tj)]) {
            return jar_placements;
        }
    }

    HashSet::new()
}

fn can_reach_goal_with_route(
    n: usize,
    already_placed: &HashSet<(usize, usize)>,
    p: (usize, usize),
    goal: (usize, usize),
) -> Vec<(usize, usize)> {
    let (pi, pj) = p;
    let (ti, tj) = goal;
    let mut queue = VecDeque::new();
    queue.push_back((pi, pj, Vec::new()));
    let mut visited = HashSet::new();
    visited.insert((pi, pj));
    while !queue.is_empty() {
        let (ni, nj, mut route) = queue.pop_front().unwrap();
        visited.insert((ni, nj));
        route.push((ni, nj));
        if ni == ti && nj == tj {
            return route;
        }
        for (di, dj) in DIJ {
            let ni2 = ni + di;
            let nj2 = nj + dj;
            if ni2 < n && nj2 < n && !visited.contains(&(ni2, nj2)) {
                queue.push_back((ni2, nj2, route.clone()));
            }
        }
    }
    Vec::new()
}

fn divide_forest(
    n: usize,
    already_placed: &HashSet<(usize, usize)>,
    p: (usize, usize),
    t: (usize, usize),
) -> HashSet<(usize, usize)> {
    let (pi, pj) = p;
    let (ti, tj) = t;
    let mut route = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((n - 1, pj));
    let mut visited = HashSet::new();
    visited.insert((n - 1, pj));
    while !queue.is_empty() {
        let (ni, nj) = queue.pop_front().unwrap();
        visited.insert((ni, nj));
        if !already_placed.contains(&(ni, nj)) {
            route = can_reach_goal_with_route(n, already_placed, p, (ni, nj));
            if !route.is_empty() {
                break;
            }
        }
        for (di, dj) in DIJ {
            let ni2 = ni + di;
            let nj2 = nj + dj;
            if ni2 < n && nj2 < n && !visited.contains(&(ni2, nj2)) {
                queue.push_back((ni2, nj2));
            }
        }
    }
    let mut trent_placements = HashSet::new();
    for (ni, nj) in route {
        if nj < n && nj < n {
            if !already_placed.contains(&(ni, nj - 1)) {
                trent_placements.insert((ni, nj - 1));
            }
            if !already_placed.contains(&(ni, nj + 1)) {
                trent_placements.insert((ni, nj + 1));
            }
        }
    }
    let mut uf = new_uf(n, &trent_placements);
    for i in 0..n {
        for j in 0..n {
            if trent_placements.contains(&(i, j)) {}
        }
    }
    trent_placements
}

// 上下左右に木があるかどうか
fn exist_dij_tree(
    n: usize,
    already_placed: &HashSet<(usize, usize)>,
    ti: usize,
    tj: usize,
) -> bool {
    for (di, dj) in DIJ {
        let ni = ti + di;
        let nj = tj + dj;
        if ni < n && nj < n {
            if already_placed.contains(&(ni, nj)) {
                return true;
            }
        }
    }
    false
}

fn select_next_place(
    turn: usize,
    n: usize,
    confirmed: &HashSet<(usize, usize)>,
    p: (usize, usize),
    t: (usize, usize),
    will_place: &mut HashSet<(usize, usize)>,
    already_placed: &HashSet<(usize, usize)>,
    before_place: Option<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    if will_place.is_empty() {
        return HashSet::new();
    }

    let (pi, pj) = p;
    let mut next_place = HashSet::new();
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
                    next_place.insert(pos);
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

/*
  x
  xx
  x
  xfx
   x
*/
fn make_side_vertical_candidate(
    n: usize,
    t: (usize, usize),
    direction: usize,
    already_placed: &HashSet<(usize, usize)>,
    confirmed: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut candidate = HashSet::from([(t.0 + 1, t.1), (t.0, t.1 - direction)]);
    candidate.extend((0..=t.0).map(|i| (i, t.1 + direction)));
    let mut ni = t.0 + !0;
    while ni < n {
        if already_placed.contains(&(ni, t.1)) || confirmed.contains(&(ni, t.1)) {
            break;
        }
        let mut tmp_already_placed = already_placed.clone();
        tmp_already_placed.extend(candidate.clone());
        tmp_already_placed.insert((ni, t.1));
        let can_reach = can_reach_goals(n, &tmp_already_placed, 0, n / 2, &vec![t]);
        if can_reach {
            candidate.insert((ni, t.1));
            break;
        }
        ni += !0
    }
    candidate.retain(|&(i, j)| {
        i < n && j < n && !confirmed.contains(&(i, j)) && !already_placed.contains(&(i, j))
    });
    candidate
}

/*
   x
   x
  x
  xfx
   x
*/
fn make_just_above_vertical_candidate(
    n: usize,
    t: (usize, usize),
    direction: usize,
    already_placed: &HashSet<(usize, usize)>,
    confirmed: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut candidate = HashSet::from([
        (t.0 - 1, t.1 + direction),
        (t.0, t.1 + direction),
        (t.0 + 1, t.1),
        (t.0, t.1 - direction),
    ]);
    if t.0 >= 2 {
        candidate.extend((0..=t.0 - 2).map(|i| (i, t.1)));
    }
    candidate.retain(|&(i, j)| {
        i < n && j < n && !confirmed.contains(&(i, j)) && !already_placed.contains(&(i, j))
    });
    candidate
}
/*
   x
   x
   x
  xf x
   xx
*/
fn make_just_above_vertical_candidate2(
    n: usize,
    t: (usize, usize),
    direction: usize,
    already_placed: &HashSet<(usize, usize)>,
    confirmed: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut candidate = HashSet::from([(t.0, t.1 + direction), (t.0 + 1, t.1)]);
    if t.0 >= 1 {
        candidate.extend((0..=t.0 - 1).map(|i| (i, t.1)));
    }

    let mut nj1: usize = t.1 - direction - direction;
    let ni1 = t.0;
    let mut nj2: usize = t.1 - direction;
    let ni2 = t.0 + 1;
    let mut placed = HashSet::new();
    let mut cannnot_surround = true;
    while nj1 < n {
        let mut tmp_already_placed = already_placed.clone();
        tmp_already_placed.extend(candidate.clone());
        tmp_already_placed.extend(placed.clone());
        tmp_already_placed.insert((ni1, nj1));
        tmp_already_placed.insert((ni2, nj2));
        let can_reach = can_reach_goals(n, &tmp_already_placed, 0, n / 2, &vec![t]);
        if can_reach {
            candidate.extend(placed.clone());
            candidate.insert((ni1, nj1));
            candidate.insert((ni2, nj2));
            cannnot_surround = false;
            break;
        }
        placed.insert((ni2, nj2));
        nj1 -= direction;
        nj2 -= direction;
    }
    if cannnot_surround {
        return HashSet::new();
    }
    candidate.retain(|&(i, j)| {
        i < n && j < n && !confirmed.contains(&(i, j)) && !already_placed.contains(&(i, j))
    });
    candidate
}

fn put_vertical_treant(
    n: usize,
    already_placed: &HashSet<(usize, usize)>,
    confirmed: &HashSet<(usize, usize)>,
    t: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut tree = HashSet::new();
    if t.1 == n / 2 {
        let mut best_tree = HashSet::new();
        for direction in vec![!0, 1] {
            for i in 0..=t.0 - 2 {
                if !already_placed.contains(&(i, t.1 + direction))
                    && !confirmed.contains(&(i, t.1 + direction))
                {
                    tree.insert((i, t.1 + direction));
                }
            }
            for (i, j) in vec![
                (t.0 - 1, t.1),
                (t.0, t.1 - direction),
                (t.0 + 1, t.1),
                (t.0 + 1, t.1 + direction),
                (t.0, (t.1 + direction) + direction),
            ] {
                if i < n
                    && j < n
                    && !already_placed.contains(&(i, j))
                    && !confirmed.contains(&(i, j))
                {
                    tree.insert((i, j));
                }
            }
            let mut tmp_already_placed = already_placed.clone();
            tmp_already_placed.extend(tree.clone());
            let can_reach = can_reach_goals(n, &tmp_already_placed, 0, n / 2, &vec![t]);
            if can_reach {
                best_tree = tree.clone();
            }
        }
        return best_tree;
    } else if t.1 == n / 2 - 1 || t.1 == n / 2 + 1 {
        let direction = if t.1 == n / 2 - 1 { 1 } else { !0 };

        let candidate = if !already_placed.contains(&(t.0 - 1, t.1)) {
            make_just_above_vertical_candidate(n, t, direction, already_placed, confirmed)
        } else {
            make_just_above_vertical_candidate2(n, t, direction, already_placed, confirmed)
        };
        let mut tmp_already_placed = already_placed.clone();
        tmp_already_placed.extend(candidate.clone());
        let can_reach = can_reach_goals(n, &tmp_already_placed, 0, n / 2, &vec![t]);
        if can_reach {
            return candidate;
        } else {
            return HashSet::new();
        }
    }
    let direction = if t.1 < n / 2 { 1 } else { !0 };
    let candidates: Vec<HashSet<(usize, usize)>> = if !already_placed.contains(&(t.0 - 1, t.1)) {
        vec![
            make_just_above_vertical_candidate(n, t, direction, already_placed, confirmed),
            make_side_vertical_candidate(n, t, direction, already_placed, confirmed),
            make_just_above_vertical_candidate2(n, t, direction, already_placed, confirmed),
        ]
    } else {
        vec![make_just_above_vertical_candidate2(
            n,
            t,
            direction,
            already_placed,
            confirmed,
        )]
    };
    for candidate in candidates {
        let mut tmp_already_placed = already_placed.clone();
        tmp_already_placed.extend(candidate.clone());
        let can_reach = can_reach_goals(n, &tmp_already_placed, 0, n / 2, &vec![t]);
        if can_reach {
            return candidate;
        }
    }
    return HashSet::new();
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
    for (di, dj) in DIJ {
        let ni = ti + di;
        let nj = tj + dj;

        if ni < n && nj < n && !confirmed.contains(&(ni, nj)) && !already_placed.contains(&(ni, nj))
        {
            empty_arround_flower.insert((ni, nj));
        }
        for (ddi, ddj) in DIJ {
            let ni = ti + di + ddi;
            let nj = tj + dj + ddj;

            if ni < n
                && nj < n
                && !confirmed.contains(&(ni, nj))
                && !already_placed.contains(&(ni, nj))
            {
                empty_arround_flower.insert((ni, nj));
            }
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
    n: usize,
    already_placed: &HashSet<(usize, usize)>,
    confirmed: &HashSet<(usize, usize)>,
    ti: usize,
    tj: usize,
    direction: usize,
    t: (usize, usize),
    empty_arround_flower: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut treant_placements = HashSet::new();
    let mut tmp_ti = ti;
    let mut tmp_empty_arround_flower = empty_arround_flower.clone();
    for (di, dj) in DIJ {
        let ni = t.0 + di;
        let nj = t.1 + dj;
        if ni < n && nj < n {
            tmp_empty_arround_flower.remove(&(ni, nj));
        }
    }

    let mut tmp_tj = if direction == 1 { tj } else { n - 1 - tj };
    while tmp_ti < n && tmp_tj < n {
        if !already_placed.contains(&(tmp_ti, tmp_tj))
            && !confirmed.contains(&(tmp_ti, tmp_tj))
            // 木がなければ置いていい
            // 木があっても花の周りの中で花の隣り合わせでなければ置いていい
            && (!exist_dij_tree(n, already_placed, tmp_ti, tmp_tj)
                || tmp_empty_arround_flower.contains(&(tmp_ti, tmp_tj)))
            && (tmp_ti, tmp_tj) != t
        {
            treant_placements.insert((tmp_ti, tmp_tj));
        }
        tmp_ti += 1;
        tmp_tj += direction;
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

fn can_reach_goal(
    n: usize,
    already_placed: &HashSet<(usize, usize)>,
    start_ti: usize,
    start_tj: usize,
    goal: (usize, usize),
) -> (bool, usize) {
    if start_ti == goal.0 && start_tj == goal.1 {
        return (true, 0);
    }

    let mut queue = VecDeque::new();
    queue.push_back((start_ti, start_tj, 0));
    let mut visited = HashSet::new();
    visited.insert((start_ti, start_tj));

    while !queue.is_empty() {
        let (i, j, steps) = queue.pop_front().unwrap();

        for (di, dj) in DIJ {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);

            if ni < n && nj < n && !already_placed.contains(&(ni, nj)) && visited.insert((ni, nj)) {
                if ni == goal.0 && nj == goal.1 {
                    return (true, steps + 1);
                }
                queue.push_back((ni, nj, steps + 1));
            }
        }
    }
    (false, 0)
}

fn can_reach_goals(
    n: usize,
    already_placed: &HashSet<(usize, usize)>,
    start_ti: usize,
    start_tj: usize,
    goals: &[(usize, usize)],
) -> bool {
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

            if ni < n && nj < n && !already_placed.contains(&(ni, nj)) && visited.insert((ni, nj)) {
                remaining_goals.remove(&(ni, nj));
                queue.push_back((ni, nj, steps + 1));
            }
        }
    }
    remaining_goals.is_empty()
}
fn surround_flower(
    n: usize,
    already_placed: &HashSet<(usize, usize)>,
    confirmed: &HashSet<(usize, usize)>,
    t: (usize, usize),
    must_reach: &Vec<(usize, usize)>,
    surround_candidates: &Vec<Vec<(i64, i64)>>,
) -> Vec<HashSet<(usize, usize)>> {
    let mut can_reach_treant_placements = Vec::new();
    for candidate in surround_candidates {
        let mut treant_placements = HashSet::new();
        for (di, dj) in candidate {
            let ni = t.0 as i64 + di;
            let nj = t.1 as i64 + dj;
            if ni < 0 || nj < 0 {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if ni < n && nj < n {
                if !already_placed.contains(&(ni, nj)) && !confirmed.contains(&(ni, nj)) {
                    treant_placements.insert((ni, nj));
                }
            }
        }
        let mut trents = already_placed.clone();
        trents.extend(treant_placements.clone());
        if can_reach_goals(n, &trents, 0, n / 2, must_reach) {
            can_reach_treant_placements.push(treant_placements);
        }
    }

    if can_reach_treant_placements.is_empty() {
        let mut treant_placements = HashSet::new();
        for (di, dj) in DIJ {
            let mut ni = t.0 + di;
            let mut nj = t.1 + dj;
            while ni < n && nj < n {
                if already_placed.contains(&(ni, nj)) || confirmed.contains(&(ni, nj)) {
                    break;
                }
                let mut tmp_already_placed = already_placed.clone();
                tmp_already_placed.extend(treant_placements.clone());
                tmp_already_placed.insert((ni, nj));
                let can_reach = can_reach_goals(n, &tmp_already_placed, 0, n / 2, must_reach);
                if can_reach {
                    treant_placements.insert((ni, nj));
                    break;
                }
                ni += di;
                nj += dj;
            }
        }
        if !treant_placements.is_empty() {
            can_reach_treant_placements.push(treant_placements);
        }
    }

    can_reach_treant_placements
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
fn put_all_diagonal(
    n: usize,
    already_placed: &HashSet<(usize, usize)>,
    confirmed: &HashSet<(usize, usize)>,
    start_pos: usize,
    direction: usize,
    t: (usize, usize),
    empty_arround_flower: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let (ti, tj) = t;
    let mut treant_placements = HashSet::new();
    // 3つ飛ばしで斜めに線を引く
    for i in (0..n).step_by(3) {
        treant_placements.extend(put_diagonal(
            n,
            already_placed,
            &confirmed,
            0,
            i + start_pos,
            direction,
            t,
            empty_arround_flower,
        ));
    }
    for i in (0..n).step_by(3) {
        treant_placements.extend(put_diagonal(
            n,
            already_placed,
            &confirmed,
            i + 3 - start_pos,
            0,
            direction,
            t,
            empty_arround_flower,
        ));
    }

    treant_placements
}

fn new_uf(n: usize, treant_placements: &HashSet<(usize, usize)>) -> UnionFind {
    let mut uf = UnionFind::new(n * n);
    for i in 0..n {
        for j in 0..n {
            if !treant_placements.contains(&(i, j)) {
                if !treant_placements.contains(&((i + 1), j)) && i < n - 1 {
                    uf.union(i * n + j, (i + 1) * n + j);
                }
                if !treant_placements.contains(&(i, j + 1)) && j < n - 1 {
                    uf.union(i * n + j, i * n + j + 1);
                }
            }
        }
    }
    let i = n - 1;
    let j = n - 1;
    if !treant_placements.contains(&(i, j)) {
        if !treant_placements.contains(&((i - 1), j)) {
            uf.union(i * n + j, (i - 1) * n + j);
        }
        if !treant_placements.contains(&(i, j - 1)) {
            uf.union(i * n + j, i * n + j - 1);
        }
    }
    uf
}

fn init_qs(n: usize) -> Vec<Vec<(usize, usize)>> {
    use rand::prelude::*;
    let mut rng = thread_rng();
    let mut q = Vec::new();
    let entrance = (0, n / 2);
    for i in 0..n {
        for j in 0..n {
            if (i, j) != entrance {
                q.push((i, j));
            }
        }
    }
    let mut qs = Vec::new();
    for _ in 0..3 {
        q.shuffle(&mut rng);
        qs.push(q.clone());
    }
    qs
}

fn solve(
    start_time: std::time::Instant,
    n: usize,
    already_placed: &HashSet<(usize, usize)>,
    confirmed: &HashSet<(usize, usize)>,
    pi: usize,
    pj: usize,
    ti: usize,
    tj: usize,
    empty_arround_flower: &HashSet<(usize, usize)>,
) -> (HashSet<(usize, usize)>, bool) {
    // Generate q: all cells except entrance in random order

    let mut tmp_already_placements = already_placed.clone();
    let jar_placements = put_jar(n, &tmp_already_placements, ti, tj);
    tmp_already_placements.extend(jar_placements.clone());

    let vertical_treant_placements = if jar_placements.is_empty() {
        put_vertical_treant(n, &already_placed, &confirmed, (ti, tj))
    } else {
        HashSet::new()
    };
    tmp_already_placements.extend(vertical_treant_placements.clone());
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
    let can_reach_surround_placements =
        if jar_placements.is_empty() && vertical_treant_placements.is_empty() {
            surround_flower(
                n,
                &tmp_already_placements,
                &confirmed,
                (ti, tj),
                &vec![(ti, tj)],
                &surround_candidates,
            )
        } else {
            vec![]
        };
    let surround_placements = if !can_reach_surround_placements.is_empty() {
        can_reach_surround_placements[0].clone()
    } else {
        HashSet::new()
    };
    tmp_already_placements.extend(surround_placements.clone());
    eprintln!(
        "vertical_treant_placements: {:?}",
        vertical_treant_placements
    );
    eprintln!("jar_placements: {:?}", jar_placements);
    eprintln!("surround_placements: {:?}", surround_placements);
    let mut best_placements = HashSet::new();
    let mut best_score = 0;
    let mut tmp_confirmed = confirmed.clone();
    if vertical_treant_placements.len() > 0 {
        for i in 1..=4 {
            tmp_confirmed.insert((ti + i, tj));
            tmp_confirmed.insert((ti + i, tj + 1));
            tmp_confirmed.insert((ti + i, tj - 1));
        }
    }
    let qs: Vec<Vec<(usize, usize)>> = init_qs(n);
    for i in 0..3 {
        let direction: usize = if tj >= n / 2 { 1 } else { !0 };
        let mut treant_placements = put_all_diagonal(
            n,
            &tmp_already_placements,
            &tmp_confirmed,
            i,
            direction,
            (ti, tj),
            empty_arround_flower,
        );
        let mut all_placements = tmp_already_placements.clone();
        all_placements.extend(treant_placements.clone());
        let mut uf: UnionFind = new_uf(n, &all_placements);
        for (i, j) in treant_placements.clone() {
            if tmp_already_placements.contains(&(i, j)) {
                continue;
            }
            if (i == 0 || i == n - 1) && (j == 0 || j == n - 1) {
                // 角の位置にいる場合
                let di = if i == 0 { 1 } else { -1 };
                let dj = if j == 0 { 1 } else { -1 };

                let neighbor1_idx = ((i as isize + di) as usize) * n + j;
                let neighbor2_idx = i * n + ((j as isize + dj) as usize);

                if !uf.is_same(neighbor1_idx, neighbor2_idx) {
                    uf.union(neighbor1_idx, neighbor2_idx);
                    treant_placements.remove(&(i, j));
                }
            }
            if (i > 0 && i < n - 1 && !uf.is_same((i + 1) * n + j, (i - 1) * n + j))
                || (j > 0 && j < n - 1 && !uf.is_same(i * n + j + 1, i * n + j - 1))
            {
                treant_placements.remove(&(i, j));
                let mut tmp = tmp_already_placements.clone();
                tmp.extend(treant_placements.clone());
                if i > 0 && i < n - 1 {
                    if !tmp.contains(&((i + 1), j)) {
                        uf.union((i + 1) * n + j, i * n + j);
                    }
                    if !tmp.contains(&((i - 1), j)) {
                        uf.union((i - 1) * n + j, i * n + j);
                    }
                }
                if j > 0 && j < n - 1 {
                    if !tmp.contains(&((i, j + 1))) {
                        uf.union(i * n + j + 1, i * n + j);
                    }
                    if !tmp.contains(&((i, j - 1))) {
                        uf.union(i * n + j, i * n + j - 1);
                    }
                }
            }
        }
        let score = simulate(n, &tmp_already_placements, pi, pj, (ti, tj), qs.clone());
        if score > best_score {
            best_score = score;
            best_placements = treant_placements;
        }
    }
    best_placements.extend(tmp_already_placements.clone());
    (best_placements, surround_placements.len() > 0)
}

fn size_large_or_start_position(
    n: usize,
    uf: &mut UnionFind,
    xs: (usize, usize),
    ys: (usize, usize),
    ps: (usize, usize),
    ts: (usize, usize),
) -> bool {
    let x = xs.0 * n + xs.1;
    let y = ys.0 * n + ys.1;
    let p = ps.0 * n + ps.1;
    let t = ts.0 * n + ts.1;

    // サイズが1なら統合したくないが、それがスタートポジションなら絶対繋がないといけないので、スタートポジションは特別扱いする
    (uf.get_size(x) > 1 && uf.get_size(y) > 1)
        || uf.is_same(x, p)
        || uf.is_same(y, p)
        || uf.is_same(x, t)
        || uf.is_same(y, t)
}

fn is_same_position(x: (usize, usize), y: (usize, usize)) -> bool {
    x.0 == y.0 && x.1 == y.1
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
    let mut before_place = None;
    let mut confirmed: HashSet<(usize, usize)> = HashSet::new();
    let mut empty_arround_flower =
        make_empty_arround_flower(n, &confirmed, &already_placed, &will_place, (ti, tj));
    let mut surround_candidates: Vec<Vec<(i64, i64)>> = if tj >= n / 2 {
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
            let solve_result = solve(
                start_time,
                n,
                &already_placed,
                &confirmed,
                pi,
                pj,
                ti,
                tj,
                &empty_arround_flower,
            );
            if !solve_result.1 {
                surround_candidates = vec![];
            }
            will_place = solve_result.0;
        }

        let elapsed_ms = start_time.elapsed().as_millis();
        let next_place = if elapsed_ms < 1900 {
            select_next_place2(
                turn,
                n,
                &confirmed,
                (pi, pj),
                (ti, tj),
                &mut will_place,
                &already_placed,
                before_place,
                &empty_arround_flower,
                &mut surround_candidates,
            )
        } else {
            HashSet::new()
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
    eprintln!("elapsed_ms: {}", start_time.elapsed().as_millis());
}

pub struct Sim {
    pub n: usize,
    pub already_placed: HashSet<(usize, usize)>,
    pub p: (usize, usize),
    pub t: (usize, usize),
    pub target: (usize, usize),
    pub revealed: Vec<bool>,
    pub new_revealed: Vec<(usize, usize)>,
    pub dist: Vec<i32>,
    pub q: Vec<(usize, usize)>,
}

impl Sim {
    pub fn new(
        n: usize,
        already_placed: HashSet<(usize, usize)>,
        t: (usize, usize),
        q: Vec<(usize, usize)>,
    ) -> Self {
        let mut revealed = vec![false; n * n];
        revealed[n / 2] = true;
        Self {
            n: n,
            already_placed: already_placed,
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
                    && (!self.revealed[i2 * self.n + j2]
                        || !self.already_placed.contains(&(i2, j2)))
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
            } else if self.already_placed.contains(&(i, j)) {
                return Err(format!("({}, {}) is not empty", i, j));
            } else if (i, j) == self.t {
                return Err(format!("({}, {}) contains the flower", i, j));
            }
            self.already_placed.insert((i, j));
        }
        let mut changed = false;
        for (di, dj) in DIJ {
            let mut i2: usize = self.p.0;
            let mut j2 = self.p.1;
            while i2 < self.n && j2 < self.n {
                if self.revealed[i2 * self.n + j2].setmax(true) {
                    self.new_revealed.push((i2, j2));
                    if self.already_placed.contains(&(i2, j2)) {
                        changed = true;
                    }
                }
                if self.already_placed.contains(&(i2, j2)) {
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
    n: usize,
    already_placed: &HashSet<(usize, usize)>,
    start_ti: usize,
    start_tj: usize,
    flower_pos: (usize, usize),
    qs: Vec<Vec<(usize, usize)>>,
) -> usize {
    let mut min_steps = usize::MAX;
    for q in qs {
        let mut sim = Sim::new(n, already_placed.clone(), flower_pos, q);
        sim.p = (start_ti, start_tj);
        sim.revealed.fill(false);
        sim.revealed[start_ti * n + start_tj] = true;

        let mut steps = 0;
        loop {
            if sim.p == flower_pos {
                break;
            }
            // Simulate one step with no treant placements
            if let Err(_) = sim.step(&[]) {
                steps = 0;
                break; // Cannot continue
            }
            steps += 1;
        }
        min_steps.setmin(steps);
    }
    min_steps
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

    /// デバッグ用: 機械可読な形式で親情報を出力
    pub fn debug_print_parents_json(
        &mut self,
        n: usize,
        treant_placements: &HashSet<(usize, usize)>,
    ) {
        let parents = self.debug_get_parents(n);
        eprintln!("UNION_FIND_PARENTS_START");
        for i in 0..n {
            for j in 0..n {
                let (pi, pj) = parents[i][j];
                let is_treant = treant_placements.contains(&(i, j));
                eprintln!(
                    "{} {} {} {} {}",
                    i,
                    j,
                    pi,
                    pj,
                    if is_treant { 1 } else { 0 }
                );
            }
        }
        eprintln!("UNION_FIND_PARENTS_END");
    }

    /// デバッグ用: 各要素の親を2D配列で返す
    fn debug_get_parents(&mut self, n: usize) -> Vec<Vec<(usize, usize)>> {
        let mut result = vec![vec![(0, 0); n]; n];
        for i in 0..n {
            for j in 0..n {
                let idx = i * n + j;
                let parent = self.find(idx);
                let parent_i = parent / n;
                let parent_j = parent % n;
                result[i][j] = (parent_i, parent_j);
            }
        }
        result
    }
}

fn select_next_place2(
    turn: usize,
    n: usize,
    confirmed: &HashSet<(usize, usize)>,
    p: (usize, usize),
    t: (usize, usize),
    will_place: &mut HashSet<(usize, usize)>,
    already_placed: &HashSet<(usize, usize)>,
    before_place: Option<(usize, usize)>,
    empty_arround_flower: &HashSet<(usize, usize)>,
    surround_candidates: &mut Vec<Vec<(i64, i64)>>,
) -> HashSet<(usize, usize)> {
    if will_place.is_empty() {
        return HashSet::new();
    }

    let (pi, pj) = p;
    let mut next_place = HashSet::new();
    // 前回の移動方向から、今回スキャンする方向を決定
    let (scan_vertical, scan_horizontal) = match before_place {
        Some((prev_i, _)) if prev_i == pi => (true, false), // 横移動してきた→縦スキャン
        Some(_) => (false, true),                           // 縦移動してきた→横スキャン
        None => (true, true),                               // 初回→全方向スキャン
    };

    // 共通のスキャン処理
    let mut need_simulation = HashSet::new();
    let mut scan_line = |positions: Vec<(usize, usize)>| {
        let mut push_need_simulation = false;
        for pos in positions {
            if already_placed.contains(&pos) {
                break;
            }
            if !confirmed.contains(&pos) {
                if will_place.contains(&pos) && !push_need_simulation {
                    will_place.remove(&pos);
                    next_place.insert(pos);
                    break;
                }
                if empty_arround_flower.contains(&pos) && surround_candidates.len() > 0 {
                    need_simulation.insert(pos);
                    push_need_simulation = true;
                    continue;
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
    // 見られる方向をなるべく隠せるように徐々に囲っていく
    let mut surround_place = HashSet::new();
    if !need_simulation.is_empty() {
        let mut candidates = surround_candidates.clone();

        candidates.retain(|candidate| {
            candidate.iter().any(|&(di, dj)| {
                need_simulation.contains(&((di as usize + t.0, dj as usize + t.1)))
            })
        });

        if !candidates.is_empty() {
            let surround_placements =
                surround_flower(n, &already_placed, &confirmed, t, &vec![t], &candidates);
            let mut best_surround_placement = HashSet::new();
            for surround_placement in surround_placements {
                let place: HashSet<(usize, usize)> = surround_placement
                    .clone()
                    .intersection(&need_simulation)
                    .cloned()
                    .collect();
                if place.len() > best_surround_placement.len()
                    || (place.len() > 0
                        && place.len() == best_surround_placement.len()
                        && distance(p, *place.iter().next().unwrap())
                            < distance(p, *best_surround_placement.iter().next().unwrap()))
                {
                    best_surround_placement = place;
                }
            }
            if !best_surround_placement.is_empty() {
                let place = best_surround_placement
                    .iter()
                    .min_by_key(|(i, j)| distance(p, (*i, *j)))
                    .unwrap();
                surround_place.insert(*place);
            }
        }
    }
    if !surround_place.is_empty() {
        next_place.extend(surround_place.clone());
        let mut all_place = will_place.clone();
        all_place.extend(next_place.clone());
        all_place.extend(already_placed.clone());
        let mut uf = new_uf(n, &all_place);

        for (i, j) in all_place.clone() {
            if surround_place.contains(&(i, j)) || already_placed.contains(&(i, j)) {
                continue;
            }
            if (i > 0
                && i < n - 1
                && !uf.is_same((i + 1) * n + j, (i - 1) * n + j)
                && size_large_or_start_position(n, &mut uf, (i + 1, j), (i - 1, j), (pi, pj), t))
                || (j > 0
                    && j < n - 1
                    && !uf.is_same(i * n + j + 1, i * n + j - 1)
                    && size_large_or_start_position(
                        n,
                        &mut uf,
                        (i, j + 1),
                        (i, j - 1),
                        (pi, pj),
                        t,
                    ))
            {
                if i > 0 && i < n - 1 {
                    uf.union((i + 1) * n + j, (i - 1) * n + j);
                }
                if j > 0 && j < n - 1 {
                    uf.union(i * n + j + 1, i * n + j - 1);
                }
                if next_place.contains(&(i, j)) {
                    next_place.remove(&(i, j));
                }
                if will_place.contains(&(i, j)) {
                    will_place.remove(&(i, j));
                }
            }
        }
    }
    next_place
}

fn eprint_placements(placements: &HashSet<(usize, usize)>) {
    eprint!("{}", placements.len());
    for (i, j) in placements {
        eprint!(" {} {}", i, j);
    }
    eprintln!();
}
