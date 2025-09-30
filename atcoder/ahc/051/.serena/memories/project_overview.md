# AtCoder Heuristic Contest 051 Project Overview

## Purpose
This is a competitive programming project for AtCoder Heuristic Contest 051, focused on solving the "Probabilistic Waste Sorting" problem. The goal is to design an optimal network of probabilistic separators to route different types of waste to their corresponding processing units with maximum efficiency.

## Problem Description
- **Input**: N types of waste, M separator locations, K separator types with probabilistic routing
- **Goal**: Minimize the score by maximizing the probability that each waste type reaches its correct processing unit
- **Constraints**: 5≤N≤20, 10N≤M≤50N, N≤K≤4N
- **Time Limit**: 2 seconds
- **Score Formula**: round(10⁹ × (1/N) × Σ(1-q_i)) where q_i is the probability that waste type i reaches its correct processor

## Tech Stack
- **Language**: Rust (Edition 2021)
- **Build System**: Cargo
- **Key Dependencies**:
  - `rand` (0.8.5) - Random number generation for optimization algorithms
  - `itertools` (0.11.0) - Extended iterator functionality
  - `proconio` (0.4.5) - Input parsing utilities
  - `svg` (0.17.0) - Visualization support
  - `clap` (4.3.19) - Command line argument parsing
  - `serde`/`serde_json` - Serialization support

## Project Structure
```
├── src/
│   ├── bin/
│   │   ├── main.rs      # Main solver implementation
│   │   ├── vis.rs       # Visualization tool
│   │   └── gen.rs       # Input generation tool
│   └── lib.rs           # Shared library code
├── in/                  # Test input files (0000.txt - 0099.txt)
├── Cargo.toml           # Project configuration
├── test.sh              # Batch testing script
├── README.md            # Problem description
└── out_best.txt         # Best known solution output
```

## Key Algorithms Implemented
1. **Network Construction**: Greedy algorithm with edge intersection handling
2. **Hill Climbing Optimization**: Random separator type changes with batch processing
3. **Route-Based Specialization**: Processor-dedicated routes for separator assignment
4. **Graph Analysis**: Topological sorting, reachability analysis
5. **Scoring**: Optimal device assignment and probability calculation