# Algorithm Architecture for AtCoder AHC051 Solver

## Core Algorithm Pipeline

### 1. Input Processing
- Parse N (waste types), M (separator locations), K (separator types)
- Read processor and separator coordinates
- Read probability matrix for each separator type

### 2. Graph Construction (`build_network_greedy`)
- **Objective**: Build a DAG (Directed Acyclic Graph) connecting waste input to processors
- **Method**: Distance-based greedy algorithm with intersection handling
- **Key Features**:
  - Starts from nearest separator to entrance (0, 5000)
  - Avoids edge intersections using geometric algorithms
  - Prevents cycles through DFS cycle detection
  - Removes disconnected separators

### 3. Route-Based Specialization (`generate_configs_from_graph`)
**New Implementation**: Processor-dedicated routing approach
- **Route Tracing**: Each processor traces back to entrance via `trace_route_to_start`
- **Specialization**: Separators assigned to specific processors they serve
- **Type Selection**: `select_processor_specialized_separator_type` chooses optimal separator types based on:
  - High probability for assigned processors
  - Low probability for non-assigned processors  
  - Specialization bonus for fewer assigned processors

### 4. Hill Climbing Optimization (`solve`)
**Batch Optimization Strategy**:
- Select 10 random separators simultaneously
- Change their types to different random types
- Calculate score for entire batch
- Accept if improvement found
- Continue for 1 second time limit

### 5. Score Calculation
- **Probability Propagation**: `build_processor_probabilities` uses topological sort
- **Optimal Assignment**: `generate_optimal_device_assignments` maximizes probability matching
- **Final Score**: `calculate_score` computes contest score formula

## Key Data Structures

### Graph Representation
```rust
struct Graph {
    n: usize,                     // Number of waste types/processors  
    m: usize,                     // Number of separator locations
    processor_positions: Vec<Point>,
    separator_positions: Vec<Point>, 
    probabilities: Vec<Vec<f64>>,    // [separator_type][waste_type] -> probability
    edges: HashMap<NodeId, Out>,     // Node -> (out1, out2) connections
    start_node: NodeId,              // Entry point node
}
```

### Optimization State
- **Probability Matrix**: `processor_probs[processor][waste_type]` = probability
- **Device Assignments**: `assignments[waste_type]` = processor_id  
- **Configurations**: Vector of separator config strings

## Geometric Algorithms

### Edge Intersection Detection
- **Purpose**: Prevent belt conveyor crossings (problem constraint)
- **Method**: Line segment intersection using cross product
- **Implementation**: `segments_intersect`, `edge_intersects`

### Distance Calculations
- **Purpose**: Greedy nearest-neighbor selection
- **Method**: Euclidean distance in 2D plane
- **Optimization**: Pre-sorted distance arrays per separator

## Optimization Strategies

### Time Management
- 1.5 second total time limit
- 1 second dedicated to hill climbing
- Remaining time for initial construction

### Search Space
- **Separator Types**: K choices per separator (typically 20-80 types)
- **Connections**: Fixed by graph construction phase
- **Device Assignments**: Optimized via Hungarian-like algorithm

### Convergence Strategy
- **Batch Size**: 10 separators changed simultaneously
- **Accept Criteria**: Any improvement (greedy hill climbing)
- **Restart**: No restarts, single trajectory optimization

## Performance Characteristics

### Time Complexity
- **Graph Construction**: O(m² × |E|) where |E| = edge count
- **Route Specialization**: O(n × path_length + m × k × n)
- **Hill Climbing**: O(iterations × score_calculation)
- **Score Calculation**: O(topological_sort + assignment_optimization)

### Memory Usage
- **Graph Storage**: O(n + m + edges)
- **Probability Matrices**: O((n + m) × n + k × n)
- **Optimization State**: O(m × config_string_length)

## Design Patterns

### Probabilistic Modeling
- Forward probability propagation through network
- Topological ordering for efficient computation
- Separator types modeled as probability distributions

### Geometric Constraints
- 2D coordinate system with distance metrics
- Line intersection algorithms for constraint satisfaction
- Spatial optimization for separator placement

### Metaheuristic Framework
- Hill climbing with batch modifications
- Time-bounded optimization loops
- Greedy constructive heuristics followed by local search