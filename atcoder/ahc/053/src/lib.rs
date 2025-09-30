#![allow(non_snake_case, unused_macros)]

use itertools::Itertools;
use proconio::input;
use rand::prelude::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::ChildStdout;

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub m: usize,
    pub l: u64,
    pub u: u64,
    pub b: Vec<u64>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {} {}\n{}",
            self.n,
            self.m,
            self.l,
            self.u,
            self.b.iter().join(" ")
        )?;
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        n: usize, m: usize, l: u64, u: u64, b: [u64; m],
    }
    Input { n, m, l, u, b }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr>(
    token: Option<&str>,
    lb: T,
    ub: T,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if v < lb || ub < v {
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
    pub a: Vec<u64>,
    pub x: Vec<usize>,
}

const N: usize = 500;
const M: usize = 50;
const U: u64 = 1_002_000_000_000_000;
const L: u64 = 998_000_000_000_000;
const AMAX: u64 = U;

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut tokens = f.split_whitespace();
    let mut a = Vec::new();
    for _ in 0..input.n {
        a.push(read(tokens.next(), 1, AMAX)?)
    }
    let mut x = Vec::new();
    for _ in 0..input.n {
        x.push(read(tokens.next(), 0, input.m)?)
    }
    Ok(Output { a, x })
}

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let mut b = Vec::new();
    let distr = rand_distr::Uniform::new_inclusive(L, U);
    while b.len() < M {
        let x = distr.sample(&mut rng);
        b.push(x as u64);
    }
    b.sort();
    Input {
        n: N,
        m: M,
        l: L,
        u: U,
        b,
    }
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, &out);
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

pub fn compute_score_details(input: &Input, out: &Output) -> (i64, String, ()) {
    let mut sum = vec![0u64; input.m];
    for i in 0..input.n {
        if out.x[i] > 0 {
            sum[out.x[i] - 1] += out.a[i];
        }
    }
    let mut diff = 1;
    for j in 0..input.m {
        diff += sum[j].abs_diff(input.b[j]) as i64
    }

    let score = ((20.0 - (diff as f64).log10()) * 5e7).round() as i64;
    (score, String::new(), ())
}

/// 0 <= val <= 1
pub fn color(mut val: f64) -> String {
    val = val.clamp(0.0, 1.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (
            0. * (1.0 - x) + 255. * x,
            112. * (1.0 - x) + 255. * x,
            192. * (1.0 - x) + 255. * x,
        )
    } else {
        let x = val * 2.0 - 1.0;
        (
            255. * (1.0 - x) + 208. * x,
            255. * (1.0 - x) + 64. * x,
            255. * (1.0 - x) + 64. * x,
        )
    };
    format!(
        "#{:02x}{:02x}{:02x}",
        r.round() as i32,
        g.round() as i32,
        b.round() as i32
    )
}

pub fn vis_default(input: &Input, out: &Output) -> (i64, String, String) {
    let (mut score, err, svg) = vis(input, &out, input.n);
    if err.len() > 0 {
        score = 0;
    }
    (score, err, svg)
}

pub fn vis(input: &Input, output: &Output, t: usize) -> (i64, String, String) {
    use svg::node::element::path::Data as SvgData;
    use svg::node::element::Group as SvgGroup;
    use svg::node::element::Path as SvgPath;
    use svg::node::element::Text as SvgText;
    use svg::Document;

    const ZOOM: f32 = 8.0;
    const MERGIN: f32 = 8.0;
    const BOX_H: f32 = 104.0 / 26.0 * ZOOM;
    const BOX_W: f32 = 100.0 / 2.0 * ZOOM;

    const BOX_NUM_POINT: (f32, f32) = (2.0 * ZOOM, BOX_H / 2.0);
    const BOX_NUM_FONT_SIZE: f32 = 2.0 * ZOOM;
    const BOX_RECT: (f32, f32, f32, f32) = (4.0 * ZOOM, 0.0 * ZOOM, BOX_W, BOX_H);
    const BOX_A_POINT: (f32, f32) = (24.0 * ZOOM, BOX_H * 1.0 / 4.0);
    const BOX_B_POINT: (f32, f32) = (24.0 * ZOOM, BOX_H * 3.0 / 4.0);
    const BOX_C_POINT: (f32, f32) = (49.0 * ZOOM, BOX_H * 3.0 / 4.0);
    const A_FONT_SIZE: f32 = 1.8 * ZOOM;

    const BOX_STROKE_WIDTH: f32 = 0.1 * ZOOM;

    const MOD: usize = 25;

    let mut svg = Document::new()
        .set("id", "vis")
        .set("width", (800.0 + MERGIN * 2.0) as i64)
        .set("style", "background: white")
        .set("height", (BOX_H * 26.0 + MERGIN * 2.0) as i64)
        .set(
            "viewBox",
            (
                -MERGIN,
                -MERGIN,
                BOX_W * ((input.m + MOD - 1) / MOD) as f32 + MERGIN * 2.0,
                BOX_H * (MOD + 1) as f32 + MERGIN * 2.0,
            ),
        );

    let mut belong = vec![vec![]; input.m + 1];
    let mut poll_sum = vec![0; input.m + 1];

    for i in 0..t {
        if output.x[i] > 0 {
            belong[output.x[i] - 1].push(i);
            poll_sum[output.x[i] - 1] += output.a[i];
        } else {
            belong[input.m].push(i);
            poll_sum[input.m] += output.a[i];
        }
    }

    for i in 0..=input.m {
        let mut group = SvgGroup::new().set(
            "onclick",
            format!(
                "view_info({}, {}, [{}])",
                i,
                input.b.get(i).cloned().unwrap_or(0),
                belong[i]
                    .iter()
                    .map(|k| format!("{{ k: {}, a: {} }}", k + 1, output.a[*k]))
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        );

        let base = if i < input.m {
            (BOX_W * (i / MOD) as f32, BOX_H * (i % MOD) as f32)
        } else {
            (BOX_W * 1 as f32, BOX_H * 25 as f32)
        };
        let data = SvgData::new()
            .move_to((BOX_RECT.0 + base.0, BOX_RECT.1 + base.1))
            .line_to((BOX_RECT.0 + base.0, BOX_RECT.3 + base.1))
            .line_to((BOX_RECT.2 + base.0, BOX_RECT.3 + base.1))
            .line_to((BOX_RECT.2 + base.0, BOX_RECT.1 + base.1))
            .close();

        if i < input.m {
            let mut rate = 0.5;
            let diff = poll_sum[i] as i64 - input.b[i] as i64;
            if diff != 0 {
                rate = 0.5 - (0.15 + 0.35 * (diff.abs() as f64).log10() / 12.0);
            }
            if diff < 0 {
                rate = 1.0 - rate;
            }
            let path = SvgPath::new()
                .set("d", data)
                .set("fill", "white")
                .set("stroke-width", BOX_STROKE_WIDTH)
                .set("stroke", "black");
            group = group.add(path);

            {
                let width_rate = if diff > 0 {
                    0.5 + 0.5 * ((1.0 + diff.abs() as f64).log10() / 15.0).clamp(0.0, 1.0)
                } else {
                    0.5 - 0.5 * ((1.0 + diff.abs() as f64).log10() / 15.0).clamp(0.0, 1.0)
                };
                let color = if diff != 0 {
                    color(rate)
                } else {
                    String::from("#c0ffc0")
                };
                let left = BOX_RECT.0 + base.0;
                let top = BOX_RECT.1 + base.1;
                let right = left + width_rate as f32 * (BOX_RECT.2 - BOX_RECT.0) as f32;
                let bot = BOX_RECT.3 + base.1;
                let data = SvgData::new()
                    .move_to((left, top))
                    .line_to((left, bot))
                    .line_to((right, bot))
                    .line_to((right, top))
                    .close();
                let path = SvgPath::new()
                    .set("d", data)
                    .set("fill", color)
                    .set("stroke-width", 0);
                group = group.add(path);
            }
        } else {
            let path = SvgPath::new()
                .set("d", data)
                .set("fill", "white")
                .set("stroke-width", BOX_STROKE_WIDTH)
                .set("stroke", "black");
            group = group.add(path);
        }

        let text_i = SvgText::new(format!("{}", if i < input.m { i + 1 } else { 0 }))
            .set("x", BOX_NUM_POINT.0 + base.0)
            .set("y", BOX_NUM_POINT.1 + base.1)
            .set("font-size", BOX_NUM_FONT_SIZE)
            .set("font-family", "monospace")
            .set("dominant-baseline", "central")
            .set("text-anchor", "middle");
        group = group.add(text_i);

        let str_a = format!("{}", poll_sum[i]);
        let str_b = input
            .b
            .get(i)
            .map(|x| x.to_string())
            .unwrap_or(String::new());

        let text_a = SvgText::new(format!("{}", str_a))
            .set("x", BOX_A_POINT.0 + base.0)
            .set("y", BOX_A_POINT.1 + base.1)
            .set("font-size", A_FONT_SIZE)
            .set("font-family", "monospace")
            .set("dominant-baseline", "central")
            .set("text-anchor", "end");
        group = group.add(text_a);

        if i < input.m {
            let text_b = SvgText::new(format!("{}", str_b))
                .set("x", BOX_B_POINT.0 + base.0)
                .set("y", BOX_B_POINT.1 + base.1)
                .set("font-size", A_FONT_SIZE)
                .set("font-family", "monospace")
                .set("dominant-baseline", "central")
                .set("text-anchor", "end");
            group = group.add(text_b);
        }
        if i < input.m {
            let diff = poll_sum[i] as i64 - input.b[i] as i64;
            let log10_diff = if diff != 0 {
                (diff.abs() as f64).log10()
            } else {
                0.0
            };
            let text_c = SvgText::new(format!("{}\nlog10: {:.2}", diff, log10_diff))
                .set("x", BOX_C_POINT.0 + base.0)
                .set("y", BOX_C_POINT.1 + base.1)
                .set("font-size", A_FONT_SIZE)
                .set("font-family", "monospace")
                .set("dominant-baseline", "central")
                .set("text-anchor", "end");
            group = group.add(text_c);
        }

        svg = svg.add(group);
    }

    (
        compute_score(input, output).0,
        String::new(),
        svg.to_string(),
    )
}

fn read_line(stdout: &mut BufReader<ChildStdout>, local: bool) -> Result<String, String> {
    loop {
        let mut out = String::new();
        match stdout.read_line(&mut out) {
            Ok(0) | Err(_) => {
                return Err(format!("Your program has terminated unexpectedly"));
            }
            _ => (),
        }
        if local {
            print!("{}", out);
        }
        let v = out.trim();
        if v.len() == 0 {
            continue;
        }
        return Ok(v.to_owned());
    }
}

pub fn exec(p: &mut std::process::Child, local: bool) -> Result<i64, String> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let input = parse_input(&input);
    let mut stdin = std::io::BufWriter::new(p.stdin.take().unwrap());
    let mut stdout = std::io::BufReader::new(p.stdout.take().unwrap());

    // write N M L U
    let _ = writeln!(stdin, "{} {} {} {}", input.n, input.m, input.l, input.u,);
    let _ = stdin.flush();

    // read A
    let line = read_line(&mut stdout, local)?;
    let a: Vec<u64> = line
        .split_whitespace()
        .map(|v| read(Some(v), 1, AMAX))
        .collect::<Result<Vec<_>, _>>()?;

    if a.len() < input.n {
        return Err(format!("too few input A"));
    }
    if a.len() > input.n {
        return Err(format!("too many input A"));
    }

    // write B
    let _ = writeln!(stdin, "{}", input.b.iter().join(" "));
    let _ = stdin.flush();

    // read X
    let line = read_line(&mut stdout, local)?;
    let x: Vec<usize> = line
        .split_whitespace()
        .map(|v| read(Some(v), 0, input.m))
        .collect::<Result<Vec<_>, _>>()?;
    if x.len() < input.n {
        return Err(format!("too few input X"));
    }
    if x.len() > input.n {
        return Err(format!("too many input X"));
    }

    p.wait().unwrap();
    let out = Output { a, x };

    let (score, err) = compute_score(&input, &out);

    if err.is_empty() {
        Ok(score)
    } else {
        Err(err)
    }
}
