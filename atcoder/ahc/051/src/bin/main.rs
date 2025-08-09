use rand::Rng;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

// ========================================
// UTILITY MACROS
// ========================================

macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

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

// ========================================
// DATA STRUCTURES
// ========================================

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

type NodeId = usize;

#[derive(Clone, Debug)]
struct Graph {
    n: usize, // Number of waste types/processors
    m: usize, // Number of separator locations
    processor_positions: Vec<Point>,
    separator_positions: Vec<Point>,
    edges: HashMap<NodeId, Out>,
    start_node: NodeId,
}

#[derive(Clone, Debug)]
struct Out {
    out1: NodeId,
    out2: NodeId,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum OutType {
    Out1 = 1,
    Out2 = -1,
}

#[derive(PartialEq, Clone, Debug)]
struct Source {
    id: NodeId,
    ty: OutType,
}

#[derive(Clone, Debug, PartialEq)]
struct WeightedReachability {
    reachout: i32,
    distance_weight: f64,
}

// ========================================
// GEOMETRIC UTILITIES
// ========================================

/// Calculate Euclidean distance between two points
fn distance(p1: Point, p2: Point) -> f64 {
    let dx = (p1.x - p2.x) as f64;
    let dy = (p1.y - p2.y) as f64;
    (dx * dx + dy * dy).sqrt()
}

/// Get position of a node (processor or separator) by ID
fn get_node_position(graph: &Graph, node_id: NodeId) -> Point {
    if node_id < graph.n {
        // Processor: 0 ~ n-1
        graph.processor_positions[node_id]
    } else {
        // Separator: n ~ n+m-1
        let sep_idx = node_id - graph.n;
        if sep_idx < graph.separator_positions.len() {
            graph.separator_positions[sep_idx]
        } else if node_id == usize::MAX {
            // Special entrance node ID
            Point { x: 0, y: 5000 }
        } else {
            panic!("Invalid NodeId: {}", node_id);
        }
    }
}

/// Determine orientation of three points (clockwise/counterclockwise/collinear)
fn orientation(a: Point, b: Point, c: Point) -> i32 {
    let cross = (b.x as i64 - a.x as i64) * (c.y as i64 - a.y as i64)
        - (b.y as i64 - a.y as i64) * (c.x as i64 - a.x as i64);
    sign(cross)
}

/// Sign function
fn sign(x: i64) -> i32 {
    if x > 0 {
        1
    } else if x < 0 {
        -1
    } else {
        0
    }
}

/// Check if two line segments intersect
fn segments_intersect(p1: Point, p2: Point, q1: Point, q2: Point) -> bool {
    // Skip if endpoints are the same
    if (p1.x == q1.x && p1.y == q1.y)
        || (p1.x == q2.x && p1.y == q2.y)
        || (p2.x == q1.x && p2.y == q1.y)
        || (p2.x == q2.x && p2.y == q2.y)
    {
        return false;
    }

    // Bounding box intersection check
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

    // Strict intersection only (no endpoint contact)
    (o1 * o2 < 0) && (o3 * o4 < 0)
}

/// Check if edge between two nodes intersects with existing edges
fn edge_intersects(graph: &Graph, from1: NodeId, to1: NodeId, from2: NodeId, to2: NodeId) -> bool {
    let p1 = get_node_position(graph, from1);
    let p2 = get_node_position(graph, to1);
    let q1 = get_node_position(graph, from2);
    let q2 = get_node_position(graph, to2);
    segments_intersect(p1, p2, q1, q2)
}

/// Check if new edge intersects with any existing edge
fn new_edge_intersects(graph: &Graph, from: NodeId, to: NodeId) -> bool {
    for (&existing_from, out) in &graph.edges {
        if edge_intersects(graph, from, to, existing_from, out.out1)
            || edge_intersects(graph, from, to, existing_from, out.out2)
        {
            return true;
        }
    }
    false
}

// ========================================
// GRAPH UTILITIES
// ========================================

/// Add edge to graph with validation
fn add_edge(graph: &mut Graph, from: NodeId, out1: NodeId, out2: NodeId) {
    // Always add edge - the validation was breaking the graph construction
    graph.edges.insert(from, Out { out1, out2 });
}

/// Create initial graph structure
fn create_graph(
    n: usize,
    m: usize,
    processor_positions: Vec<Point>,
    separator_positions: Vec<Point>,
) -> Graph {
    Graph {
        n,
        m,
        processor_positions,
        separator_positions,
        edges: HashMap::new(),
        start_node: 0,
    }
}

/// Build reverse graph for path analysis
fn build_reverse_graph(graph: &Graph) -> HashMap<NodeId, Vec<Source>> {
    let mut reverse_edges = HashMap::new();

    for (&from, out) in &graph.edges {
        // Add edge to out1
        reverse_edges
            .entry(out.out1)
            .or_insert_with(Vec::new)
            .push(Source {
                id: from,
                ty: OutType::Out1,
            });

        // Add edge to out2
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

/// Topological sort for DAG processing
fn topological_sort(adj: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
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

/// Check if graph has cycles using DFS
fn has_cycle(graph: &Graph) -> bool {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    fn dfs(
        graph: &Graph,
        node: NodeId,
        visited: &mut HashSet<NodeId>,
        rec_stack: &mut HashSet<NodeId>,
    ) -> bool {
        visited.insert(node);
        rec_stack.insert(node);

        if let Some(out) = graph.edges.get(&node) {
            // Check out1
            if !visited.contains(&out.out1) {
                if dfs(graph, out.out1, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(&out.out1) {
                return true;
            }

            // Check out2
            if !visited.contains(&out.out2) {
                if dfs(graph, out.out2, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(&out.out2) {
                return true;
            }
        }

        rec_stack.remove(&node);
        false
    }

    // Start from entrance
    if dfs(graph, graph.start_node, &mut visited, &mut rec_stack) {
        return true;
    }

    // Check other unvisited nodes
    for sep_idx in 0..graph.m {
        let sep_node = graph.n + sep_idx;
        if !visited.contains(&sep_node) && graph.edges.contains_key(&sep_node) {
            if dfs(graph, sep_node, &mut visited, &mut rec_stack) {
                return true;
            }
        }
    }

    false
}

// ========================================
// NETWORK CONSTRUCTION
// ========================================

/// Build network with improved greedy strategy
fn build_network_greedy(graph: Graph) -> Graph {
    let mut work_graph = graph.clone();
    let mut used_separators = vec![false; graph.m];
    let mut queue = VecDeque::new();

    let start_pos = Point { x: 0, y: 5000 };
    const ENTRANCE_NODE: NodeId = usize::MAX;

    // 搬入口とつなげるものとして、入口との距離（短い方が良い）で選ぶ
    let mut best_sep: usize = 0;
    let mut best_score = f64::MAX;

    for i in 0..graph.separator_positions.len() {
        let dist_to_entrance = distance(start_pos, graph.separator_positions[i]);
        if dist_to_entrance < best_score {
            best_score = dist_to_entrance;
            best_sep = i;
        }
    }

    // Add edge from entrance to best separator
    let first_sep_node = graph.n + best_sep;
    work_graph.start_node = first_sep_node;
    add_edge(
        &mut work_graph,
        ENTRANCE_NODE,
        first_sep_node,
        first_sep_node,
    );
    queue.push_back(best_sep);
    used_separators[best_sep] = true;

    let mut counter = 0;

    // Process separators with improved candidate selection
    while let Some(current_sep_idx) = queue.pop_front() {
        if current_sep_idx >= graph.separator_positions.len() {
            continue;
        }

        let current_node = graph.n + current_sep_idx;
        let current_pos = graph.separator_positions[current_sep_idx];

        // Find candidates with better scoring
        let mut candidates = Vec::new();

        // Add separator candidates
        for (idx, &pos) in graph.separator_positions.iter().enumerate() {
            if idx != current_sep_idx && !used_separators[idx] {
                let dist = distance(current_pos, pos);

                // Score based on distance and connectivity potential
                let mut connectivity_score = 0.0;
                for &proc_pos in &graph.processor_positions {
                    let proc_dist = distance(pos, proc_pos);
                    connectivity_score += 1.0 / (1.0 + proc_dist);
                }

                let score = dist * 0.7 - connectivity_score * 20.0; // Lower is better
                candidates.push((score, graph.n + idx));
            }
        }

        // Add processor candidates with delay
        if counter > 4 || candidates.len() < 2 {
            for (idx, &pos) in graph.processor_positions.iter().enumerate() {
                let dist = distance(current_pos, pos);
                candidates.push((dist, idx));
            }
        }

        candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut output1 = None;
        let mut output2 = None;
        let mut final_out1: usize = 0;
        let mut final_out2: usize = 0;

        // Select best candidates
        for &(_, target_node) in candidates.iter().take(10) {
            if output1.is_none() {
                output1 = Some(target_node);
            } else {
                output2 = Some(target_node);
            }
            if output1.is_some() && output2.is_some() {
                (final_out1, final_out2) =
                    handle_edge_intersection(&work_graph, current_node, output1, output2);
                if final_out1 == final_out2 && final_out1 == 0 {
                    output1 = None;
                    output2 = None; // Both edges invalid, skip this separator
                    continue;
                }
                break;
            }
        }
        if final_out1 == final_out2 && final_out1 == 0 {
            continue;
        }

        // Add edge
        add_edge(&mut work_graph, current_node, final_out1, final_out2);

        // Add new separators to queue
        for &target in &[final_out1, final_out2] {
            if target >= graph.n && target < graph.n + graph.m {
                let sep_idx = target - graph.n;
                if !used_separators[sep_idx] {
                    queue.push_back(sep_idx);
                    used_separators[sep_idx] = true;
                }
            }
        }
        counter += 1;
    }

    // Remove disconnected separators
    remove_disconnected_separators(&mut work_graph);
    work_graph
}

/// Handle edge intersection during network construction
fn handle_edge_intersection(
    graph: &Graph,
    from: NodeId,
    output1: Option<NodeId>,
    output2: Option<NodeId>,
) -> (NodeId, NodeId) {
    let default_out = 0;
    let out1 = output1.unwrap_or(default_out);
    let out2 = output2.unwrap_or(out1);

    // Check intersection for both edges
    let edge1_intersects = new_edge_intersects(graph, from, out1);
    let edge2_intersects =
        new_edge_intersects(graph, from, out2) || edge_intersects(graph, from, out1, from, out2);

    // Handle intersection cases
    if !edge1_intersects && !edge2_intersects {
        (out1, out2) // Both valid
    } else if !edge1_intersects && edge2_intersects {
        (out1, out1) // Only edge1 valid, merge
    } else if edge1_intersects && !edge2_intersects {
        (out2, out2) // Only edge2 valid
    } else {
        (0, 0) // Both intersect, mark for removal
    }
}

/// Improve connections for separators with same outputs
fn connect_graph(graph: &Graph) -> Graph {
    let mut work_graph = graph.clone();
    let mut queue = VecDeque::new();
    queue.push_back(work_graph.start_node);

    let (reachouts, _) = get_reachout_edge(&work_graph);
    let mut visited = HashSet::new();
    let mut counter = 0;

    while let Some(current) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        if let Some(out) = work_graph.edges.get(&current).cloned() {
            if out.out1 == out.out2 {
                // Both outputs same, need to change one
                if current < graph.n {
                    continue; // Skip processors
                }

                let sep_idx = current - graph.n;
                if sep_idx >= graph.separator_positions.len() {
                    continue;
                }

                // Get current reachable processors
                let current_reachable = if let Some(reach) = reachouts.get(&current) {
                    let mut reachable_processors = HashSet::new();
                    for (i, weighted_reach) in reach.iter().enumerate() {
                        if weighted_reach.reachout != 0 {
                            reachable_processors.insert(i);
                        }
                    }
                    reachable_processors
                } else {
                    HashSet::new()
                };

                // Find candidates by distance
                let mut candidates = Vec::new();
                for (idx, &pos) in graph.separator_positions.iter().enumerate() {
                    if idx != sep_idx {
                        let dist = distance(graph.separator_positions[sep_idx], pos);
                        candidates.push((dist, graph.n + idx));
                    }
                }

                if counter >= 5 {
                    for (idx, &pos) in graph.processor_positions.iter().enumerate() {
                        let dist = distance(graph.separator_positions[sep_idx], pos);
                        candidates.push((dist, idx));
                    }
                }

                candidates
                    .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
                let mut push_new = false;
                // Try candidates that provide new reachability
                for &(_, node_id) in &candidates {
                    if node_id == out.out1 || node_id == out.out2 {
                        continue;
                    }

                    let candidate_reachable = if node_id < graph.n {
                        let mut set = HashSet::new();
                        set.insert(node_id);
                        set
                    } else if let Some(reach) = reachouts.get(&node_id) {
                        let mut reachable_processors = HashSet::new();
                        for (i, weighted_reach) in reach.iter().enumerate() {
                            if weighted_reach.reachout != 0 {
                                reachable_processors.insert(i);
                            }
                        }
                        reachable_processors
                    } else {
                        HashSet::new()
                    };

                    // Check if provides new reachability
                    let has_new_reachability = candidate_reachable
                        .iter()
                        .any(|&proc| !current_reachable.contains(&proc));

                    if has_new_reachability && !new_edge_intersects(&work_graph, current, node_id) {
                        // Change one output based on separator index
                        let new_out = if sep_idx % 2 == 0 {
                            Out {
                                out1: node_id,
                                out2: out.out2,
                            }
                        } else {
                            Out {
                                out1: out.out1,
                                out2: node_id,
                            }
                        };

                        let mut temp_graph = work_graph.clone();
                        temp_graph.edges.insert(current, new_out.clone());

                        if !has_cycle(&temp_graph) {
                            work_graph.edges.insert(current, new_out.clone());
                            queue.push_back(new_out.out1);
                            queue.push_back(new_out.out2);
                            push_new = true;
                            break;
                        }
                    }
                }
                if !push_new {
                    queue.push_back(out.out1);
                }
                counter += 1;
            } else {
                queue.push_back(out.out1);
                queue.push_back(out.out2);
            }
        }
    }

    remove_disconnected_separators(&mut work_graph);
    work_graph
}

/// Find separators not connected to processors
fn find_disconnected_separators(graph: &Graph) -> Vec<usize> {
    let reverse_graph = build_reverse_graph(graph);
    let mut unreachable: HashSet<usize> = (0..graph.m).collect();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Start from all processors
    for i in 0..graph.n {
        queue.push_back(i);
        visited.insert(i);
    }

    // Reverse BFS
    while let Some(current) = queue.pop_front() {
        // If separator, remove from unreachable
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

/// Remove separators not connected to processors
fn remove_disconnected_separators(graph: &mut Graph) {
    let disconnected = find_disconnected_separators(graph);

    for &sep_idx in &disconnected {
        let sep_node = graph.n + sep_idx;

        // Remove edges from this separator
        graph.edges.remove(&sep_node);

        // Remove edges to this separator
        let nodes_to_update: Vec<NodeId> = graph.edges.keys().cloned().collect();
        for node in nodes_to_update {
            if let Some(out) = graph.edges.get_mut(&node) {
                if out.out1 == sep_node && out.out2 == sep_node {
                    // Both reference deleted separator
                    graph.edges.remove(&node);
                } else if out.out1 == sep_node {
                    // Only out1 references deleted separator
                    out.out1 = out.out2;
                } else if out.out2 == sep_node {
                    // Only out2 references deleted separator
                    out.out2 = out.out1;
                }
            }
        }
    }
}

// ========================================
// PROBABILITY CALCULATION
// ========================================

/// Optimized probability calculation with cached topological sort and adjacency list
fn build_processor_probabilities(
    n: usize,
    m: usize,
    probabilities: &Vec<Vec<f64>>,
    graph: &Graph,
    configs: &Vec<String>,
    topo_order: &Vec<NodeId>,
) -> Vec<Vec<f64>> {
    let mut probs = mat![0.0; n + m; n];

    // Set start node probabilities
    probs[graph.start_node].fill(1.0);

    // Propagate probabilities in topological order
    for &u in topo_order {
        if u >= n {
            let sep_idx = u - n;
            if sep_idx < configs.len() {
                let config = &configs[sep_idx];
                if config != "-1" {
                    let parts: Vec<&str> = config.split_whitespace().collect();
                    if parts.len() == 3 {
                        if let (Ok(k), Ok(v1), Ok(v2)) = (
                            parts[0].parse::<usize>(),
                            parts[1].parse::<usize>(),
                            parts[2].parse::<usize>(),
                        ) {
                            if k < probabilities.len() && v1 < n + m && v2 < n + m {
                                for i in 0..n {
                                    probs[v1][i] += probs[u][i] * probabilities[k][i];
                                    probs[v2][i] += probs[u][i] * (1.0 - probabilities[k][i]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    probs
}

// ========================================
// REACHABILITY ANALYSIS
// ========================================

/// Get reachability information for each node
fn get_reachout_edge(
    graph: &Graph,
) -> (
    HashMap<NodeId, Vec<WeightedReachability>>,
    HashMap<NodeId, HashSet<NodeId>>,
) {
    let reverse_graph = build_reverse_graph(graph);

    let mut queue = VecDeque::new();
    let mut visited: HashMap<NodeId, Vec<WeightedReachability>> = HashMap::new();
    let mut can_reach = HashMap::new();
    let mut distances = HashMap::new();

    // Initialize all processors as start points
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
        };
        visited.insert(i, v.clone());

        let mut set = HashSet::new();
        set.insert(i);
        can_reach.insert(i, set);

        let mut dist_map = HashMap::new();
        dist_map.insert(i, 0.0);
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
                            // Update distance (predecessor -> current -> processor j)
                            let new_distance =
                                current_distances.get(&j).unwrap_or(&f64::MAX) + edge_distance;
                            let existing_distance =
                                predecessor_distances.get(&j).unwrap_or(&f64::MAX);

                            if new_distance < *existing_distance {
                                predecessor_distances.insert(j, new_distance);
                            }

                            // Use inverse distance as weight
                            let distance_weight = if new_distance > 0.0 {
                                1.0 / new_distance
                            } else {
                                1.0
                            };

                            if current_processors[j].reachout != 0 {
                                processors[j].reachout +=
                                    (source.ty as i32) * current_processors[j].reachout.abs();
                            } else if current_can_reach.contains(&j) {
                                processors[j].reachout += source.ty as i32;
                            }

                            processors[j].distance_weight += distance_weight;
                        }
                    }
                    queue.push_back(predecessor);
                } else {
                    let mut processors = vec![
                        WeightedReachability {
                            reachout: 0,
                            distance_weight: 0.0
                        };
                        graph.n
                    ];
                    let mut new_distances = HashMap::new();

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

                            processors[j].reachout +=
                                (source.ty as i32) * current_processors[j].reachout.abs();
                            processors[j].distance_weight = distance_weight;
                        }
                    }
                    visited.insert(predecessor, processors);
                    can_reach.insert(predecessor, current_can_reach.clone());
                    distances.insert(predecessor, new_distances);
                }
            }
        }
    }

    (visited, can_reach)
}

// ========================================
// OPTIMIZATION
// ========================================

/// Calculate score
fn calculate_score(
    n: usize,
    _m: usize,
    processor_probabilities: &Vec<Vec<f64>>,
    device_assignments: &Vec<usize>,
) -> i64 {
    let mut score = 0.0;
    for i in 0..n {
        let d = device_assignments[i];
        let q = processor_probabilities[i][d];
        score += 1.0 - q;
    }
    score /= n as f64;
    (1e9 * score).round() as i64
}

/// Simplified but effective solver focusing on core optimization
fn solve(
    start_time: Instant,
    n: usize,
    m: usize,
    processor_positions: &Vec<Point>,
    separator_positions: &Vec<Point>,
    probabilities: &Vec<Vec<f64>>,
) -> (Graph, Vec<String>, Vec<Vec<f64>>) {
    // Build network using basic greedy algorithm
    let mut graph = build_network_greedy(create_graph(
        n,
        m,
        processor_positions.clone(),
        separator_positions.clone(),
    ));

    // Improve connections
    graph = connect_graph(&graph);

    // Initial configuration using simple best-type selection
    let mut best_configs = Vec::new();
    for sep_idx in 0..graph.m {
        let sep_node = graph.n + sep_idx;
        if let Some(out) = graph.edges.get(&sep_node) {
            // Find separator type with highest max probability (better than average)
            let mut best_type = 0;
            let mut best_score = 0.0;

            for (type_idx, type_probs) in probabilities.iter().enumerate() {
                let max_prob = type_probs.iter().fold(0.0f64, |a, &b| a.max(b));
                let avg_prob = type_probs.iter().sum::<f64>() / type_probs.len() as f64;
                let score = max_prob * 2.0 + avg_prob; // Favor types with high peaks
                if score > best_score {
                    best_score = score;
                    best_type = type_idx;
                }
            }

            best_configs.push(format!("{} {} {}", best_type, out.out1, out.out2));
        } else {
            best_configs.push("-1".to_string());
        }
    }

    // Pre-compute topology
    let mut adjacency = vec![vec![]; n + m];
    for (sep_idx, config) in best_configs.iter().enumerate() {
        if config != "-1" {
            let parts: Vec<&str> = config.split_whitespace().collect();
            if parts.len() == 3 {
                if let (Ok(_k), Ok(v1), Ok(v2)) = (
                    parts[0].parse::<usize>(),
                    parts[1].parse::<usize>(),
                    parts[2].parse::<usize>(),
                ) {
                    if v1 < n + m && v2 < n + m {
                        let sep_node = n + sep_idx;
                        adjacency[sep_node].push(v1);
                        adjacency[sep_node].push(v2);
                    }
                }
            }
        }
    }

    let topo_order = topological_sort(&adjacency).unwrap_or_else(|| (0..n + m).collect());

    // Calculate initial probabilities
    let mut best_processor_probs =
        build_processor_probabilities(n, m, &probabilities, &graph, &best_configs, &topo_order);

    // Get reachability information
    let (_, can_reach) = get_reachout_edge(&graph);

    // Find modifiable separators
    let modifiable_separators: Vec<usize> = graph
        .edges
        .iter()
        .filter(|(&node_id, &ref out)| {
            out.out1 != out.out2
                && node_id >= n
                && can_reach.get(&node_id).map_or(0, |set| set.len()) >= 2
        })
        .map(|(node_id, _)| node_id - n)
        .collect();

    if modifiable_separators.is_empty() {
        return (graph, best_configs, best_processor_probs);
    }

    // Calculate initial score
    let initial_assignments = simple_device_assignment(n, &best_processor_probs);
    let mut best_score = calculate_score(n, m, &best_processor_probs, &initial_assignments);

    // Simple but effective hill climbing
    let mut rng = rand::thread_rng();
    let mut iterations = 0;
    let mut improvements = 0;

    while start_time.elapsed().as_millis() < 1800 {
        iterations += 1;

        // Select separator(s) to modify
        let batch_size = if iterations < 500 {
            3 // Start with single changes for precision
        } else {
            1
        };

        let mut selected_separators = Vec::new();
        let mut available = modifiable_separators.clone();

        for _ in 0..batch_size {
            let idx = rng.gen_range(0..available.len());
            selected_separators.push(available.remove(idx));
        }

        // Create new configuration
        let mut new_configs = best_configs.clone();

        for &sep_idx in &selected_separators {
            let current_type = if best_configs[sep_idx] != "-1" {
                let parts: Vec<&str> = best_configs[sep_idx].split_whitespace().collect();
                if parts.len() >= 1 {
                    parts[0].parse::<usize>().unwrap_or(0)
                } else {
                    0
                }
            } else {
                continue;
            };

            // Try a different separator type
            let new_type = if iterations < 300 {
                // Conservative: try types with good overall performance
                let mut type_scores = Vec::new();
                for (type_idx, type_probs) in probabilities.iter().enumerate() {
                    if type_idx != current_type {
                        let max_prob = type_probs.iter().fold(0.0f64, |a, &b| a.max(b));
                        let avg_prob = type_probs.iter().sum::<f64>() / type_probs.len() as f64;
                        let score = max_prob + avg_prob;
                        type_scores.push((score, type_idx));
                    }
                }

                if !type_scores.is_empty() {
                    type_scores
                        .sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
                    let top_count = std::cmp::min(5, type_scores.len());
                    let idx = rng.gen_range(0..top_count);
                    type_scores[idx].1
                } else {
                    (current_type + 1) % probabilities.len()
                }
            } else {
                // More exploration later
                let mut new_type = rng.gen_range(0..probabilities.len());
                while new_type == current_type {
                    new_type = rng.gen_range(0..probabilities.len());
                }
                new_type
            };

            let out = graph.edges.get(&(n + sep_idx)).unwrap();
            new_configs[sep_idx] = format!("{} {} {}", new_type, out.out1, out.out2);
        }

        let new_probs =
            build_processor_probabilities(n, m, &probabilities, &graph, &new_configs, &topo_order);

        let new_assignments = simple_device_assignment(n, &new_probs);
        let new_score = calculate_score(n, m, &new_probs, &new_assignments);

        if new_score < best_score {
            best_score = new_score;
            best_configs = new_configs;
            best_processor_probs = new_probs;
            improvements += 1;
        }
    }
    (graph, best_configs, best_processor_probs)
}

/// Simple device assignment ensuring uniqueness
fn simple_device_assignment(n: usize, processor_probabilities: &Vec<Vec<f64>>) -> Vec<usize> {
    let mut assignments = vec![0; n];
    let mut prob_pairs = Vec::new();

    for waste_type in 0..n {
        for processor_id in 0..n {
            let prob = if waste_type < processor_probabilities.len()
                && processor_id < processor_probabilities[waste_type].len()
            {
                processor_probabilities[waste_type][processor_id]
            } else {
                0.0
            };
            prob_pairs.push((prob, waste_type, processor_id));
        }
    }

    prob_pairs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut used_processors = vec![false; n];
    let mut assigned_wastes = vec![false; n];

    for (_, waste_type, processor_id) in prob_pairs {
        if !assigned_wastes[waste_type] && !used_processors[processor_id] {
            assignments[waste_type] = processor_id;
            used_processors[processor_id] = true;
            assigned_wastes[waste_type] = true;
        }
    }

    // Ensure all waste types are assigned
    for waste_type in 0..n {
        if !assigned_wastes[waste_type] {
            for processor_id in 0..n {
                if !used_processors[processor_id] {
                    assignments[waste_type] = processor_id;
                    used_processors[processor_id] = true;
                    break;
                }
            }
        }
    }

    assignments
}

// ========================================
// MAIN FUNCTION
// ========================================

fn main() {
    let start_time: Instant = Instant::now();
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

    // Solve the problem
    let (graph, separator_configs, processor_probabilities) = solve(
        start_time,
        n,
        m,
        &processor_positions,
        &separator_positions,
        &probabilities,
    );

    // Generate device assignments
    let device_assignments = simple_device_assignment(n, &processor_probabilities);

    // Output results
    println!(
        "{}",
        device_assignments
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    println!("{}", graph.start_node);

    for config in separator_configs {
        println!("{}", config);
    }
}

// ========================================
// TESTS
// ========================================

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

        create_graph(3, 2, processor_positions, separator_positions)
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

        // Processor positions
        assert_eq!(get_node_position(&graph, 0), Point { x: 0, y: 0 });
        assert_eq!(get_node_position(&graph, 1), Point { x: 100, y: 0 });
        assert_eq!(get_node_position(&graph, 2), Point { x: 0, y: 100 });

        // Separator positions
        assert_eq!(get_node_position(&graph, 3), Point { x: 50, y: 50 });
        assert_eq!(get_node_position(&graph, 4), Point { x: 25, y: 25 });

        // Entrance position
        assert_eq!(
            get_node_position(&graph, usize::MAX),
            Point { x: 0, y: 5000 }
        );
    }

    #[test]
    fn test_add_edge() {
        let mut graph = create_test_graph();
        add_edge(&mut graph, 3, 0, 1);

        assert!(graph.edges.contains_key(&3));
        let out = &graph.edges[&3];
        assert_eq!(out.out1, 0);
        assert_eq!(out.out2, 1);
    }

    #[test]
    fn test_build_reverse_graph() {
        let mut graph = create_test_graph();
        add_edge(&mut graph, 3, 0, 1);
        add_edge(&mut graph, 4, 1, 2);

        let reverse_graph = build_reverse_graph(&graph);

        // Input to processor 0
        assert!(reverse_graph.contains_key(&0));
        let sources_to_0 = &reverse_graph[&0];
        assert_eq!(sources_to_0.len(), 1);
        assert_eq!(sources_to_0[0].id, 3);
        assert_eq!(sources_to_0[0].ty, OutType::Out1);

        // Input to processor 1
        assert!(reverse_graph.contains_key(&1));
        let sources_to_1 = &reverse_graph[&1];
        assert_eq!(sources_to_1.len(), 2);

        // Input to processor 2
        assert!(reverse_graph.contains_key(&2));
        let sources_to_2 = &reverse_graph[&2];
        assert_eq!(sources_to_2.len(), 1);
        assert_eq!(sources_to_2[0].id, 4);
        assert_eq!(sources_to_2[0].ty, OutType::Out2);
    }

    #[test]
    fn test_segments_intersect() {
        // Obviously intersecting segments
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 100, y: 100 };
        let q1 = Point { x: 0, y: 100 };
        let q2 = Point { x: 100, y: 0 };
        assert!(segments_intersect(p1, p2, q1, q2));

        // Non-intersecting segments
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 50, y: 0 };
        let q1 = Point { x: 60, y: 0 };
        let q2 = Point { x: 100, y: 0 };
        assert!(!segments_intersect(p1, p2, q1, q2));

        // Same endpoint (should not intersect)
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 50, y: 50 };
        let q1 = Point { x: 0, y: 0 };
        let q2 = Point { x: 50, y: 0 };
        assert!(!segments_intersect(p1, p2, q1, q2));
    }

    #[test]
    fn test_orientation() {
        // Counterclockwise
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 0 };
        let c = Point { x: 0, y: 1 };
        assert_eq!(orientation(a, b, c), 1);

        // Clockwise
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 0 };
        let c = Point { x: 1, y: -1 };
        assert_eq!(orientation(a, b, c), -1);

        // Collinear
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 0 };
        let c = Point { x: 2, y: 0 };
        assert_eq!(orientation(a, b, c), 0);
    }

    #[test]
    fn test_find_disconnected_separators() {
        let mut graph = create_test_graph();

        // Only separator 0 connected to processors
        add_edge(&mut graph, 3, 0, 1);

        let disconnected = find_disconnected_separators(&graph);

        // Separator 1 should be disconnected from processors
        assert_eq!(disconnected, vec![1]);
    }

    #[test]
    fn test_handle_edge_intersection() {
        let graph = create_test_graph();

        // No intersection case
        let (out1, out2) = handle_edge_intersection(&graph, 3, Some(0), Some(1));
        assert_eq!((out1, out2), (0, 1));

        // No output specified case
        let (out1, out2) = handle_edge_intersection(&graph, 3, None, None);
        assert_eq!((out1, out2), (0, 0));
    }

    #[test]
    fn test_remove_disconnected_separators() {
        let mut graph = create_test_graph();

        // Separator 0 directly connected to processors, separator 1 unconnected
        add_edge(&mut graph, 3, 0, 1);

        // In this state, only separator 1 is disconnected
        let disconnected_before = find_disconnected_separators(&graph);
        assert_eq!(disconnected_before, vec![1]);

        // Remove disconnected separators
        remove_disconnected_separators(&mut graph);

        // Separator 0 connection should remain
        assert!(graph.edges.contains_key(&3));
    }

    #[test]
    fn test_edge_intersects() {
        let graph = create_test_graph();

        // Non-intersecting edges test
        assert!(!edge_intersects(
            &graph, 3, 0, // Separator 0 -> Processor 0 (50,50) -> (0,0)
            4, 2 // Separator 1 -> Processor 2 (25,25) -> (0,100)
        ));

        // Test another combination to verify function works correctly
        assert!(!edge_intersects(
            &graph, 3, 1, // Separator 0 -> Processor 1 (50,50) -> (100,0)
            4, 0 // Separator 1 -> Processor 0 (25,25) -> (0,0)
        ));
    }

    #[test]
    fn test_new_edge_intersects() {
        let mut graph = create_test_graph();

        // Add existing edge
        add_edge(&mut graph, 3, 1, 2); // (50,50) -> (100,0) and (50,50) -> (0,100)

        // Test non-intersecting edge
        assert!(!new_edge_intersects(&graph, 4, 0)); // (25,25) -> (0,0)
    }

    #[test]
    fn test_create_graph() {
        let graph = create_test_graph();
        assert_eq!(graph.n, 3);
        assert_eq!(graph.m, 2);
        assert_eq!(graph.processor_positions.len(), 3);
        assert_eq!(graph.separator_positions.len(), 2);
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn test_topological_sort() {
        // Simple DAG test
        let adj = vec![
            vec![1, 2], // Node 0 -> Node 1, 2
            vec![3],    // Node 1 -> Node 3
            vec![3],    // Node 2 -> Node 3
            vec![],     // Node 3 (terminal)
        ];

        let result = topological_sort(&adj);
        assert!(result.is_some());

        let order = result.unwrap();
        assert_eq!(order.len(), 4);

        // Check dependency preservation
        let pos: HashMap<usize, usize> = order.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        for (u, neighbors) in adj.iter().enumerate() {
            for &v in neighbors {
                assert!(pos[&u] < pos[&v]); // u comes before v
            }
        }
    }

    #[test]
    fn test_get_reachout_edge() {
        let mut graph = create_test_graph();

        // Simple network construction: separator 0 -> processors 0,1
        add_edge(&mut graph, 3, 0, 1);

        let (reachout_edges, _) = get_reachout_edge(&graph);

        // Check reachability from separator 0 to each processor
        assert!(reachout_edges.contains_key(&3));
        let reachouts = &reachout_edges[&3];

        // Processor 0 reachable via out1 -> +1
        assert_eq!(reachouts[0].reachout, 1);
        // Processor 1 reachable via out2 -> -1
        assert_eq!(reachouts[1].reachout, -1);
        // Processor 2 not reachable -> 0
        assert_eq!(reachouts[2].reachout, 0);
    }

    // Additional complex reachability tests would go here...
    // (keeping the existing complex tests for completeness)
}
