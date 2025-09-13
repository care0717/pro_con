use rand::Rng;

const EPS: usize = 1000000000000;

// m個分lを生成し、残りは1 ~ u-lのランダムな数を生成
fn gen_cards(n: usize, m: usize, l: usize, u: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut cards: Vec<usize> = vec![];
    for _ in 0..m {
        cards.push(rng.gen_range(l..=u) - EPS / 5);
    }
    cards.sort();
    while cards.len() < n {
        cards.push(rng.gen_range(1..EPS / 17));
    }
    cards
}

fn compute_diff(b: &Vec<usize>, cards: &Vec<usize>, placement: &Vec<usize>) -> usize {
    let m = b.len();
    let n = cards.len();
    let mut sum = vec![0; m];
    for i in 0..n {
        if placement[i] == 0 {
            continue;
        }
        sum[placement[i] - 1] += cards[i];
    }
    let mut diff = 0;
    for i in 0..m {
        diff += sum[i].abs_diff(b[i]);
    }
    diff
}

fn gen_random_placement(n: usize, m: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut placement: Vec<usize> = vec![];
    for i in 0..m {
        placement.push(i + 1);
    }
    while placement.len() < n {
        placement.push(rng.gen_range(1..m + 1));
    }

    placement
}

fn main() {
    use std::io::{self, Write};
    let start_time: std::time::Instant = std::time::Instant::now();

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let mut iter = line.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let l: usize = iter.next().unwrap().parse().unwrap();
    let u: usize = iter.next().unwrap().parse().unwrap();
    let cards = gen_cards(n, m, l, u);

    println!(
        "{}",
        cards
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    std::io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let b: Vec<usize> = line
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let placement = gen_random_placement(n, m);

    // 1.8sec以内で、diffを最小化するplacementを生成
    // placementのm~n-1までをランダムに選び、半分の確率で0、もう半分の確率で1~mのランダムな数を生成
    // これをdiffが小さくなったら採択する焼きなましをする
    let mut best_placement = placement;
    let mut best_diff = compute_diff(&b, &cards, &best_placement);
    let time_limit = std::time::Duration::from_millis(1800);
    let mut rng = rand::thread_rng();
    while start_time.elapsed() < time_limit {
        let mut new_placement = best_placement.clone();
        let i = rng.gen_range(m..n);
        let mut temperature = 1.0 - start_time.elapsed().as_secs_f64() / time_limit.as_secs_f64();
        if temperature < 0.0 {
            temperature = 0.0;
        }

        new_placement[i] = rng.gen_range(0..m + 1);

        let new_diff = compute_diff(&b, &cards, &new_placement);
        if new_diff < best_diff || rng.gen_bool(temperature) {
            best_diff = new_diff;
            best_placement = new_placement;
        }
    }

    println!(
        "{}",
        best_placement
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    std::io::stdout().flush().unwrap();
}
