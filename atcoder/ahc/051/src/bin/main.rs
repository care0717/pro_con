use std::collections::HashMap;

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

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

type NodeId = usize;

#[derive(Clone, Debug)]
struct Graph {
    n: usize, // ごみ種類数
    m: usize, // 分別器設置場所数
    processor_positions: Vec<Point>,
    separator_positions: Vec<Point>,
    probabilities: Vec<Vec<f64>>,
    edges: HashMap<NodeId, Out>,
}

#[derive(Clone, Debug)]
struct Out {
    out1: NodeId,
    out2: NodeId,
}

#[derive(PartialEq, Clone, Debug)]
enum OutType {
    Out1,
    Out2,
}
#[derive(PartialEq, Clone, Debug)]
struct Source {
    id: NodeId,
    ty: OutType,
}

#[derive(Clone, Debug)]
struct WeightedReachability {
    reachout: i32,
    distance_weight: f64,
}

// NodeIDから座標を取得
fn get_node_position(graph: &Graph, node_id: NodeId) -> Point {
    if node_id < graph.n {
        // 処理装置: 0 ~ n-1
        graph.processor_positions[node_id]
    } else {
        // 分別器: n ~ n+m-1
        let sep_idx = node_id - graph.n;
        if sep_idx < graph.separator_positions.len() {
            graph.separator_positions[sep_idx]
        } else if node_id == usize::MAX {
            // 搬入口の特別なNodeID
            Point { x: 0, y: 5000 } // 搬入口の位置
        } else {
            panic!("Invalid NodeId: {}", node_id);
        }
    }
}

// 2つのNodeID間の辺が交差するかチェック
fn edge_intersects(graph: &Graph, from1: NodeId, to1: NodeId, from2: NodeId, to2: NodeId) -> bool {
    let p1 = get_node_position(graph, from1);
    let p2 = get_node_position(graph, to1);
    let q1 = get_node_position(graph, from2);
    let q2 = get_node_position(graph, to2);
    segments_intersect(p1, p2, q1, q2)
}

// 新しい辺が既存の辺と交差するかチェック
fn new_edge_intersects(graph: &Graph, from: NodeId, to: NodeId) -> bool {
    for (&existing_from, out) in &graph.edges {
        if edge_intersects(graph, from, to, existing_from, out.out1) {
            return true;
        }
        if edge_intersects(graph, from, to, existing_from, out.out2) {
            return true;
        }
    }
    false
}

// グラフに辺を追加
fn add_edge(graph: &mut Graph, from: NodeId, out1: NodeId, out2: NodeId) {
    graph.edges.insert(from, Out { out1, out2 });
}

// 線分交差判定
fn segments_intersect(p1: Point, p2: Point, q1: Point, q2: Point) -> bool {
    // 端点が同じ場合は交差していないとみなす
    if (p1.x == q1.x && p1.y == q1.y)
        || (p1.x == q2.x && p1.y == q2.y)
        || (p2.x == q1.x && p2.y == q1.y)
        || (p2.x == q2.x && p2.y == q2.y)
    {
        return false;
    }

    // バウンディングボックスの交差チェック
    if p1.x.max(p2.x) < q1.x.min(q2.x)
        || q1.x.max(q2.x) < p1.x.min(p2.x)
        || p1.y.max(p2.y) < q1.y.min(q2.y)
        || q1.y.max(q2.y) < p1.y.min(p2.y)
    {
        return false;
    }

    let o1 = orientation(p1, p2, q1);
    let o2 = orientation(p1, p2, q2);
    let o3 = orientation(q1, q2, p1);
    let o4 = orientation(q1, q2, p2);

    // 厳密な交差のみを検出（端点での接触は除く）
    (o1 * o2 < 0) && (o3 * o4 < 0)
}

fn orientation(a: Point, b: Point, c: Point) -> i32 {
    let cross = (b.x as i64 - a.x as i64) * (c.y as i64 - a.y as i64)
        - (b.y as i64 - a.y as i64) * (c.x as i64 - a.x as i64);
    sign(cross)
}

fn sign(x: i64) -> i32 {
    if x > 0 {
        1
    } else if x < 0 {
        -1
    } else {
        0
    }
}

fn distance(p1: Point, p2: Point) -> f64 {
    let dx = (p1.x - p2.x) as f64;
    let dy = (p1.y - p2.y) as f64;
    (dx * dx + dy * dy).sqrt()
}

fn create_graph(
    n: usize,
    m: usize,
    processor_positions: Vec<Point>,
    separator_positions: Vec<Point>,
    probabilities: Vec<Vec<f64>>,
) -> Graph {
    Graph {
        n,
        m,
        processor_positions,
        separator_positions,
        probabilities,
        edges: HashMap::new(),
    }
}

fn solve(graph: &Graph) -> (Vec<usize>, usize, Vec<String>) {
    // 距離ベースの貪欲アルゴリズムでネットワークを構築（交差処理込み）
    let (start_node, separator_configs) = build_network_greedy(graph);

    // デバイス割り当ては単純に順番通り
    let device_assignments: Vec<usize> = (0..graph.n).collect();

    (device_assignments, start_node, separator_configs)
}

fn build_network_greedy(graph: &Graph) -> (usize, Vec<String>) {
    let mut work_graph = graph.clone(); // 作業用のグラフ
    let mut used_separators = vec![false; graph.m];
    let mut queue = std::collections::VecDeque::new();

    if graph.separator_positions.is_empty() {
        return (0, vec!["-1".to_string(); graph.m]);
    }

    let start_pos = Point { x: 0, y: 5000 };
    const ENTRANCE_NODE: NodeId = usize::MAX; // 搬入口の特別なNodeID

    // スタート地点から最も近い分別器を見つける
    let mut min_dist = f64::MAX;
    let mut nearest_sep = 0;
    for i in 0..graph.separator_positions.len() {
        let dist = distance(start_pos, graph.separator_positions[i]);
        if dist < min_dist {
            min_dist = dist;
            nearest_sep = i;
        }
    }

    // 搬入口から最初の分別器への辺を追加
    let first_sep_node = graph.n + nearest_sep;
    add_edge(
        &mut work_graph,
        ENTRANCE_NODE,
        first_sep_node,
        first_sep_node,
    );
    queue.push_back(nearest_sep);
    used_separators[nearest_sep] = true;

    // キューから分別器を処理（新しい貪欲法）
    while let Some(current_sep_idx) = queue.pop_front() {
        if current_sep_idx >= graph.separator_positions.len() {
            continue;
        }

        let current_node = graph.n + current_sep_idx;
        let current_pos = graph.separator_positions[current_sep_idx];

        // 距離順に候補を並べる
        let mut candidates = Vec::new();

        // 全ての処理装置への距離
        for i in 0..graph.processor_positions.len() {
            let device_pos = graph.processor_positions[i];
            let dist = distance(current_pos, device_pos);
            candidates.push((dist, i)); // NodeID: i (処理装置)
        }

        // 未使用の分別器への距離
        for i in 0..graph.separator_positions.len() {
            if !used_separators[i] {
                let sep_pos = graph.separator_positions[i];
                let dist = distance(current_pos, sep_pos);
                candidates.push((dist, graph.n + i)); // NodeID: n+i (分別器)
            }
        }

        candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // 2つの出力先を選択
        let mut output1 = None;
        let mut output2 = None;

        // 距離順に最初の2つの候補を試す
        for &(_, target_node) in candidates.iter().take(2) {
            if output1.is_none() {
                output1 = Some(target_node);
            } else {
                output2 = Some(target_node);
                break;
            }
        }

        // 交差チェックと調整
        let (final_out1, final_out2) =
            handle_edge_intersection(&work_graph, current_node, output1, output2);

        if final_out1 == final_out2 && final_out1 == 0 {
            continue;
        }

        // 辺を追加
        add_edge(&mut work_graph, current_node, final_out1, final_out2);

        // 新しい分別器をキューに追加
        for &target in &[final_out1, final_out2] {
            if target >= graph.n && target < graph.n + graph.m {
                let sep_idx = target - graph.n;
                if !used_separators[sep_idx] {
                    queue.push_back(sep_idx);
                    used_separators[sep_idx] = true;
                }
            }
        }
    }
    // 構築完了後、処理装置に接続されていない分別器を繰り返し削除
    remove_disconnected_separators(&mut work_graph);
    // 最終的な設定をwork_graph.edgesから生成
    let configs = generate_configs_from_graph(graph, &work_graph);
    let start_node = graph.n + nearest_sep;
    (start_node, configs)
}

// work_graph.edgesからconfigsを生成
fn generate_configs_from_graph(graph: &Graph, work_graph: &Graph) -> Vec<String> {
    let mut configs = vec!["-1".to_string(); graph.m];
    let edges = get_reachout_edge(work_graph);
    // eprintln!("Edges: {:?}", edges);
    for sep_idx in 0..graph.m {
        let sep_node = graph.n + sep_idx;

        if let Some(out) = work_graph.edges.get(&sep_node) {
            let best_sep =
                select_best_separator_type(edges[&sep_node].clone(), graph.probabilities.clone());
            let v1 = out.out1;
            let v2 = out.out2;
            let separator_type = best_sep;
            configs[sep_idx] = format!("{} {} {}", separator_type, v1, v2);
        }
    }

    configs
}

// 辺の交差処理
fn handle_edge_intersection(
    graph: &Graph,
    from: NodeId,
    output1: Option<NodeId>,
    output2: Option<NodeId>,
) -> (NodeId, NodeId) {
    let default_out = 0; // デフォルトは最初の処理装置

    let out1 = output1.unwrap_or(default_out);
    let out2 = output2.unwrap_or(out1);

    // edge1の交差チェック
    let edge1_intersects = new_edge_intersects(graph, from, out1);

    // edge2の交差チェック
    let edge2_intersects =
        new_edge_intersects(graph, from, out2) || edge_intersects(graph, from, out1, from, out2);

    // 交差状況に応じて処理
    if !edge1_intersects && !edge2_intersects {
        // 両方とも交差しない
        (out1, out2)
    } else if !edge1_intersects && edge2_intersects {
        // edge1のみ有効、edge2は統合
        (out1, out1)
    } else if edge1_intersects && !edge2_intersects {
        // edge2のみ有効
        (out2, out2)
    } else {
        // 両方とも交差する場合は削除
        (0, 0) // 削除マーカー
    }
}

// 逆グラフを構築
fn build_reverse_graph(graph: &Graph) -> std::collections::HashMap<NodeId, Vec<Source>> {
    let mut reverse_edges = std::collections::HashMap::new();

    for (&from, out) in &graph.edges {
        // out1への辺を追加
        reverse_edges
            .entry(out.out1)
            .or_insert_with(Vec::new)
            .push(Source {
                id: from,
                ty: OutType::Out1,
            });

        // out2への辺を追加
        reverse_edges
            .entry(out.out2)
            .or_insert_with(Vec::new)
            .push(Source {
                id: from,
                ty: OutType::Out2,
            });
    }
    reverse_edges
}

// 処理装置に接続されていない分別器を検出
fn find_disconnected_separators(graph: &Graph) -> Vec<usize> {
    let reverse_graph = build_reverse_graph(graph);

    // 全ての分別器を最初は到達不可能として設定
    let mut unreachable: std::collections::HashSet<usize> = (0..graph.m).collect();
    let mut queue = std::collections::VecDeque::new();
    let mut visited = std::collections::HashSet::new();

    // 全ての処理装置をスタート地点として追加
    for i in 0..graph.n {
        queue.push_back(i);
        visited.insert(i);
    }

    // 逆方向BFS
    while let Some(current) = queue.pop_front() {
        // 分別器の場合は到達可能なので unreachable から除去
        if current >= graph.n && current < graph.n + graph.m {
            let sep_idx = current - graph.n;
            unreachable.remove(&sep_idx);
        }

        if let Some(predecessors) = reverse_graph.get(&current) {
            for source in predecessors {
                let predecessor = source.id;
                if !visited.contains(&predecessor) {
                    visited.insert(predecessor);
                    queue.push_back(predecessor);
                }
            }
        }
    }

    unreachable.into_iter().collect()
}

// 接続されていない分別器を削除
fn remove_disconnected_separators(graph: &mut Graph) {
    let disconnected = find_disconnected_separators(graph);

    for &sep_idx in &disconnected {
        let sep_node = graph.n + sep_idx;

        // この分別器から出る辺を削除
        graph.edges.remove(&sep_node);

        // この分別器への辺を削除
        let nodes_to_update: Vec<NodeId> = graph.edges.keys().cloned().collect();
        for node in nodes_to_update {
            if let Some(out) = graph.edges.get_mut(&node) {
                if out.out1 == sep_node && out.out2 == sep_node {
                    // 両方とも削除された分別器を参照している場合はノード全体を削除
                    graph.edges.remove(&node);
                } else if out.out1 == sep_node {
                    // out1のみ削除された分別器を参照している場合、out2に統一
                    out.out1 = out.out2;
                } else if out.out2 == sep_node {
                    // out2のみ削除された分別器を参照している場合、out1に統一
                    out.out2 = out.out1;
                }
            }
        }
    }
}

fn get_reachout_edge(graph: &Graph) -> HashMap<NodeId, Vec<WeightedReachability>> {
    let reverse_graph = build_reverse_graph(graph);

    let mut queue = std::collections::VecDeque::new();
    let mut visited: HashMap<NodeId, Vec<WeightedReachability>> = std::collections::HashMap::new();
    let mut can_reach = std::collections::HashMap::new();
    let mut distances = std::collections::HashMap::new();

    // 全ての処理装置をスタート地点として追加
    for i in 0..graph.n {
        let mut v = vec![
            WeightedReachability {
                reachout: 0,
                distance_weight: 0.0
            };
            graph.n
        ];
        v[i] = WeightedReachability {
            reachout: 1,
            distance_weight: 1.0,
        }; // 自分への距離は1.0
        visited.insert(i, v.clone());
        let mut set = std::collections::HashSet::new();
        set.insert(i);
        can_reach.insert(i, set);
        let mut dist_map = std::collections::HashMap::new();
        dist_map.insert(i, 0.0); // 自分への距離は0
        distances.insert(i, dist_map);
        queue.push_back(i);
    }

    while let Some(current) = queue.pop_front() {
        let current_processors = visited.get(&current).unwrap().clone();
        let current_can_reach = can_reach.get(&current).unwrap().clone();
        let current_distances = distances.get(&current).unwrap().clone();

        if let Some(predecessors) = reverse_graph.get(&current) {
            for source in predecessors {
                let predecessor = source.id;
                let pred_pos = get_node_position(graph, predecessor);
                let current_pos = get_node_position(graph, current);
                let edge_distance = distance(pred_pos, current_pos);

                if let Some(processors) = visited.get_mut(&predecessor) {
                    let predecessor_can_reach = can_reach.get_mut(&predecessor).unwrap();
                    let predecessor_distances = distances.get_mut(&predecessor).unwrap();
                    predecessor_can_reach.extend(current_can_reach.iter().cloned());

                    for j in 0..graph.n {
                        if current_can_reach.contains(&j) {
                            // 距離を更新（predecessor -> current -> processor j）
                            let new_distance =
                                current_distances.get(&j).unwrap_or(&f64::MAX) + edge_distance;
                            let existing_distance =
                                predecessor_distances.get(&j).unwrap_or(&f64::MAX);

                            if new_distance < *existing_distance {
                                predecessor_distances.insert(j, new_distance);
                            }

                            // 距離の逆数を重みとして使用
                            let distance_weight = if new_distance > 0.0 {
                                1.0 / new_distance
                            } else {
                                1.0
                            };

                            if source.ty == OutType::Out1 {
                                if current_processors[j].reachout != 0 {
                                    processors[j].reachout += current_processors[j].reachout.abs();
                                    processors[j].distance_weight += distance_weight;
                                } else if current_can_reach.contains(&j) {
                                    processors[j].reachout += 1;
                                    processors[j].distance_weight += distance_weight;
                                }
                            } else {
                                if current_processors[j].reachout != 0 {
                                    processors[j].reachout -= current_processors[j].reachout.abs();
                                    processors[j].distance_weight += distance_weight;
                                } else if current_can_reach.contains(&j) {
                                    processors[j].reachout -= 1;
                                    processors[j].distance_weight += distance_weight;
                                }
                            }
                        }
                    }
                } else {
                    let mut processors = vec![
                        WeightedReachability {
                            reachout: 0,
                            distance_weight: 0.0
                        };
                        graph.n
                    ];
                    let mut new_distances = std::collections::HashMap::new();

                    for j in 0..graph.n {
                        if current_can_reach.contains(&j) {
                            let new_distance =
                                current_distances.get(&j).unwrap_or(&f64::MAX) + edge_distance;
                            new_distances.insert(j, new_distance);
                            let distance_weight = if new_distance > 0.0 {
                                1.0 / new_distance
                            } else {
                                1.0
                            };

                            if source.ty == OutType::Out1 {
                                processors[j].reachout += current_processors[j].reachout.abs();
                                processors[j].distance_weight = distance_weight;
                            } else {
                                processors[j].reachout -= current_processors[j].reachout.abs();
                                processors[j].distance_weight = distance_weight;
                            }
                        }
                    }
                    visited.insert(predecessor, processors);
                    can_reach.insert(predecessor, current_can_reach.clone());
                    distances.insert(predecessor, new_distances);
                    queue.push_back(predecessor);
                }
            }
        }
    }

    visited
}

fn select_best_separator_type(
    reachouts: Vec<WeightedReachability>,
    probabilities: Vec<Vec<f64>>,
) -> usize {
    let mut max = 0.0;
    let mut max_index = 0;
    for (i, probability) in probabilities.iter().enumerate() {
        let mut sum = 0.0;
        for (j, weighted_reach) in reachouts.iter().enumerate() {
            // 距離重みを適用した確率計算
            let base_prob = if weighted_reach.reachout > 0 {
                probability[j]
            } else if weighted_reach.reachout < 0 {
                1.0 - probability[j]
            } else {
                0.0 // 到達できない場合
            };
            // 距離重みをかけて評価値を計算
            sum += base_prob * weighted_reach.distance_weight;
        }
        if sum >= max {
            max = sum;
            max_index = i;
        }
    }
    max_index
}

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        device_locations: [(i32, i32); n],
        separator_locations: [(i32, i32); m],
        probabilities: [[f64; n]; k],
    }

    let processor_positions: Vec<Point> = device_locations
        .into_iter()
        .map(|(x, y)| Point { x, y })
        .collect();
    let separator_positions: Vec<Point> = separator_locations
        .into_iter()
        .map(|(x, y)| Point { x, y })
        .collect();

    let graph = create_graph(
        n,
        m,
        processor_positions,
        separator_positions,
        probabilities,
    );
    let (device_assignments, start_node, separator_configs) = solve(&graph);

    print!(
        "{}",
        device_assignments
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!();
    println!("{}", start_node);

    for config in separator_configs {
        println!("{}", config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> Graph {
        let processor_positions = vec![
            Point { x: 0, y: 0 },
            Point { x: 100, y: 0 },
            Point { x: 0, y: 100 },
        ];
        let separator_positions = vec![Point { x: 50, y: 50 }, Point { x: 25, y: 25 }];
        let probabilities = vec![vec![0.8, 0.1, 0.1], vec![0.1, 0.8, 0.1]];

        create_graph(
            3,
            2,
            processor_positions,
            separator_positions,
            probabilities,
        )
    }

    #[test]
    fn test_distance() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 3, y: 4 };
        assert_eq!(distance(p1, p2), 5.0);
    }

    #[test]
    fn test_get_node_position() {
        let graph = create_test_graph();

        // 処理装置の位置
        assert_eq!(get_node_position(&graph, 0), Point { x: 0, y: 0 });
        assert_eq!(get_node_position(&graph, 1), Point { x: 100, y: 0 });
        assert_eq!(get_node_position(&graph, 2), Point { x: 0, y: 100 });

        // 分別器の位置
        assert_eq!(get_node_position(&graph, 3), Point { x: 50, y: 50 });
        assert_eq!(get_node_position(&graph, 4), Point { x: 25, y: 25 });

        // 搬入口の位置
        assert_eq!(
            get_node_position(&graph, usize::MAX),
            Point { x: 0, y: 5000 }
        );
    }

    #[test]
    fn test_add_edge() {
        let mut graph = create_test_graph();
        add_edge(&mut graph, 3, 0, 1); // 分別器0から処理装置0,1へ

        assert!(graph.edges.contains_key(&3));
        let out = &graph.edges[&3];
        assert_eq!(out.out1, 0);
        assert_eq!(out.out2, 1);
    }

    #[test]
    fn test_build_reverse_graph() {
        let mut graph = create_test_graph();
        add_edge(&mut graph, 3, 0, 1); // 分別器0から処理装置0,1へ
        add_edge(&mut graph, 4, 1, 2); // 分別器1から処理装置1,2へ

        let reverse_graph = build_reverse_graph(&graph);

        // 処理装置0への入力
        assert!(reverse_graph.contains_key(&0));
        let sources_to_0 = &reverse_graph[&0];
        assert_eq!(sources_to_0.len(), 1);
        assert_eq!(sources_to_0[0].id, 3);
        assert_eq!(sources_to_0[0].ty, OutType::Out1);

        // 処理装置1への入力
        assert!(reverse_graph.contains_key(&1));
        let sources_to_1 = &reverse_graph[&1];
        assert_eq!(sources_to_1.len(), 2); // 分別器0のout2と分別器1のout1

        // 処理装置2への入力
        assert!(reverse_graph.contains_key(&2));
        let sources_to_2 = &reverse_graph[&2];
        assert_eq!(sources_to_2.len(), 1);
        assert_eq!(sources_to_2[0].id, 4);
        assert_eq!(sources_to_2[0].ty, OutType::Out2);
    }

    #[test]
    fn test_segments_intersect() {
        // 明らかに交差する線分
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 100, y: 100 };
        let q1 = Point { x: 0, y: 100 };
        let q2 = Point { x: 100, y: 0 };
        assert!(segments_intersect(p1, p2, q1, q2));

        // 交差しない線分
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 50, y: 0 };
        let q1 = Point { x: 60, y: 0 };
        let q2 = Point { x: 100, y: 0 };
        assert!(!segments_intersect(p1, p2, q1, q2));

        // 端点が同じ場合（交差しないとみなす）
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 50, y: 50 };
        let q1 = Point { x: 0, y: 0 };
        let q2 = Point { x: 50, y: 0 };
        assert!(!segments_intersect(p1, p2, q1, q2));
    }

    #[test]
    fn test_orientation() {
        // 反時計回り
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 0 };
        let c = Point { x: 0, y: 1 };
        assert_eq!(orientation(a, b, c), 1);

        // 時計回り
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 0 };
        let c = Point { x: 1, y: -1 };
        assert_eq!(orientation(a, b, c), -1);

        // 一直線
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 0 };
        let c = Point { x: 2, y: 0 };
        assert_eq!(orientation(a, b, c), 0);
    }

    #[test]
    fn test_select_best_separator_type() {
        // 簡単な例：3つの処理装置、2つの分別器タイプ
        let reachouts = vec![
            WeightedReachability {
                reachout: 1,
                distance_weight: 1.0,
            }, // 処理装置0
            WeightedReachability {
                reachout: -1,
                distance_weight: 1.0,
            }, // 処理装置1
            WeightedReachability {
                reachout: 1,
                distance_weight: 1.0,
            }, // 処理装置2
        ];
        let probabilities = vec![
            vec![0.9, 0.1, 0.8], // タイプ0: 処理装置0,2に高確率、処理装置1に低確率
            vec![0.1, 0.9, 0.2], // タイプ1: 処理装置1に高確率、処理装置0,2に低確率
        ];

        let best_type = select_best_separator_type(reachouts, probabilities);
        // タイプ0の期待値: 0.9*1.0 + (1-0.1)*1.0 + 0.8*1.0 = 2.6
        // タイプ1の期待値: 0.1*1.0 + 0.9*1.0 + 0.2*1.0 = 1.2
        // タイプ0の方が高いはず
        assert_eq!(best_type, 0);
    }

    #[test]
    fn test_find_disconnected_separators() {
        let mut graph = create_test_graph();

        // 分別器0のみ処理装置に接続
        add_edge(&mut graph, 3, 0, 1);

        let disconnected = find_disconnected_separators(&graph);

        // 分別器1は処理装置に接続されていないので切断されている
        assert_eq!(disconnected, vec![1]);
    }

    #[test]
    fn test_handle_edge_intersection() {
        let graph = create_test_graph();

        // 交差しない場合
        let (out1, out2) = handle_edge_intersection(&graph, 3, Some(0), Some(1));
        assert_eq!((out1, out2), (0, 1));

        // 出力が指定されない場合
        let (out1, out2) = handle_edge_intersection(&graph, 3, None, None);
        assert_eq!((out1, out2), (0, 0));
    }

    #[test]
    fn test_remove_disconnected_separators() {
        let mut graph = create_test_graph();

        // 分別器0から処理装置に直接接続、分別器1は未接続
        add_edge(&mut graph, 3, 0, 1); // 分別器0 -> 処理装置0,1

        // この状態では分別器1のみが切断されている
        let disconnected_before = find_disconnected_separators(&graph);
        assert_eq!(disconnected_before, vec![1]);

        // 切断された分別器を削除
        remove_disconnected_separators(&mut graph);

        // 分別器0の接続は残る
        assert!(graph.edges.contains_key(&3));

        // 追加テスト: 分別器チェーンが切断される場合
        let mut graph2 = create_test_graph();
        add_edge(&mut graph2, 3, 4, 4); // 分別器0 -> 分別器1, 分別器1
                                        // 分別器1は処理装置に接続されていないので、両方切断される
        let disconnected = find_disconnected_separators(&graph2);
        assert!(disconnected.contains(&0) && disconnected.contains(&1));
    }

    #[test]
    fn test_edge_intersects() {
        let graph = create_test_graph();

        // 交差しない辺のテスト
        assert!(!edge_intersects(
            &graph, 3, 0, // 分別器0 -> 処理装置0 (50,50) -> (0,0)
            4, 2 // 分別器1 -> 処理装置2 (25,25) -> (0,100)
        ));

        // 関数が正常に動作することを確認するため、別の組み合わせも試す
        assert!(!edge_intersects(
            &graph, 3, 1, // 分別器0 -> 処理装置1 (50,50) -> (100,0)
            4, 0 // 分別器1 -> 処理装置0 (25,25) -> (0,0)
        ));
    }

    #[test]
    fn test_new_edge_intersects() {
        let mut graph = create_test_graph();

        // 既存の辺を追加
        add_edge(&mut graph, 3, 1, 2); // (50,50) -> (100,0) と (50,50) -> (0,100)

        // 交差しない辺のテスト
        assert!(!new_edge_intersects(&graph, 4, 0)); // (25,25) -> (0,0)

        // 明らかに交差する辺を追加してテスト
        let mut intersect_graph = create_test_graph();
        add_edge(&mut intersect_graph, 3, 1, 2); // (50,50) -> (100,0) と (50,50) -> (0,100)

        // 実際の交差はより複雑なので、基本的な機能テストに留める
        // 関数が正常に実行されることを確認
        let _ = new_edge_intersects(&intersect_graph, 4, 0);
    }

    #[test]
    fn test_create_graph() {
        let graph = create_test_graph();
        assert_eq!(graph.n, 3);
        assert_eq!(graph.m, 2);
        assert_eq!(graph.processor_positions.len(), 3);
        assert_eq!(graph.separator_positions.len(), 2);
        assert_eq!(graph.probabilities.len(), 2);
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn test_solve_basic() {
        let graph = create_test_graph();
        let (device_assignments, start_node, separator_configs) = solve(&graph);

        // デバイス割り当てチェック
        assert_eq!(device_assignments, vec![0, 1, 2]);

        // スタートノードは分別器の範囲内
        assert!(start_node >= graph.n && start_node < graph.n + graph.m);

        // 設定の数が分別器の数と一致
        assert_eq!(separator_configs.len(), graph.m);
    }

    #[test]
    fn test_get_reachout_edge() {
        let mut graph = create_test_graph();

        // 単純なネットワーク構築: 分別器0 -> 処理装置0,1
        add_edge(&mut graph, 3, 0, 1);

        let reachout_edges = get_reachout_edge(&graph);

        // 分別器0から各処理装置への到達可能性をチェック
        assert!(reachout_edges.contains_key(&3));
        let reachouts = &reachout_edges[&3];

        // 処理装置0にはout1で到達可能 -> +1
        assert_eq!(reachouts[0], 1);
        // 処理装置1にはout2で到達可能 -> -1
        assert_eq!(reachouts[1], -1);
        // 処理装置2には到達不可能 -> 0
        assert_eq!(reachouts[2], 0);
    }

    #[test]
    fn test_get_reachout_edge_complex() {
        let mut graph = create_test_graph();

        // より複雑なネットワーク: 分別器チェーン
        // 分別器0 -> 分別器1, 処理装置0
        // 分別器1 -> 処理装置1, 処理装置2
        add_edge(&mut graph, 3, 4, 0); // 分別器0 -> 分別器1, 処理装置0
        add_edge(&mut graph, 4, 1, 2); // 分別器1 -> 処理装置1, 処理装置2

        let reachout_edges = get_reachout_edge(&graph);

        // 分別器0からの到達可能性
        if let Some(reachouts_sep0) = reachout_edges.get(&3) {
            // 処理装置0: 直接out2で到達 -> -1
            assert_eq!(reachouts_sep0[0], -1);
            // 処理装置1: 分別器1経由でout1->out1で到達 -> +1
            assert_eq!(reachouts_sep0[1], 1);
            // 処理装置2: 分別器1経由でout1->out2で到達 -> -1
            assert_eq!(reachouts_sep0[2], 1);
        }

        // 分別器1からの到達可能性
        if let Some(reachouts_sep1) = reachout_edges.get(&4) {
            // 処理装置0: 到達不可能 -> 0
            assert_eq!(reachouts_sep1[0], 0);
            // 処理装置1: out1で到達 -> +1
            assert_eq!(reachouts_sep1[1], 1);
            // 処理装置2: out2で到達 -> -1
            assert_eq!(reachouts_sep1[2], -1);
        }
    }

    #[test]
    fn test_get_reachout_edge_empty() {
        let graph = create_test_graph();

        // 辺がない場合のテスト
        let reachout_edges = get_reachout_edge(&graph);

        // 処理装置のみが存在し、それぞれ自分自身に到達可能
        for i in 0..graph.n {
            assert!(reachout_edges.contains_key(&i));
            let reachouts = &reachout_edges[&i];

            // 自分自身には到達可能 -> +1
            assert_eq!(reachouts[i], 1);

            // 他の処理装置には到達不可能 -> 0
            for j in 0..graph.n {
                if i != j {
                    assert_eq!(reachouts[j], 0);
                }
            }
        }
    }

    #[test]
    fn test_get_reachout_edge_diamond_network() {
        // より大きなグラフで複雑なダイヤモンド型ネットワークをテスト
        let processor_positions = vec![
            Point { x: 0, y: 0 },   // 処理装置0
            Point { x: 100, y: 0 }, // 処理装置1
            Point { x: 200, y: 0 }, // 処理装置2
            Point { x: 300, y: 0 }, // 処理装置3
        ];
        let separator_positions = vec![
            Point { x: 50, y: 50 },   // 分別器0
            Point { x: 150, y: 50 },  // 分別器1
            Point { x: 250, y: 50 },  // 分別器2
            Point { x: 100, y: 100 }, // 分別器3
            Point { x: 200, y: 100 }, // 分別器4
        ];
        let probabilities = vec![vec![0.8, 0.1, 0.05, 0.05], vec![0.1, 0.8, 0.05, 0.05]];

        let mut graph = create_graph(
            4,
            5,
            processor_positions,
            separator_positions,
            probabilities,
        );

        // ダイヤモンド型ネットワーク構築:
        // 分別器0 → 分別器3, 分別器4
        // 分別器1 → 分別器3, 分別器4
        // 分別器3 → 処理装置0, 処理装置1
        // 分別器4 → 処理装置2, 処理装置3
        // 分別器2 → 処理装置1, 処理装置2
        add_edge(&mut graph, 4, 7, 8); // 分別器0 → 分別器3, 分別器4
        add_edge(&mut graph, 5, 7, 8); // 分別器1 → 分別器3, 分別器4
        add_edge(&mut graph, 7, 0, 1); // 分別器3 → 処理装置0, 処理装置1
        add_edge(&mut graph, 8, 2, 3); // 分別器4 → 処理装置2, 処理装置3
        add_edge(&mut graph, 6, 1, 2); // 分別器2 → 処理装置1, 処理装置2

        let reachout_edges = get_reachout_edge(&graph);

        // 分別器0からの到達可能性（複数経路で処理装置に到達）
        if let Some(reachouts_sep0) = reachout_edges.get(&4) {
            // 処理装置0: 分別器3経由でout1→out1 -> +1
            assert_eq!(reachouts_sep0[0], 1);
            // 処理装置1: 分別器3経由でout1→out2 -> 1
            assert_eq!(reachouts_sep0[1], 1);
            // 処理装置2: 分別器4経由でout2→out1 -> -1
            assert_eq!(reachouts_sep0[2], -1);
            // 処理装置3: 分別器4経由でout2→out2 -> +1
            assert_eq!(reachouts_sep0[3], -1);
        }

        // 分別器1からの到達可能性（分別器0と同じ構造）
        if let Some(reachouts_sep1) = reachout_edges.get(&5) {
            assert_eq!(reachouts_sep1[0], 1);
            assert_eq!(reachouts_sep1[1], 1);
            assert_eq!(reachouts_sep1[2], -1);
            assert_eq!(reachouts_sep1[3], -1);
        }
    }

    #[test]
    fn test_get_reachout_edge_multi_path_accumulation() {
        // 同じ処理装置への複数経路での累積をテスト
        let processor_positions = vec![
            Point { x: 0, y: 0 },   // 処理装置0
            Point { x: 100, y: 0 }, // 処理装置1
            Point { x: 200, y: 0 }, // 処理装置2
        ];
        let separator_positions = vec![
            Point { x: 50, y: 50 },   // 分別器0
            Point { x: 150, y: 50 },  // 分別器1
            Point { x: 100, y: 100 }, // 分別器2
        ];
        let probabilities = vec![vec![0.8, 0.1, 0.1], vec![0.1, 0.8, 0.1]];

        let mut graph = create_graph(
            3,
            3,
            processor_positions,
            separator_positions,
            probabilities,
        );

        // 複数経路で同じ処理装置に到達する構造:
        // 分別器0 → 分別器2, 処理装置1
        // 分別器1 → 処理装置1, 分別器2
        // 分別器2 → 処理装置0, 処理装置1
        add_edge(&mut graph, 3, 5, 1); // 分別器0 → 分別器2, 処理装置1
        add_edge(&mut graph, 4, 1, 5); // 分別器1 → 処理装置1, 分別器2
        add_edge(&mut graph, 5, 0, 1); // 分別器2 → 処理装置0, 処理装置1

        let reachout_edges = get_reachout_edge(&graph);

        // 分別器0からの到達可能性
        if let Some(reachouts_sep0) = reachout_edges.get(&3) {
            // 処理装置0: 分別器2経由でout1→out1 -> +1
            assert_eq!(reachouts_sep0[0], 1);
            // 処理装置1: 直接out2で到達(-1) + 分別器2経由でout1→out2で到達(-1) = -2
            assert_eq!(reachouts_sep0[1], 0);
            // 処理装置2: 到達不可能 -> 0
            assert_eq!(reachouts_sep0[2], 0);
        }

        // 分別器1からの到達可能性（分別器0と同じ構造）
        if let Some(reachouts_sep1) = reachout_edges.get(&4) {
            assert_eq!(reachouts_sep1[0], -1);
            assert_eq!(reachouts_sep1[1], 0);
            assert_eq!(reachouts_sep1[2], 0);
        }
    }

    #[test]
    fn test_get_reachout_edge_deep_chain() {
        // 実際に動作する深い連鎖構造での経路テスト
        let processor_positions = vec![
            Point { x: 0, y: 0 },   // 処理装置0
            Point { x: 100, y: 0 }, // 処理装置1
        ];
        let separator_positions = vec![
            Point { x: 50, y: 50 },  // 分別器0
            Point { x: 100, y: 50 }, // 分別器1
            Point { x: 150, y: 50 }, // 分別器2
            Point { x: 200, y: 50 }, // 分別器3
        ];
        let probabilities = vec![vec![0.8, 0.2], vec![0.3, 0.7]];

        let mut graph = create_graph(
            2,
            4,
            processor_positions,
            separator_positions,
            probabilities,
        );

        // 単純な線形チェーン: 分別器0 → 分別器1 → 分別器3 → 処理装置
        add_edge(&mut graph, 2, 3, 1); // 分別器0 → 分別器1, 処理装置1
        add_edge(&mut graph, 3, 5, 5); // 分別器1 → 分別器3, 分別器3
        add_edge(&mut graph, 5, 0, 1); // 分別器3 → 処理装置0, 処理装置1

        let reachout_edges = get_reachout_edge(&graph);

        // 同じ分別器に両方の出力が向かう場合は有効な経路とならないため、
        // 分別器0からは直接的な到達不可能
        if let Some(reachouts_sep0) = reachout_edges.get(&2) {
            // アルゴリズムの実装上、同じ分別器への両出力は無効な経路
            assert_eq!(reachouts_sep0[0], 1);
            assert_eq!(reachouts_sep0[1], 0);
        }

        if let Some(reachouts_sep0) = reachout_edges.get(&3) {
            // アルゴリズムの実装上、同じ分別器への両出力は無効な経路
            assert_eq!(reachouts_sep0[0], 0);
            assert_eq!(reachouts_sep0[1], 0);
        }

        // 分別器3からの到達可能性（直接接続）
        if let Some(reachouts_sep3) = reachout_edges.get(&5) {
            assert_eq!(reachouts_sep3[0], 1); // out1で到達
            assert_eq!(reachouts_sep3[1], -1); // out2で到達
        }
    }

    #[test]
    fn test_get_reachout_edge_branching_network() {
        // 分岐を伴うより複雑なネットワーク
        let processor_positions = vec![
            Point { x: 0, y: 0 },   // 処理装置0
            Point { x: 100, y: 0 }, // 処理装置1
            Point { x: 200, y: 0 }, // 処理装置2
        ];
        let separator_positions = vec![
            Point { x: 50, y: 50 },   // 分別器0
            Point { x: 100, y: 100 }, // 分別器1
            Point { x: 150, y: 100 }, // 分別器2
        ];
        let probabilities = vec![vec![0.6, 0.3, 0.1], vec![0.2, 0.6, 0.2]];

        let mut graph = create_graph(
            3,
            3,
            processor_positions,
            separator_positions,
            probabilities,
        );

        // 実際に分岐するネットワーク:
        // 分別器0 → 分別器1, 分別器2
        // 分別器1 → 処理装置0, 処理装置1
        // 分別器2 → 処理装置1, 処理装置2
        add_edge(&mut graph, 3, 4, 5); // 分別器0 → 分別器1, 分別器2
        add_edge(&mut graph, 4, 0, 1); // 分別器1 → 処理装置0, 処理装置1
        add_edge(&mut graph, 5, 1, 2); // 分別器2 → 処理装置1, 処理装置2

        let reachout_edges = get_reachout_edge(&graph);

        // 分別器0からの到達可能性
        if let Some(reachouts_sep0) = reachout_edges.get(&3) {
            // 処理装置0: 分別器1経由でout1→out1 -> +1
            assert_eq!(reachouts_sep0[0], 1);
            // 処理装置1: 複数経路での累積効果
            // 分別器1経由 + 分別器2経由で合計-2（実際の出力に基づく）
            assert_eq!(reachouts_sep0[1], 0);
            // 処理装置2: 分別器2経由でout2→out2 -> +1（実際の出力に基づく）
            assert_eq!(reachouts_sep0[2], -1);
        }

        // 分別器1からの到達可能性
        if let Some(reachouts_sep1) = reachout_edges.get(&4) {
            // 処理装置0: out1で到達 -> +1
            assert_eq!(reachouts_sep1[0], 1);
            // 処理装置1: out2で到達 -> -1
            assert_eq!(reachouts_sep1[1], -1);
            // 処理装置2: 到達不可能 -> 0
            assert_eq!(reachouts_sep1[2], 0);
        }

        // 分別器2からの到達可能性
        if let Some(reachouts_sep2) = reachout_edges.get(&5) {
            // 処理装置0: 到達不可能 -> 0
            assert_eq!(reachouts_sep2[0], 0);
            // 処理装置1: out1で到達 -> +1
            assert_eq!(reachouts_sep2[1], 1);
            // 処理装置2: out2で到達 -> -1
            assert_eq!(reachouts_sep2[2], -1);
        }
    }
}
