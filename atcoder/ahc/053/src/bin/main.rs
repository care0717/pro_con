use rand::Rng;
use std::collections::BinaryHeap;

const EPS: usize = 1000000000000;

// m個分lを生成し、残りは1 ~ u-lのランダムな数を生成
fn gen_cards(n: usize, m: usize, l: usize, u: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut cards: Vec<usize> = vec![];
    for _ in 0..20 {
        cards.push(l);
    }
    for _ in 20..40 {
        cards.push(l + EPS);
    }
    for _ in 40..m {
        cards.push(l + EPS * 2);
    }
    for _ in 0..50 {
        cards.push(EPS);
    }
    for _ in 0..50 {
        cards.push(rng.gen_range(EPS / 3..3 * EPS / 4));
    }
    for _ in 0..200 {
        cards.push(rng.gen_range(EPS / 500..EPS / 10));
    }
    for _ in 0..100 {
        cards.push(rng.gen_range(EPS / 50000..EPS / 1000));
    }
    for _ in 0..50 {
        cards.push(rng.gen_range(EPS / 50000..EPS / 10000));
    }
    while cards.len() < n {
        cards.push(rng.gen_range(EPS / 500000..EPS / 100000));
    }
    cards.sort_by(|a, b| b.cmp(a));
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

// 現在のsumとbの差が一番大きいindexを常にpriority queueで管理し、sumがbに近づくようにplacementを更新
fn gen_gready_placement(b: &Vec<usize>, cards: &Vec<usize>) -> Vec<usize> {
    let m = b.len();
    let mut tmp_cards = cards.clone();
    let mut sum: Vec<usize> = vec![0; m];
    let mut priority_queue = BinaryHeap::new();
    for i in 0..m {
        priority_queue.push((sum[i].abs_diff(b[i]), i));
    }
    let mut placement: Vec<usize> = vec![];
    while tmp_cards.len() > 0 {
        let (diff, i) = priority_queue.pop().unwrap();
        if sum[i] + tmp_cards[0] < b[i] {
            sum[i] += tmp_cards[0];
            placement.push(i + 1);
            priority_queue.push((sum[i].abs_diff(b[i]), i));
        } else {
            priority_queue.push((diff, i));
            placement.push(0);
        }
        tmp_cards.remove(0);
    }
    placement
}

fn main() {
    use std::io::{self, Write};

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

    let placement = gen_gready_placement(&b, &cards);

    println!(
        "{}",
        placement
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    std::io::stdout().flush().unwrap();
}
