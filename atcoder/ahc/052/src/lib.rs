#![allow(non_snake_case, unused_macros)]

use proconio::{input, marker::Chars};
use rand::prelude::*;
use std::ops::RangeBounds;
use svg::node::element::{Circle, Group, Line, Rectangle, Style, Text, Title};

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

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[derive(Clone, Debug)]
pub struct Input {
    pub N: usize,
    pub M: usize,
    pub K: usize,
    pub ps: Vec<(usize, usize)>,
    pub wall_v: Vec<Vec<bool>>,
    pub wall_h: Vec<Vec<bool>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {}", self.N, self.M, self.K)?;
        for (x, y) in &self.ps {
            writeln!(f, "{} {}", x, y)?;
        }
        for v in &self.wall_v {
            writeln!(
                f,
                "{}",
                v.iter()
                    .map(|&x| if x { '1' } else { '0' })
                    .collect::<String>()
            )?;
        }
        for h in &self.wall_h {
            writeln!(
                f,
                "{}",
                h.iter()
                    .map(|&x| if x { '1' } else { '0' })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        N: usize, M: usize, K: usize,
        ps: [(usize, usize); M],
        wall_v: [Chars; N],
        wall_h: [Chars; N - 1],
    }
    let wall_v = wall_v
        .into_iter()
        .map(|s| s.into_iter().map(|c| c == '1').collect())
        .collect();
    let wall_h = wall_h
        .into_iter()
        .map(|s| s.into_iter().map(|c| c == '1').collect())
        .collect();
    Input {
        N,
        M,
        K,
        ps,
        wall_v,
        wall_h,
    }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr, R: RangeBounds<T>>(
    token: Option<&str>,
    range: R,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if !range.contains(&v) {
                Err(format!("Out of range: {}", v))
            } else {
                Ok(v)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

pub const CMD: [char; 5] = ['U', 'D', 'L', 'R', 'S'];
pub const DIJ: [(usize, usize); 5] = [(!0, 0), (1, 0), (0, !0), (0, 1), (0, 0)];

pub struct Output {
    pub cs: Vec<Vec<usize>>,
    pub actions: Vec<usize>,
}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut f = f.split_whitespace();
    let mut cs = mat![0; input.K; input.M];
    for i in 0..input.K {
        for j in 0..input.M {
            let c = read(f.next(), 'A'..='Z')?;
            let c = CMD
                .iter()
                .position(|&x| x == c)
                .ok_or_else(|| format!("Invalid command: {}", c))?;
            cs[i][j] = c;
        }
    }
    let mut actions = vec![];
    for a in f {
        let a = read(Some(a), 0..input.K)?;
        actions.push(a);
        if actions.len() > 2 * input.N * input.N {
            return Err(format!("Too many actions"));
        }
    }
    Ok(Output { cs, actions })
}

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let N = 30;
    let M = 10;
    let K = 10;
    let mut ps = vec![];
    for i in 0..N {
        for j in 0..N {
            ps.push((i, j));
        }
    }
    ps.shuffle(&mut rng);
    ps.truncate(M);
    let mut wall_v;
    let mut wall_h;
    loop {
        wall_v = mat![false; N; N - 1];
        wall_h = mat![false; N - 1; N];
        let mut used_v = vec![false; N - 1];
        let mut used_h = vec![false; N - 1];
        for _ in 0..5 {
            loop {
                let dir = rng.gen_range(0i32..4) as usize;
                let l = rng.gen_range(10i32..=20) as usize;
                if dir / 2 == 0 {
                    // vertical
                    let i = rng.gen_range(5..=N as i32 - 5) as usize;
                    let j = rng.gen_range(4..=N as i32 - 6) as usize;
                    if used_v[j] {
                        continue;
                    }
                    for k in 0..9 {
                        if j - 4 + k < N - 1 {
                            used_v[j - 4 + k] = true;
                        }
                    }
                    for k in 0..l {
                        let i = i + DIJ[dir].0 * k;
                        if i < N {
                            wall_v[i][j] = true;
                        }
                    }
                } else {
                    // horizontal
                    let i = rng.gen_range(4..=N as i32 - 6) as usize;
                    let j = rng.gen_range(5..=N as i32 - 5) as usize;
                    if used_h[i] {
                        continue;
                    }
                    for k in 0..9 {
                        if i - 4 + k < N - 1 {
                            used_h[i - 4 + k] = true;
                        }
                    }
                    for k in 0..l {
                        let j = j + DIJ[dir].1 * k;
                        if j < N {
                            wall_h[i][j] = true;
                        }
                    }
                }
                break;
            }
        }
        let mut visited = mat![false; N; N];
        let mut stack = vec![(0, 0)];
        visited[0][0] = true;
        let mut num = 0;
        while let Some((i, j)) = stack.pop() {
            num += 1;
            for d in 0..4 {
                let (di, dj) = DIJ[d];
                let (i2, j2) = (i + di, j + dj);
                if i2 >= N || j2 >= N || visited[i2][j2] {
                    continue;
                }
                if di == 0 {
                    if wall_v[i][j.min(j2)] {
                        continue;
                    }
                } else {
                    if wall_h[i.min(i2)][j] {
                        continue;
                    }
                }
                visited[i2][j2] = true;
                stack.push((i2, j2));
            }
        }
        if num == N * N {
            break;
        }
    }
    Input {
        N,
        M,
        K,
        ps,
        wall_v,
        wall_h,
    }
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, out, out.actions.len());
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

#[derive(Clone, Debug)]
pub struct State {
    ps: Vec<(usize, usize)>,
    used: Vec<Vec<usize>>,
}

fn can_move(input: &Input, (i, j): (usize, usize), dir: usize) -> bool {
    if dir == 4 {
        return true;
    }
    let (di, dj) = DIJ[dir];
    let i2 = i + di;
    let j2 = j + dj;
    if i2 >= input.N || j2 >= input.N {
        return false;
    }
    if di == 0 {
        !input.wall_v[i][j.min(j2)]
    } else {
        !input.wall_h[i.min(i2)][j]
    }
}

pub fn compute_score_details(input: &Input, out: &Output, t: usize) -> (i64, String, State) {
    let mut state = State {
        ps: input.ps.clone(),
        used: mat![!0; input.N; input.N],
    };
    for i in 0..input.M {
        state.used[state.ps[i].0][state.ps[i].1] = i;
    }
    for &a in &out.actions[..t] {
        for i in 0..input.M {
            let dir = out.cs[a][i];
            if can_move(input, state.ps[i], dir) {
                state.ps[i].0 += DIJ[dir].0;
                state.ps[i].1 += DIJ[dir].1;
                state.used[state.ps[i].0][state.ps[i].1] = i;
            }
        }
    }
    let mut rem = 0;
    for i in 0..input.N {
        for j in 0..input.N {
            if state.used[i][j] == !0 {
                rem += 1;
            }
        }
    }
    let score = if rem == 0 {
        (3 * input.N * input.N - t) as i64
    } else {
        (input.N * input.N - rem) as i64
    };
    (score, String::new(), state)
}

/// 0 <= val <= 1
pub fn color(mut val: f64) -> String {
    val.setmin(1.0);
    val.setmax(0.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (
            30. * (1.0 - x) + 144. * x,
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
        )
    } else {
        let x = val * 2.0 - 1.0;
        (
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
            30. * (1.0 - x) + 70. * x,
        )
    };
    format!(
        "#{:02x}{:02x}{:02x}c0",
        r.round() as i32,
        g.round() as i32,
        b.round() as i32
    )
}

pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

pub fn group(title: String) -> Group {
    Group::new().add(Title::new(title))
}

pub fn vis_default(input: &Input, out: &Output) -> (i64, String, String) {
    let (mut score, err, svg) = vis(input, out, out.actions.len());
    if err.len() > 0 {
        score = 0;
    }
    (score, err, svg)
}

pub fn vis(input: &Input, out: &Output, t: usize) -> (i64, String, String) {
    let D = 600 / input.N;
    let W = D * input.N;
    let H = D * input.N;
    let (score, err, state) = compute_score_details(input, out, t);
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");
    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central;}}"
    )));
    for i in 0..input.N {
        for j in 0..input.N {
            let mut group = group(format!("({}, {})", i, j)).add(rect(
                j * D,
                i * D,
                D,
                D,
                &if state.used[i][j] == !0 {
                    "white".to_owned()
                } else {
                    color(state.used[i][j] as f64 / (input.M - 1) as f64)
                },
            ));
            if let Some(k) = (0..input.M).rfind(|&k| state.ps[k] == (i, j)) {
                group = group.add(
                    Circle::new()
                        .set("cx", j * D + D / 2)
                        .set("cy", i * D + D / 2)
                        .set("r", D / 2 - 2)
                        .set("fill", "white")
                        .set("stroke", "black")
                        .set("stroke-width", 1),
                );
                group = group.add(
                    Text::new(format!("{}", k))
                        .set("x", j * D + D / 2)
                        .set("y", i * D + D / 2)
                        .set("font-size", D * 2 / 3)
                        .set("fill", "black"),
                );
            }
            doc = doc.add(group);
        }
    }
    for i in 0..=input.N {
        doc = doc.add(
            Line::new()
                .set("x1", 0)
                .set("y1", i * D)
                .set("x2", input.N * D)
                .set("y2", i * D)
                .set(
                    "stroke",
                    if i == 0 || i == input.N {
                        "black"
                    } else {
                        "lightgray"
                    },
                )
                .set("stroke-width", if i == 0 || i == input.N { 2 } else { 1 }),
        );
        doc = doc.add(
            Line::new()
                .set("x1", i * D)
                .set("y1", 0)
                .set("x2", i * D)
                .set("y2", input.N * D)
                .set(
                    "stroke",
                    if i == 0 || i == input.N {
                        "black"
                    } else {
                        "lightgray"
                    },
                )
                .set("stroke-width", if i == 0 || i == input.N { 2 } else { 1 }),
        );
    }
    for i in 0..input.N {
        for j in 0..input.N {
            if j + 1 < input.N {
                if input.wall_v[i][j] {
                    doc = doc.add(
                        Line::new()
                            .set("x1", j * D + D)
                            .set("y1", i * D)
                            .set("x2", j * D + D)
                            .set("y2", (i + 1) * D)
                            .set("stroke", "black")
                            .set("stroke-width", 2),
                    );
                }
            }
            if i + 1 < input.N {
                if input.wall_h[i][j] {
                    doc = doc.add(
                        Line::new()
                            .set("x1", j * D)
                            .set("y1", i * D + D)
                            .set("x2", (j + 1) * D)
                            .set("y2", i * D + D)
                            .set("stroke", "black")
                            .set("stroke-width", 2),
                    );
                }
            }
        }
    }
    (score, err, doc.to_string())
}
