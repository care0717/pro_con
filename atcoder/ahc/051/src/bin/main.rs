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
    probabilities: Vec<Vec<f64>>, // [separator_type][waste_type] -> probability
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

/// Add edge to graph
fn add_edge(graph: &mut Graph, from: NodeId, out1: NodeId, out2: NodeId) {
    graph.edges.insert(from, Out { out1, out2 });
}

/// Create initial graph structure
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

/// Build network using greedy algorithm with intersection handling
fn build_network_greedy(graph: Graph) -> Graph {
    let mut work_graph = graph.clone();
    let mut used_separators = vec![false; graph.m];
    let mut queue = VecDeque::new();

    if graph.separator_positions.is_empty() {
        return graph.clone();
    }

    let start_pos = Point { x: 0, y: 5000 };
    const ENTRANCE_NODE: NodeId = usize::MAX;

    // Find nearest separator to entrance
    let mut min_dist = f64::MAX;
    let mut nearest_sep = 0;
    for i in 0..graph.separator_positions.len() {
        let dist = distance(start_pos, graph.separator_positions[i]);
        if dist < min_dist {
            min_dist = dist;
            nearest_sep = i;
        }
    }

    // Add edge from entrance to first separator
    let first_sep_node = graph.n + nearest_sep;
    work_graph.start_node = first_sep_node;
    add_edge(
        &mut work_graph,
        ENTRANCE_NODE,
        first_sep_node,
        first_sep_node,
    );
    queue.push_back(nearest_sep);
    used_separators[nearest_sep] = true;
    let mut counter = 0;

    // Process separators from queue (greedy method)
    while let Some(current_sep_idx) = queue.pop_front() {
        if current_sep_idx >= graph.separator_positions.len() {
            continue;
        }

        let current_node = graph.n + current_sep_idx;

        // Find nearest candidates
        let mut candidates = Vec::new();
        for (idx, &pos) in graph.separator_positions.iter().enumerate() {
            if idx != current_sep_idx {
                let dist = distance(graph.separator_positions[current_sep_idx], pos);
                if !used_separators[idx] {
                    candidates.push((dist, graph.n + idx));
                }
            }
        }

        // Add processors after counter > 5
        if counter > 5 {
            for (idx, &pos) in graph.processor_positions.iter().enumerate() {
                let dist = distance(graph.separator_positions[current_sep_idx], pos);
                candidates.push((dist, idx));
            }
        }

        candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut output1 = None;
        let mut output2 = None;

        // Select first two candidates
        for &(_, target_node) in candidates.iter().take(2) {
            if output1.is_none() {
                output1 = Some(target_node);
            } else {
                output2 = Some(target_node);
                break;
            }
        }

        // Handle edge intersection and adjustment
        let (final_out1, final_out2) =
            handle_edge_intersection(&work_graph, current_node, output1, output2);

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

    // Remove disconnected separators after construction
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
                            break;
                        }
                    }
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
// ROUTE-BASED SPECIALIZATION
// ========================================

/// Build processor-dedicated routes for specialization
fn build_processor_dedicated_routes(graph: &Graph) -> HashMap<usize, Vec<usize>> {
    let mut routes = HashMap::new();
    let reverse_graph = build_reverse_graph(graph);

    for processor_id in 0..graph.n {
        let route = trace_route_to_start(processor_id, &reverse_graph, graph.start_node, graph.n);
        routes.insert(processor_id, route);
    }

    routes
}

/// Trace route from processor to start in reverse order
fn trace_route_to_start(
    processor_id: usize,
    reverse_graph: &HashMap<NodeId, Vec<Source>>,
    start_node: NodeId,
    n: usize,
) -> Vec<usize> {
    let mut route = Vec::new();
    let mut current = processor_id;
    let mut visited = HashSet::new();

    while current != start_node && !visited.contains(&current) {
        visited.insert(current);

        if let Some(predecessors) = reverse_graph.get(&current) {
            // Select predecessor with highest "specialization" (least overlap)
            let best_predecessor = predecessors
                .iter()
                .min_by_key(|source| calculate_route_overlap_penalty(source.id, &reverse_graph))
                .map(|source| source.id);

            if let Some(next_node) = best_predecessor {
                if next_node >= n {
                    // If separator, add to route
                    route.push(next_node - n);
                }
                current = next_node;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    route
}

/// Calculate route overlap penalty (sharing degree)
fn calculate_route_overlap_penalty(
    node_id: NodeId,
    reverse_graph: &HashMap<NodeId, Vec<Source>>,
) -> usize {
    // Simple implementation: number of edges from that node (higher = more shared)
    reverse_graph
        .get(&node_id)
        .map(|sources| sources.len())
        .unwrap_or(0)
}

/// Assign separators to processors they mainly contribute to
fn assign_separators_to_processors(
    _graph: &Graph,
    routes: &HashMap<usize, Vec<usize>>,
) -> HashMap<usize, Vec<usize>> {
    let mut assignments: HashMap<usize, Vec<usize>> = HashMap::new();

    for (processor_id, route) in routes {
        for &sep_idx in route {
            assignments
                .entry(sep_idx)
                .or_insert_with(Vec::new)
                .push(*processor_id);
        }
    }

    assignments
}

/// Select separator type specialized for assigned processors
fn select_processor_specialized_separator_type(
    assigned_processors: &Vec<usize>,
    probabilities: &Vec<Vec<f64>>,
) -> usize {
    if assigned_processors.is_empty() {
        return 0; // Default
    }

    let mut best_type = 0;
    let mut best_score = f64::NEG_INFINITY;

    for (type_idx, type_probs) in probabilities.iter().enumerate() {
        let mut score = 0.0;

        // Calculate concentration for assigned processors
        for &processor_id in assigned_processors {
            if processor_id < type_probs.len() {
                // High probability for this processor's waste type
                score += type_probs[processor_id];
            }
        }

        // Penalty for non-assigned processors
        for processor_id in 0..type_probs.len() {
            if !assigned_processors.contains(&processor_id) {
                // Low probability for non-assigned processors is better
                score -= type_probs[processor_id] * 0.3; // Penalty weight
            }
        }

        // Specialization bonus (fewer assigned processors = higher specialization)
        let specialization_bonus = 1.0 / (assigned_processors.len() as f64 + 1.0);
        score *= 1.0 + specialization_bonus;

        if score > best_score {
            best_score = score;
            best_type = type_idx;
        }
    }

    best_type
}

/// Generate configs from graph using processor-dedicated route approach
fn generate_configs_from_graph(graph: &Graph) -> Vec<String> {
    let mut configs = vec!["-1".to_string(); graph.m];

    // Build processor-dedicated routes
    let processor_dedicated_routes = build_processor_dedicated_routes(graph);

    // Assign separators to processors
    let separator_processor_assignments =
        assign_separators_to_processors(graph, &processor_dedicated_routes);

    for sep_idx in 0..graph.m {
        let sep_node = graph.n + sep_idx;

        if let Some(out) = graph.edges.get(&sep_node) {
            // Get assigned processors for this separator
            let assigned_processors = separator_processor_assignments
                .get(&sep_idx)
                .cloned()
                .unwrap_or_default();

            // Select optimal separator type for assigned processors
            let separator_type = select_processor_specialized_separator_type(
                &assigned_processors,
                &graph.probabilities,
            );

            let v1 = out.out1;
            let v2 = out.out2;
            configs[sep_idx] = format!("{} {} {}", separator_type, v1, v2);
        }
    }

    configs
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

/// Generate optimal device assignments based on probabilities
fn generate_optimal_device_assignments(
    n: usize,
    processor_probabilities: &Vec<Vec<f64>>,
) -> Vec<usize> {
    let mut assignments = vec![0; n];

    // Create (probability, waste_type, processor_id) tuples
    let mut probability_pairs = Vec::new();
    for processor_id in 0..n {
        for waste_type in 0..n {
            let prob = processor_probabilities[processor_id][waste_type];
            probability_pairs.push((prob, waste_type, processor_id));
        }
    }

    // Sort by probability (descending)
    probability_pairs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut used_processors = vec![false; n];
    let mut assigned_wastes = vec![false; n];

    // Assign in probability order
    for (_, waste_type, processor_id) in probability_pairs {
        if !assigned_wastes[waste_type] && !used_processors[processor_id] {
            assignments[waste_type] = processor_id;
            used_processors[processor_id] = true;
            assigned_wastes[waste_type] = true;
        }
    }

    assignments
}

/// Fast device assignment calculation with memoization
fn generate_optimal_device_assignments_fast(
    n: usize,
    processor_probabilities: &Vec<Vec<f64>>,
    assignments: &Vec<usize>,
) -> Vec<usize> {
    // Use cached assignments if probability pattern is similar

    // Simple heuristic: if cached assignment still looks reasonable, use it
    let mut total_prob = 0.0;
    for i in 0..n {
        if assignments[i] < processor_probabilities.len()
            && i < processor_probabilities[assignments[i]].len()
        {
            total_prob += processor_probabilities[assignments[i]][i];
        }
    }

    // If cached assignments give reasonable probabilities (>50% on average), keep them
    if total_prob / n as f64 > 0.5 {
        return assignments.clone();
    }

    // Fallback to full calculation
    generate_optimal_device_assignments(n, processor_probabilities)
}

/// Fast score calculation with cached assignments
fn calculate_score_with_optimal_assignments(
    n: usize,
    m: usize,
    processor_probabilities: &Vec<Vec<f64>>,
    assignments: &Vec<usize>,
) -> (i64, Vec<usize>) {
    let optimal_assignments =
        generate_optimal_device_assignments_fast(n, processor_probabilities, assignments);
    (
        calculate_score(n, m, processor_probabilities, &optimal_assignments),
        optimal_assignments,
    )
}

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

/// Main solving function with stage-2 optimization (safe cached topology + assignments)
fn solve(
    start_time: Instant,
    n: usize,
    m: usize,
    processor_positions: &Vec<Point>,
    separator_positions: &Vec<Point>,
    probabilities: &Vec<Vec<f64>>,
) -> (Graph, Vec<String>, Vec<Vec<f64>>) {
    // Build network using distance-based greedy algorithm (with intersection handling)
    let mut graph = build_network_greedy(create_graph(
        n,
        m,
        processor_positions.clone(),
        separator_positions.clone(),
        probabilities.clone(),
    ));

    // Improve connections for separators with out1==out2
    graph = connect_graph(&graph);

    // Hill climbing optimization: change separator types randomly
    let mut rng = rand::thread_rng();

    // Find modifiable separators (out1 != out2)
    let modifiable_separators = &graph
        .edges
        .iter()
        .filter(|(&node_id, &ref out)| out.out1 != out.out2 && node_id >= n)
        .map(|(node_id, _)| node_id - n)
        .collect::<Vec<_>>();

    // Initial configuration calculation
    let mut best_configs = generate_configs_from_graph(&graph);

    // Stage 1 optimization: Pre-compute adjacency list and topological order
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

    // Use optimized probability calculation (stage 1 only)
    let mut best_processor_probs =
        build_processor_probabilities(n, m, &probabilities, &graph, &best_configs, &topo_order);

    // Stage 2 optimization: Cache initial device assignments
    let initial_assignments = generate_optimal_device_assignments(n, &best_processor_probs);
    let (mut best_score, mut best_assignments) =
        calculate_score_with_optimal_assignments(n, m, &best_processor_probs, &initial_assignments);

    // Hill climbing with stage 1+2 optimizations (NO stage 3 differential updates)
    let mut counter = 0;
    let mut improvements = 0;
    while start_time.elapsed().as_millis() < 1900 && !modifiable_separators.is_empty() {
        counter += 1;

        // Dynamic batch size based on improvement rate
        let batch_size = if improvements > counter / 15 {
            std::cmp::min(6, modifiable_separators.len()) // Larger batch if doing well
        } else {
            std::cmp::min(3, modifiable_separators.len()) // Smaller batch for precision
        };

        let mut selected_separators = Vec::new();
        let mut available_separators = modifiable_separators.clone();

        for _ in 0..batch_size {
            if available_separators.is_empty() {
                break;
            }
            let idx = rng.gen_range(0..available_separators.len());
            let selected_sep_idx = available_separators.remove(idx);
            selected_separators.push(selected_sep_idx);
        }

        // Create new configuration based on current best
        let mut new_configs = best_configs.clone();
        let mut all_changes_valid = true;

        // Change types of selected separators randomly
        for &selected_sep_idx in &selected_separators {
            let selected_node = n + selected_sep_idx;

            // Get current separator type
            let current_type = if best_configs[selected_sep_idx] != "-1" {
                let parts: Vec<&str> = best_configs[selected_sep_idx].split_whitespace().collect();
                if parts.len() >= 1 {
                    parts[0].parse::<usize>().unwrap_or(0)
                } else {
                    0
                }
            } else {
                continue; // Skip unconnected separators
            };

            // Select different separator type randomly
            let mut new_type = rng.gen_range(0..probabilities.len());
            while new_type == current_type && probabilities.len() > 1 {
                new_type = rng.gen_range(0..probabilities.len());
            }

            // Change separator type
            if let Some(out) = graph.edges.get(&selected_node) {
                new_configs[selected_sep_idx] = format!("{} {} {}", new_type, out.out1, out.out2);
            } else {
                all_changes_valid = false;
                break;
            }
        }

        // Use ONLY stage 1+2 optimizations (cached topology + fast assignments)
        if all_changes_valid && !selected_separators.is_empty() {
            let new_processor_probs = build_processor_probabilities(
                n,
                m,
                &probabilities,
                &graph,
                &new_configs,
                &topo_order,
            );
            let (new_score, new_assignments) = calculate_score_with_optimal_assignments(
                n,
                m,
                &new_processor_probs,
                &best_assignments,
            );

            // Accept entire batch if improved
            if new_score < best_score {
                best_score = new_score;
                best_assignments = new_assignments;
                best_configs = new_configs;
                best_processor_probs = new_processor_probs;
                improvements += 1;
            }
        }

        // Occasionally try single separator changes for fine-tuning
        if counter % 7 == 0 && !modifiable_separators.is_empty() {
            let single_sep = modifiable_separators[rng.gen_range(0..modifiable_separators.len())];
            let current_type = if best_configs[single_sep] != "-1" {
                let parts: Vec<&str> = best_configs[single_sep].split_whitespace().collect();
                if parts.len() >= 1 {
                    parts[0].parse::<usize>().unwrap_or(0)
                } else {
                    0
                }
            } else {
                continue;
            };

            let new_type = rng.gen_range(0..probabilities.len());
            if new_type != current_type {
                let mut single_config = best_configs.clone();
                if let Some(out) = graph.edges.get(&(n + single_sep)) {
                    single_config[single_sep] = format!("{} {} {}", new_type, out.out1, out.out2);

                    let single_probs = build_processor_probabilities(
                        n,
                        m,
                        &probabilities,
                        &graph,
                        &single_config,
                        &topo_order,
                    );
                    let (single_score, single_assignments) =
                        calculate_score_with_optimal_assignments(
                            n,
                            m,
                            &single_probs,
                            &best_assignments,
                        );

                    if single_score < best_score {
                        best_score = single_score;
                        best_assignments = single_assignments;
                        best_configs = single_config;
                        best_processor_probs = single_probs;
                        improvements += 1;
                    }
                }
            }
        }
    }

    // eprintln!(
    //     "Hill climbing: {} iterations, {} improvements, final score: {}",
    //     counter, improvements, best_score
    // );
    (graph, best_configs, best_processor_probs)
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

    // Solve with hill climbing optimization (1 second dedicated)
    let (graph, separator_configs, processor_probabilities) = solve(
        start_time,
        n,
        m,
        &processor_positions,
        &separator_positions,
        &probabilities,
    );

    // Generate optimal device assignments based on probabilities
    let initial_device_assignments =
        generate_optimal_device_assignments(n, &processor_probabilities);

    let mut best_score =
        calculate_score(n, m, &processor_probabilities, &initial_device_assignments);
    let mut best_device_assignments = initial_device_assignments.clone();

    // Continue optimizing device assignments until time limit
    let mut rng = rand::thread_rng();
    let current_assignments = initial_device_assignments;

    while start_time.elapsed().as_millis() < 1500 {
        let mut cloned_assignments = current_assignments.clone();

        // Shuffle assignments randomly
        for i in 0..cloned_assignments.len() {
            let j = rng.gen_range(0..cloned_assignments.len());
            cloned_assignments.swap(i, j);
        }

        let score = calculate_score(n, m, &processor_probabilities, &cloned_assignments);
        if score < best_score {
            best_score = score;
            best_device_assignments = cloned_assignments.clone();
        }
    }

    // Output results
    print!(
        "{}",
        best_device_assignments
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!();
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
    fn test_build_processor_dedicated_routes() {
        let mut graph = create_test_graph();
        add_edge(&mut graph, 3, 0, 1);
        add_edge(&mut graph, 4, 1, 2);
        graph.start_node = 3;

        let routes = build_processor_dedicated_routes(&graph);

        // Each processor should have route
        assert!(routes.len() <= graph.n);

        // Routes should be valid (non-empty, separator indices in range)
        for (processor_id, route) in &routes {
            assert!(*processor_id < graph.n);
            for &sep_idx in route {
                assert!(sep_idx < graph.m);
            }
        }
    }

    #[test]
    fn test_trace_route_to_start() {
        let mut graph = create_test_graph();
        add_edge(&mut graph, 3, 0, 1);
        graph.start_node = 3;

        let reverse_graph = build_reverse_graph(&graph);
        let route = trace_route_to_start(0, &reverse_graph, graph.start_node, graph.n);

        // Route should contain only separator indices
        for &sep_idx in &route {
            assert!(sep_idx < graph.m);
        }
    }

    #[test]
    fn test_select_processor_specialized_separator_type() {
        let assigned_processors = vec![0, 1];
        let probabilities = vec![
            vec![0.9, 0.1, 0.0], // Type 0: specialized for processor 0
            vec![0.1, 0.9, 0.0], // Type 1: specialized for processor 1
            vec![0.3, 0.3, 0.8], // Type 2: specialized for processor 2
        ];

        let selected_type =
            select_processor_specialized_separator_type(&assigned_processors, &probabilities);

        // Should select type specialized for assigned processors
        assert!(selected_type < probabilities.len());

        // Test with empty assigned processors
        let empty_assigned = vec![];
        let empty_result =
            select_processor_specialized_separator_type(&empty_assigned, &probabilities);
        assert_eq!(empty_result, 0); // Default value
    }

    #[test]
    fn test_generate_optimal_device_assignments() {
        // 3 processors, 3 waste types
        let processor_probabilities = vec![
            vec![0.8, 0.1, 0.1], // Processor 0: high probability for waste type 0
            vec![0.1, 0.9, 0.0], // Processor 1: high probability for waste type 1
            vec![0.1, 0.0, 0.9], // Processor 2: high probability for waste type 2
            // Separator data (unused)
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
        ];

        let assignments = generate_optimal_device_assignments(3, &processor_probabilities);

        assert_eq!(assignments[0], 0); // Waste type 0 -> Processor 0
        assert_eq!(assignments[1], 1); // Waste type 1 -> Processor 1
        assert_eq!(assignments[2], 2); // Waste type 2 -> Processor 2

        // All processors should be uniquely assigned
        let mut used = vec![false; 3];
        for &processor in &assignments {
            assert!(
                !used[processor],
                "Processor {} assigned multiple times",
                processor
            );
            used[processor] = true;
        }
    }

    #[test]
    fn test_generate_optimal_device_assignments_conflict() {
        // Conflict case: two waste types prefer same processor
        let processor_probabilities = vec![
            vec![0.9, 0.8, 0.1], // Processor 0: high probability for waste types 0,1
            vec![0.1, 0.1, 0.7], // Processor 1: high probability for waste type 2
            vec![0.0, 0.1, 0.2], // Processor 2: low probability
        ];

        let assignments = generate_optimal_device_assignments(3, &processor_probabilities);
        assert_eq!(assignments.len(), 3);

        // All processors should be uniquely assigned
        let mut used = vec![false; 3];
        for &processor in &assignments {
            assert!(
                !used[processor],
                "Processor {} assigned multiple times",
                processor
            );
            used[processor] = true;
        }

        // All waste types should be assigned
        assert_eq!(used, vec![true, true, true]);
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
        assert_eq!(graph.probabilities.len(), 2);
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn test_solve_basic() {
        let test_graph = create_test_graph();
        let start_time = std::time::Instant::now();
        let (graph, separator_configs, processor_probabilities) = solve(
            start_time,
            test_graph.n,
            test_graph.m,
            &test_graph.processor_positions,
            &test_graph.separator_positions,
            &test_graph.probabilities,
        );

        let device_assignments =
            generate_optimal_device_assignments(test_graph.n, &processor_probabilities);

        // Device assignment size check
        assert_eq!(device_assignments.len(), test_graph.n);

        // Start node should be within separator range
        assert!(graph.start_node >= test_graph.n && graph.start_node < test_graph.n + test_graph.m);

        // Number of configs should match number of separators
        assert_eq!(separator_configs.len(), test_graph.m);

        // Probability matrix size check
        assert_eq!(processor_probabilities.len(), test_graph.n + test_graph.m);
        for probs in &processor_probabilities {
            assert_eq!(probs.len(), test_graph.n);
        }
    }

    #[test]
    fn test_build_processor_probabilities() {
        let test_graph = create_test_graph();
        let mut graph = test_graph.clone();
        add_edge(&mut graph, 3, 0, 1);

        let configs = vec![
            "-1".to_string(),
            "0 0 1".to_string(), // Separator 1 with type 0, outputs to 0 and 1
        ];

        let probs =
            build_processor_probabilities(graph.n, graph.m, &graph.probabilities, &graph, &configs);

        // Probability matrix size check
        assert_eq!(probs.len(), graph.n + graph.m);
        for prob_row in &probs {
            assert_eq!(prob_row.len(), graph.n);
        }

        // Probability range check (0.0-1.0 range)
        for prob_row in &probs {
            for &prob in prob_row {
                assert!(prob >= 0.0 && prob <= 1.0);
            }
        }
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
