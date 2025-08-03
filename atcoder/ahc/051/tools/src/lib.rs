#![allow(non_snake_case, unused_macros)]

use itertools::Itertools;
use proconio::input;
use rand::prelude::*;
use std::{collections::VecDeque, ops::RangeBounds};
use svg::node::element::{Circle, Group, Line, Rectangle, Style, Title};

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
    pub pos: Vec<(i64, i64)>,
    pub ps: Vec<Vec<f64>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {}", self.N, self.M, self.K)?;
        for (x, y) in &self.pos {
            writeln!(f, "{} {}", x, y)?;
        }
        for ps in &self.ps {
            writeln!(f, "{:.4}", ps.iter().format(" "))?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        N: usize, M: usize, K: usize,
        pos: [(i64, i64); N + M],
        ps: [[f64; N]; K],
    }
    Input { N, M, K, pos, ps }
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

pub struct Output {
    pub ds: Vec<usize>,
    pub s: usize,
    pub cs: Vec<(usize, usize, usize)>,
}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut f = f.split_whitespace();
    let mut ds = vec![];
    for _ in 0..input.N {
        ds.push(read(f.next(), 0..input.N)?);
    }
    let s = read(f.next(), 0..input.N + input.M)?;
    let mut cs = vec![];
    for _ in 0..input.M {
        let k = read(f.next(), -1..(input.K as i32))?;
        let k = if k < 0 { !0 } else { k as usize };
        let (v1, v2) = if k == !0 {
            (!0, !0)
        } else {
            (
                read(f.next(), 0..input.N + input.M)?,
                read(f.next(), 0..input.N + input.M)?,
            )
        };
        cs.push((k, v1, v2));
    }
    if f.next().is_some() {
        return Err("Too many output".to_owned());
    }
    Ok(Output { ds, s, cs })
}

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let N = rng.gen_range(5i32..=20) as usize;
    let M = rng.gen_range((10 * N) as i32..=(50 * N) as i32) as usize;
    let K = rng.gen_range((N as i32)..=(4 * N as i32)) as usize;
    let mut pos = vec![];
    let mut S = vec![(0, 5000)];
    while pos.len() < N + M {
        let x = rng.gen_range(0i64..=10000);
        let y = rng.gen_range(0i64..=10000);
        if S.iter()
            .all(|&(px, py)| (px - x).pow(2) + (py - y).pow(2) > 10000)
        {
            pos.push((x, y));
            S.push((x, y));
        }
    }
    let mut ps = mat![0.0; K; N];
    for i in 0..K {
        for j in 0..N {
            ps[i][j] = rng.gen_range(1000i32..=9000) as f64 * 1e-4;
        }
    }
    Input { N, M, K, pos, ps }
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, out);
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

pub fn compute_score_details(input: &Input, out: &Output) -> (i64, String, Vec<Vec<f64>>) {
    let mut probs = mat![0.0; input.N + input.M; input.N];
    let mut tmp = out.ds.clone();
    tmp.sort();
    tmp.dedup();
    if tmp.len() != input.N {
        return (0, format!("Duplicated d"), probs);
    }
    if out.s >= input.N && out.cs[out.s - input.N].0 == !0 {
        return (0, format!("Invalid s"), probs);
    }
    let mut segs = vec![((0, 5000), input.pos[out.s])];
    let mut g = vec![vec![]; input.N + input.M];
    for i in 0..input.M {
        if out.cs[i].0 == !0 {
            continue;
        }
        if out.cs[i].1 >= input.N && out.cs[out.cs[i].1 - input.N].0 == !0 {
            return (0, format!("Invalid v1 in the {}-th position", i), probs);
        }
        if out.cs[i].2 >= input.N && out.cs[out.cs[i].2 - input.N].0 == !0 {
            return (0, format!("Invalid v2 in the {}-th position", i), probs);
        }
        segs.push((input.pos[input.N + i], input.pos[out.cs[i].1]));
        segs.push((input.pos[input.N + i], input.pos[out.cs[i].2]));
        g[input.N + i].push(out.cs[i].1);
        g[input.N + i].push(out.cs[i].2);
    }
    for i in 0..segs.len() {
        for j in 0..i {
            if segs[i].0 == segs[j].0
                || segs[i].0 == segs[j].1
                || segs[i].1 == segs[j].0
                || segs[i].1 == segs[j].1
            {
                continue;
            }
            if segments_intersect(segs[i].0, segs[i].1, segs[j].0, segs[j].1) {
                return (
                    0,
                    format!(
                        "Belt conveyors are intersecting. ({},{})-({},{}), ({},{})-({},{})",
                        segs[i].0 .0,
                        segs[i].0 .1,
                        segs[i].1 .0,
                        segs[i].1 .1,
                        segs[j].0 .0,
                        segs[j].0 .1,
                        segs[j].1 .0,
                        segs[j].1 .1
                    ),
                    probs,
                );
            }
        }
    }
    let Some(order) = topological_sort(&g) else {
        return (0, "Graph contains a cycle".to_owned(), probs);
    };
    probs[out.s].fill(1.0);
    for u in order {
        if u >= input.N {
            let u = u - input.N;
            let k = out.cs[u].0;
            if k == !0 {
                continue;
            }
            let (v1, v2) = (out.cs[u].1, out.cs[u].2);
            for i in 0..input.N {
                probs[v1][i] += probs[u + input.N][i] * input.ps[k][i];
                probs[v2][i] += probs[u + input.N][i] * (1.0 - input.ps[k][i]);
            }
        }
    }
    let mut score = 0.0;
    for i in 0..input.N {
        let d = out.ds[i];
        let q = probs[i][d];
        score += 1.0 - q;
    }
    score /= input.N as f64;
    let score = (1e9 * score).round() as i64;
    (score, String::new(), probs)
}

fn segments_intersect(p1: (i64, i64), p2: (i64, i64), q1: (i64, i64), q2: (i64, i64)) -> bool {
    if p1.0.max(p2.0) < q1.0.min(q2.0)
        || q1.0.max(q2.0) < p1.0.min(p2.0)
        || p1.1.max(p2.1) < q1.1.min(q2.1)
        || q1.1.max(q2.1) < p1.1.min(p2.1)
    {
        return false;
    }
    fn orientation(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> i64 {
        let cross = (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
        cross.signum()
    }
    let o1 = orientation(p1, p2, q1);
    let o2 = orientation(p1, p2, q2);
    let o3 = orientation(q1, q2, p1);
    let o4 = orientation(q1, q2, p2);
    (o1 * o2 <= 0) && (o3 * o4 <= 0)
}

pub fn topological_sort(adj: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut in_deg = vec![0; n];
    for u in 0..n {
        for &v in &adj[u] {
            in_deg[v] += 1;
        }
    }
    let mut queue = VecDeque::new();
    for u in 0..n {
        if in_deg[u] == 0 {
            queue.push_back(u);
        }
    }
    let mut order = Vec::with_capacity(n);
    while let Some(u) = queue.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            in_deg[v] -= 1;
            if in_deg[v] == 0 {
                queue.push_back(v);
            }
        }
    }
    if order.len() == n {
        Some(order)
    } else {
        None
    }
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
        "#{:02x}{:02x}{:02x}",
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
    let (mut score, err, svg) = vis(input, &out, !0);
    if err.len() > 0 {
        score = 0;
    }
    (score, err, svg)
}

pub fn vis(input: &Input, out: &Output, target: usize) -> (i64, String, String) {
    let W = 600;
    let H = 600;
    let (score, err, probs) = compute_score_details(input, &out);
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");
    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central;}}"
    )));
    let c = if target == !0 {
        "gray".to_owned()
    } else {
        color(1.0)
    };
    fn get_x(p: (i64, i64)) -> i64 {
        p.0 * 600 / 10000
    }
    fn get_y(p: (i64, i64)) -> i64 {
        600 - p.1 * 600 / 10000
    }
    let p = input.pos[out.s];
    doc = doc.add(
        group(if target == !0 {
            format!("edge: inlet - {}", out.s)
        } else {
            format!("edge: inlet - {}\nprob = 1.0", out.s)
        })
        .add(
            Line::new()
                .set("x1", 0)
                .set("y1", H / 2)
                .set("x2", get_x(p))
                .set("y2", get_y(p))
                .set("stroke", c.clone())
                .set("stroke-width", 2),
        ),
    );
    doc = doc.add(
        group(if target == !0 {
            format!("vertex: inlet")
        } else {
            format!("vertex: inlet\nprob = 1.0")
        })
        .add(
            Circle::new()
                .set("cx", 0)
                .set("cy", H / 2)
                .set("r", 3)
                .set("fill", c.clone()),
        ),
    );
    // edge
    for i in 0..input.M {
        if out.cs[i].0 == !0 {
            continue;
        }
        let pr = if target == !0 {
            1.0
        } else {
            probs[input.N + i][target]
        };
        let k = out.cs[i].0;
        let (v1, v2) = (out.cs[i].1, out.cs[i].2);
        let p = input.pos[input.N + i];
        let q1 = input.pos[v1];
        let q2 = input.pos[v2];
        let (pr1, pr2) = if target == !0 {
            (1.0, 1.0)
        } else {
            (pr * input.ps[k][target], pr * (1.0 - input.ps[k][target]))
        };
        let (c1, c2) = if target == !0 {
            ("gray".to_owned(), "gray".to_owned())
        } else {
            (color(pr1), color(pr2))
        };
        doc = doc.add(
            group(if target == !0 {
                format!("edge: {} - {}", input.N + i, v1)
            } else {
                format!("edge: {} - {}\nprob = {}", input.N + i, v1, pr1)
            })
            .add(
                Line::new()
                    .set("x1", get_x(p))
                    .set("y1", get_y(p))
                    .set("x2", get_x(q1))
                    .set("y2", get_y(q1))
                    .set("stroke", c1.clone())
                    .set("stroke-width", 2),
            ),
        );
        doc = doc.add(
            group(if target == !0 {
                format!("edge: {} - {}", input.N + i, v2)
            } else {
                format!("edge: {} - {}\nprob = {}", input.N + i, v2, pr2)
            })
            .add(
                Line::new()
                    .set("x1", get_x(p))
                    .set("y1", get_y(p))
                    .set("x2", get_x(q2))
                    .set("y2", get_y(q2))
                    .set("stroke", c2.clone())
                    .set("stroke-width", 2),
            ),
        );
    }
    // vertex
    for i in 0..input.M {
        let pr = if target == !0 {
            1.0
        } else {
            probs[input.N + i][target]
        };
        let k = out.cs[i].0;
        let k = if k == !0 {
            format!("no sorter")
        } else {
            format!("sorter {}", k)
        };
        let c = if target == !0 {
            "gray".to_owned()
        } else {
            color(pr)
        };
        doc = doc.add(
            group(if target == !0 {
                format!("vertex: {} ({})", input.N + i, k)
            } else {
                format!("vertex: {} ({})\nprob = {}", input.N + i, k, pr)
            })
            .add(
                Circle::new()
                    .set("cx", get_x(input.pos[input.N + i]))
                    .set("cy", get_y(input.pos[input.N + i]))
                    .set("r", 3)
                    .set("fill", c),
            ),
        );
    }
    for i in 0..input.N {
        let pr = if target == !0 { 1.0 } else { probs[i][target] };
        let d = out.ds[i];
        let c = if target == !0 {
            "gray".to_owned()
        } else {
            color(pr)
        };
        doc = doc.add(
            group(if target == !0 {
                format!("vertex: {} (processor {})", i, d)
            } else {
                format!("vertex: {} (processor {})\nprob = {}", i, d, pr)
            })
            .add(
                Circle::new()
                    .set("cx", get_x(input.pos[i]))
                    .set("cy", get_y(input.pos[i]))
                    .set("r", if d == target { 6 } else { 4 })
                    .set("fill", c)
                    .set("stroke", "black")
                    .set("stroke-width", if d == target { 2 } else { 1 }),
            ),
        );
    }
    (score, err, doc.to_string())
}
