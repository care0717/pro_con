# Task Completion Workflow for AtCoder AHC051

## When a Coding Task is Completed

### 1. Build and Verify Compilation
```bash
# Always build after making changes
cargo build --bin main

# Fix any compilation errors before proceeding
# Look for warnings but they're usually acceptable in competitive programming
```

### 2. Test with Sample Cases
```bash
# Test with sample input
./target/debug/main < sample_input.txt > test_output.txt

# Verify output format is correct
cat test_output.txt

# Test with a few specific cases
./target/debug/main < in/0000.txt > out.txt
./target/debug/main < in/0001.txt > out.txt
```

### 3. Validate Solution Format
The output should follow this format:
```
d₀ d₁ ... d_{N-1}    # Device assignments (one line)
s                     # Start node (one line)  
config₀              # Separator configurations (M lines)
config₁              # Either "-1" or "k v₁ v₂"
...
config_{M-1}
```

### 4. Performance Testing
```bash
# Quick performance check
time ./target/debug/main < in/0000.txt > /dev/null

# Should complete well within 2 seconds
# If too slow, consider release build
time ./target/release/main < in/0000.txt > /dev/null
```

### 5. Score Evaluation
```bash
# Check score with visualizer
cargo run -r --bin vis in/0000.txt out.txt

# Extract numerical score
score=$(cargo run -r --bin vis in/0000.txt out.txt 2>/dev/null | grep -oE "Score = [0-9]+" | grep -oE "[0-9]+")
echo "Score: $score"
```

### 6. Batch Testing (Optional but Recommended)
```bash
# Run on multiple test cases to ensure stability
./test.sh

# Or test a smaller subset for quick validation
for i in {0..4}; do
  padded_i=$(printf "%04d" $i)
  ./target/debug/main < "in/${padded_i}.txt" > "test_out.txt"
  echo "Tested case $padded_i"
done
```

### 7. Code Quality Checks (Optional)
```bash
# Format code (if available)
cargo fmt

# Run clippy for suggestions (if available)
cargo clippy

# Run unit tests
cargo test
```

## Performance Benchmarks

### Expected Behavior
- **Runtime**: Should complete in under 1 second for most cases
- **Memory**: Should not exceed 100MB usage typically
- **Score**: Lower scores are better (minimize waste routing errors)

### Red Flags
- **Timeout**: Takes longer than 2 seconds
- **Invalid Output**: Format doesn't match requirements  
- **Crashes**: Panics or segmentation faults
- **Very High Scores**: Indicates poor routing efficiency

## Deployment Checklist

Before submitting or considering the implementation complete:

- [ ] Code compiles without errors
- [ ] Passes sample test cases
- [ ] Output format is valid
- [ ] Runtime is acceptable (< 2 seconds)
- [ ] Score is reasonable compared to baseline
- [ ] No obvious logic errors in key algorithms
- [ ] Edge cases handled (empty graphs, single nodes, etc.)

## Debugging Tips

### Common Issues
1. **Graph Cycles**: Check `has_cycle()` function
2. **Invalid Configs**: Verify separator configuration format
3. **Score Calculation**: Debug probability calculations step by step
4. **Memory Issues**: Monitor Vec/HashMap growth in long loops

### Debug Output
Add debug prints as needed:
```rust
eprint!("Debug info: {}", value);  // Goes to stderr, doesn't affect solution output
```