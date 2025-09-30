# Essential Commands for AtCoder AHC051 Project

## Build Commands
```bash
# Build in debug mode
cargo build --bin main

# Build in release mode  
cargo build --release --bin main

# Build all binaries
cargo build
```

## Running the Solver
```bash
# Run with sample input
./target/debug/main < sample_input.txt > output.txt

# Run with specific test case
./target/debug/main < in/0000.txt > out.txt

# Run in release mode (faster)
./target/release/main < in/0000.txt > out.txt
```

## Visualization and Analysis
```bash
# Visualize solution
cargo run -r --bin vis in/0000.txt out.txt

# Generate test input
cargo run --bin gen

# Extract score from visualization output
cargo run -r --bin vis input.txt output.txt 2>/dev/null | grep -oE "Score = [0-9]+"
```

## Batch Testing
```bash
# Run full test suite
./test.sh

# Test specific range (modify script)
for i in {0..9}; do
  padded_i=$(printf "%04d" $i)
  ./target/debug/main < "in/${padded_i}.txt" > "out.txt"
done
```

## Development Tools
```bash
# Format code (if rustfmt is available)
cargo fmt

# Run tests
cargo test

# Check for common mistakes
cargo clippy

# Clean build artifacts
cargo clean
```

## System Commands (Darwin/macOS)
```bash
# List files
ls -la
find . -name "*.rs"

# Search in files  
grep -r "pattern" src/

# Monitor real-time performance
time ./target/debug/main < input.txt

# Check dependencies
cargo tree
```

## Git Operations
```bash
# Standard git workflow
git add .
git commit -m "message"
git push

# Check status
git status
git log --oneline
```