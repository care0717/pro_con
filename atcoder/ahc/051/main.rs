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

#[derive(Clone, Debug)]
struct Segment {
    start: Point,
    end: Point,
}

// ノードの基底トレイト
trait Node {
    fn get_id(&self) -> &str;
    fn get_position(&self) -> Point;
    fn get_next_nodes(&self, waste_type: usize) -> Vec<(String, f64)>;
}

// 搬入口ノード
#[derive(Clone)]
struct EntranceNode {
    id: String,
    position: Point,
    target: Option<String>,
}

impl EntranceNode {
    fn new(position: Point) -> Self {
        Self {
            id: "entrance".to_string(),
            position,
            target: None,
        }
    }
    
    fn set_target(&mut self, target_id: String) {
        self.target = Some(target_id);
    }
}

impl Node for EntranceNode {
    fn get_id(&self) -> &str {
        &self.id
    }
    
    fn get_position(&self) -> Point {
        self.position
    }
    
    fn get_next_nodes(&self, _waste_type: usize) -> Vec<(String, f64)> {
        if let Some(ref target) = self.target {
            vec![(target.clone(), 1.0)]
        } else {
            vec![]
        }
    }
}

// 分別器ノード
#[derive(Clone)]
struct SeparatorNode {
    id: String,
    position: Point,
    separator_type: usize,
    probabilities: Vec<f64>, // 各ごみ種類の出口1確率
    exit1_target: Option<String>,
    exit2_target: Option<String>,
}

impl SeparatorNode {
    fn new(id: String, position: Point, separator_type: usize, probabilities: Vec<f64>) -> Self {
        Self {
            id,
            position,
            separator_type,
            probabilities,
            exit1_target: None,
            exit2_target: None,
        }
    }
    
    fn set_targets(&mut self, exit1_target: String, exit2_target: String) {
        self.exit1_target = Some(exit1_target);
        self.exit2_target = Some(exit2_target);
    }
}

impl Node for SeparatorNode {
    fn get_id(&self) -> &str {
        &self.id
    }
    
    fn get_position(&self) -> Point {
        self.position
    }
    
    fn get_next_nodes(&self, waste_type: usize) -> Vec<(String, f64)> {
        let mut result = Vec::new();
        
        if waste_type < self.probabilities.len() {
            let prob_exit1 = self.probabilities[waste_type];
            let prob_exit2 = 1.0 - prob_exit1;
            
            if let Some(ref exit1) = self.exit1_target {
                if prob_exit1 > 0.0 {
                    result.push((exit1.clone(), prob_exit1));
                }
            }
            
            if let Some(ref exit2) = self.exit2_target {
                if prob_exit2 > 0.0 {
                    result.push((exit2.clone(), prob_exit2));
                }
            }
        }
        
        result
    }
}

// 処理装置ノード
#[derive(Clone)]
struct ProcessorNode {
    id: String,
    position: Point,
    waste_type: usize, // 処理するごみの種類
}

impl ProcessorNode {
    fn new(id: String, position: Point, waste_type: usize) -> Self {
        Self {
            id,
            position,
            waste_type,
        }
    }
    
    fn is_correct_processor(&self, waste_type: usize) -> bool {
        self.waste_type == waste_type
    }
}

impl Node for ProcessorNode {
    fn get_id(&self) -> &str {
        &self.id
    }
    
    fn get_position(&self) -> Point {
        self.position
    }
    
    fn get_next_nodes(&self, _waste_type: usize) -> Vec<(String, f64)> {
        // 処理装置は終端ノード
        vec![]
    }
}

// 廃棄物処理システム
struct WasteProcessingSystem {
    n: usize, // ごみ種類数
    m: usize, // 分別器設置場所数
    k: usize, // 分別器種類数
    
    // ノード管理
    entrance: Option<EntranceNode>,
    separators: HashMap<String, SeparatorNode>,
    processors: HashMap<String, ProcessorNode>,
    
    // 位置情報
    processor_positions: Vec<Point>,
    separator_positions: Vec<Point>,
    probabilities: Vec<Vec<f64>>,
}

impl WasteProcessingSystem {
    fn new(n: usize, m: usize, k: usize) -> Self {
        Self {
            n,
            m,
            k,
            entrance: None,
            separators: HashMap::new(),
            processors: HashMap::new(),
            processor_positions: Vec::new(),
            separator_positions: Vec::new(),
            probabilities: Vec::new(),
        }
    }
    
    // 線分交差判定
    fn segments_intersect(&self, p1: Point, p2: Point, q1: Point, q2: Point) -> bool {
        // 端点が同じ場合は交差していないとみなす
        if (p1.x == q1.x && p1.y == q1.y) || (p1.x == q2.x && p1.y == q2.y) ||
           (p2.x == q1.x && p2.y == q1.y) || (p2.x == q2.x && p2.y == q2.y) {
            return false;
        }
        
        // バウンディングボックスの交差チェック
        if p1.x.max(p2.x) < q1.x.min(q2.x) ||
            q1.x.max(q2.x) < p1.x.min(p2.x) ||
            p1.y.max(p2.y) < q1.y.min(q2.y) ||
            q1.y.max(q2.y) < p1.y.min(p2.y) {
            return false;
        }
        
        let o1 = self.orientation(p1, p2, q1);
        let o2 = self.orientation(p1, p2, q2);
        let o3 = self.orientation(q1, q2, p1);
        let o4 = self.orientation(q1, q2, p2);
        
        // 厳密な交差のみを検出（端点での接触は除く）
        (o1 * o2 < 0) && (o3 * o4 < 0)
    }
    
    fn orientation(&self, a: Point, b: Point, c: Point) -> i32 {
        let cross = (b.x as i64 - a.x as i64) * (c.y as i64 - a.y as i64) - 
                    (b.y as i64 - a.y as i64) * (c.x as i64 - a.x as i64);
        self.sign(cross)
    }
    
    fn sign(&self, x: i64) -> i32 {
        if x > 0 {
            1
        } else if x < 0 {
            -1
        } else {
            0
        }
    }
    
    fn set_processor_positions(&mut self, positions: Vec<(i32, i32)>) {
        self.processor_positions = positions.into_iter()
            .map(|(x, y)| Point { x, y })
            .collect();
            
        // 処理装置ノードを作成
        for (i, pos) in self.processor_positions.iter().enumerate() {
            let id = format!("processor_{}", i);
            let processor = ProcessorNode::new(id.clone(), *pos, i);
            self.processors.insert(id, processor);
        }
    }
    
    fn set_separator_positions(&mut self, positions: Vec<(i32, i32)>) {
        self.separator_positions = positions.into_iter()
            .map(|(x, y)| Point { x, y })
            .collect();
    }
    
    fn set_probabilities(&mut self, probabilities: Vec<Vec<f64>>) {
        self.probabilities = probabilities;
    }
    
    fn set_entrance(&mut self, position: Point) {
        self.entrance = Some(EntranceNode::new(position));
    }
    
    fn add_separator(&mut self, id: String, position_idx: usize, separator_type: usize, 
                     exit1_target: String, exit2_target: String) {
        if position_idx < self.separator_positions.len() && separator_type < self.probabilities.len() {
            let position = self.separator_positions[position_idx];
            let probabilities = self.probabilities[separator_type].clone();
            
            let mut separator = SeparatorNode::new(id.clone(), position, separator_type, probabilities);
            separator.set_targets(exit1_target, exit2_target);
            
            self.separators.insert(id, separator);
        }
    }
    
    fn calculate_success_probability(&self, waste_type: usize) -> f64 {
        if let Some(ref entrance) = self.entrance {
            self.calculate_probability_recursive(entrance.get_id(), waste_type)
        } else {
            0.0
        }
    }
    
    fn calculate_probability_recursive(&self, node_id: &str, waste_type: usize) -> f64 {
        // 搬入口の場合
        if let Some(ref entrance) = self.entrance {
            if entrance.get_id() == node_id {
                let next_nodes = entrance.get_next_nodes(waste_type);
                let mut total_prob = 0.0;
                for (next_id, prob) in next_nodes {
                    total_prob += prob * self.calculate_probability_recursive(&next_id, waste_type);
                }
                return total_prob;
            }
        }
        
        // 分別器の場合
        if let Some(separator) = self.separators.get(node_id) {
            let next_nodes = separator.get_next_nodes(waste_type);
            let mut total_prob = 0.0;
            for (next_id, prob) in next_nodes {
                total_prob += prob * self.calculate_probability_recursive(&next_id, waste_type);
            }
            return total_prob;
        }
        
        // 処理装置の場合
        if let Some(processor) = self.processors.get(node_id) {
            return if processor.is_correct_processor(waste_type) { 1.0 } else { 0.0 };
        }
        
        0.0
    }
    
    fn solve(&mut self) -> (Vec<usize>, usize, Vec<String>) {
        // 搬入口を設定
        self.set_entrance(Point { x: 0, y: 5000 });
        
        // 距離ベースの貪欲アルゴリズムでネットワークを構築
        let (start_node, separator_configs) = self.build_network_greedy();
        
        // デバイス割り当ては単純に順番通り
        let device_assignments: Vec<usize> = (0..self.n).collect();
        
        (device_assignments, start_node, separator_configs)
    }
    
    fn build_network_greedy(&mut self) -> (usize, Vec<String>) {
        let mut configs = vec!["-1".to_string(); self.m];
        let mut segments = Vec::new();
        let mut used_separators = vec![false; self.m];
        let mut queue = std::collections::VecDeque::new();
        
        if self.separator_positions.is_empty() {
            return (0, configs);
        }
        
        let start_pos = Point { x: 0, y: 5000 };
        
        // スタート地点から最も近い分別器を見つける
        let mut min_dist = f64::MAX;
        let mut nearest_sep = 0;
        for i in 0..self.separator_positions.len() {
            let dist = self.distance(start_pos, self.separator_positions[i]);
            if dist < min_dist {
                min_dist = dist;
                nearest_sep = i;
            }
        }
        
        // 最初の分別器への線分を追加
        let first_sep_pos = self.separator_positions[nearest_sep];
        segments.push(Segment { start: start_pos, end: first_sep_pos });
        queue.push_back(nearest_sep);
        used_separators[nearest_sep] = true;
        
        // 搬入口の接続先を設定
        if let Some(ref mut entrance) = self.entrance {
            entrance.set_target(format!("separator_{}", nearest_sep));
        }
        
        // キューから分別器を処理（距離ベース貪欲法）
        while let Some(current_sep_idx) = queue.pop_front() {
            if current_sep_idx >= self.separator_positions.len() {
                continue;
            }
            
            let current_pos = self.separator_positions[current_sep_idx];
            let best_separator_type = self.find_best_separator_type();
            
            // 距離順に2つの出力先を選択
            let mut distances = Vec::new();
            
            // 全ての処理装置への距離
            for i in 0..self.processor_positions.len() {
                let device_pos = self.processor_positions[i];
                let dist = self.distance(current_pos, device_pos);
                distances.push((dist, i, true)); // true = device
            }
            
            // 未使用の分別器への距離
            for i in 0..self.separator_positions.len() {
                if !used_separators[i] {
                    let sep_pos = self.separator_positions[i];
                    let dist = self.distance(current_pos, sep_pos);
                    distances.push((dist, self.n + i, false)); // false = separator
                }
            }
            
            distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            
            let mut output1 = None;
            let mut output2 = None;
            let mut added_segments = Vec::new();
            
            // 距離順に2つの出力先を選択（交差チェック付き）
            for (_, target_id, is_device) in distances {
                let target_pos = if is_device {
                    self.processor_positions[target_id]
                } else {
                    self.separator_positions[target_id - self.n]
                };
                
                let new_segment = Segment { start: current_pos, end: target_pos };
                
                // 既存線分との交差チェック
                let mut has_intersection = false;
                let mut test_segments = segments.clone();
                test_segments.extend(added_segments.clone());
                test_segments.push(new_segment.clone());
                
                for i in 0..test_segments.len() {
                    for j in i + 1..test_segments.len() {
                        if self.segments_intersect(
                            test_segments[i].start, test_segments[i].end,
                            test_segments[j].start, test_segments[j].end
                        ) {
                            has_intersection = true;
                            break;
                        }
                    }
                    if has_intersection { break; }
                }
                
                if !has_intersection {
                    if output1.is_none() {
                        output1 = Some(target_id);
                        added_segments.push(new_segment);
                        if !is_device && target_id >= self.n {
                            let sep_idx = target_id - self.n;
                            if sep_idx < used_separators.len() {
                                queue.push_back(sep_idx);
                                used_separators[sep_idx] = true;
                            }
                        }
                    } else if output2.is_none() {
                        output2 = Some(target_id);
                        added_segments.push(new_segment);
                        if !is_device && target_id >= self.n {
                            let sep_idx = target_id - self.n;
                            if sep_idx < used_separators.len() {
                                queue.push_back(sep_idx);
                                used_separators[sep_idx] = true;
                            }
                        }
                        break;
                    }
                } else if output1.is_none() || output2.is_none() {
                    // 交差する場合：既存の出力先に統合（もう一本の方にまとめる）
                    if output1.is_some() {
                        output2 = output1; // 両方を同じ出力先に統合
                        break;
                    } else {
                        output1 = Some(target_id);
                        output2 = Some(target_id); // 両方を同じ出力先に統合
                        break;
                    }
                }
            }
            
            // 実際に線分を追加
            segments.extend(added_segments);
            
            // 設定を保存
            let out1 = output1.unwrap_or(0);
            let out2 = output2.unwrap_or(out1);
            
            // 分別器ノードを追加
            let sep_id = format!("separator_{}", current_sep_idx);
            let exit1_target = if out1 < self.n {
                format!("processor_{}", out1)
            } else {
                format!("separator_{}", out1 - self.n)
            };
            let exit2_target = if out2 < self.n {
                format!("processor_{}", out2)
            } else {
                format!("separator_{}", out2 - self.n)
            };
            
            self.add_separator(sep_id, current_sep_idx, best_separator_type, 
                             exit1_target, exit2_target);
            
            configs[current_sep_idx] = format!("{} {} {}", best_separator_type, out1, out2);
        }
        
        let start_node = self.n + nearest_sep;
        (start_node, configs)
    }
    
    fn find_best_separator_type(&self) -> usize {
        let mut best_idx = 0;
        let mut best_score = -1.0;
        
        for i in 0..self.probabilities.len() {
            let mut score = 0.0;
            for j in 0..self.probabilities[i].len() {
                let p = self.probabilities[i][j];
                score += (p - 0.5).abs();
            }
            if score > best_score {
                best_score = score;
                best_idx = i;
            }
        }
        
        best_idx
    }
    
    fn distance(&self, p1: Point, p2: Point) -> f64 {
        let dx = (p1.x - p2.x) as f64;
        let dy = (p1.y - p2.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        device_locations: [(i32, i32); n],
        separator_locations: [(i32, i32); m],
        probabilities: [[f64; n]; k],
    }
    
    let mut system = WasteProcessingSystem::new(n, m, k);
    system.set_processor_positions(device_locations);
    system.set_separator_positions(separator_locations);
    system.set_probabilities(probabilities);
    
    let (device_assignments, start_node, separator_configs) = system.solve();
    
    print!("{}", device_assignments.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    println!();
    println!("{}", start_node);
    
    for config in separator_configs {
        println!("{}", config);
    }
}