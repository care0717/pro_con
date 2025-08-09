# Code Style and Conventions for AtCoder AHC051 Project

## Naming Conventions

### Variables and Functions
- **snake_case** for variables and function names
  - `processor_positions`, `separator_configs`, `build_network_greedy`
- **SCREAMING_SNAKE_CASE** for constants
  - `BATCH_SIZE`, `ENTRANCE_NODE`

### Types and Structs
- **PascalCase** for struct and enum names
  - `Graph`, `Point`, `WeightedReachability`, `OutType`
- **PascalCase** for enum variants
  - `Out1`, `Out2`

### Type Aliases
- **PascalCase** for type aliases
  - `type NodeId = usize`

## Code Organization

### Macros
- Custom input parsing macros at the top of file:
  - `mat!` - Matrix/vector creation macro
  - `input!` - Input parsing macro
  - `input_inner!` - Internal input parsing helper
  - `read_value!` - Value parsing helper

### Function Organization
- **Helper functions before main functions**
- **Related functions grouped together**
- **Test modules at the end with `#[cfg(test)]`**

### Comments
- **Function-level comments** explain algorithm complexity: `O(m^2 * |E|)`
- **Inline comments** for complex logic explanations
- **Japanese comments** used for problem-specific explanations
- **No docstrings** - this is competitive programming, not library code

## Performance Patterns

### Memory Management
- Extensive use of `Vec` and `HashMap` for dynamic collections
- `clone()` used freely for algorithm iterations (performance over memory)
- No explicit memory management needed (Rust handles it)

### Optimization Settings
```toml
[profile.dev]
overflow-checks = false

[profile.release]  
debug = true
```

### Algorithm Patterns
- **Time-bounded optimization**: `while start_time.elapsed().as_millis() < 1000`
- **Batch processing**: Process multiple changes simultaneously
- **Early returns**: Skip invalid configurations quickly

## Error Handling
- **Competitive programming style**: Use `unwrap()` and `expect()` liberally
- **No Result/Option propagation**: Fail fast with panics is acceptable
- **Validation**: Minimal input validation since contest inputs are guaranteed valid

## Data Structures

### Common Patterns
```rust
// Graph representation
let mut edges: HashMap<NodeId, Out> = HashMap::new();

// Matrix initialization  
let mut matrix = mat![0.0; rows; cols];

// Probability calculations
let mut processor_probs: Vec<Vec<f64>> = vec![vec![0.0; n]; m];
```

### Iteration Patterns
```rust
// Enumerate with indices
for (i, item) in collection.iter().enumerate() { }

// Filtering and mapping
let filtered: Vec<_> = items.iter().filter(|x| condition).collect();
```

## Testing
- Unit tests in `#[cfg(test)]` modules
- Helper functions for creating test data
- Integration tests via shell script (`test.sh`)
- No formal test coverage requirements