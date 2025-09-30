#!/bin/bash

# Usage: ./bench.sh [number_of_parallel_jobs]
# Example: ./bench.sh 8  (uses 8 parallel jobs)
# Default: 4 parallel jobs

# Default number of parallel jobs
JOBS=${1:-4}

echo "Using $JOBS parallel jobs"

echo "Building binaries..."

# Build the binaries first
cargo build --bin main --bin tester

if [ $? -ne 0 ]; then
    echo "Build failed, exiting..."
    exit 1
fi



# Function to test a single case
test_case() {
    local i=$1
    local input_file="in/${i}.txt"
    local output_file="out_${i}.txt"
    local result_file="result_${i}.txt"
    
    # Check if input file exists
    if [ ! -f "$input_file" ]; then
        echo "Warning: Input file $input_file not found, skipping..." > "$result_file"
        return
    fi
    
    echo "Testing case $i..." > "$result_file"
    
    score_output=$(./target/debug/tester ./target/debug/main < "$input_file" 2>&1 >"$output_file")
    
    # Extract score from output (assumes "Score = XXXX" format)
    score=$(echo "$score_output" | grep -o "Score = [0-9]*" | grep -o "[0-9]*")
    
    if [ -n "$score" ]; then
        echo "SUCCESS:$i:$score" > "$result_file"
    else
        echo "FAILED:$i:Failed to parse score" > "$result_file"
    fi
    
    # Clean up output file
    rm -f "$output_file"
}

# Export the function for xargs
export -f test_case

echo "Running tests in parallel ($JOBS jobs)..."

# Run tests in parallel using xargs
seq -f "%04g" 0 300 | xargs -n 1 -P $JOBS -I {} bash -c 'test_case "$@"' _ {}

# Collect results
total_score=0
test_count=0
failed_count=0
max_score=0
max_case=""
all_scores=()

for result_file in result_*.txt; do
    if [ -f "$result_file" ]; then
        line=$(cat "$result_file")
        if [[ $line == SUCCESS:* ]]; then
            IFS=':' read -r status case_num score <<< "$line"
            total_score=$((total_score + score))
            test_count=$((test_count + 1))
            
            # Store all scores for sorting
            all_scores+=("$case_num:$score")
            
            # Update max score
            if [ "$score" -gt "$max_score" ]; then
                max_score=$score
                max_case=$case_num
            fi
            
            echo "Case $case_num: Score $score"
        elif [[ $line == FAILED:* ]]; then
            failed_count=$((failed_count + 1))
            echo "$line"
        else
            echo "$line"
        fi
        rm -f "$result_file"
    fi
done

echo "=== BENCHMARK RESULTS ==="
if [ $test_count -gt 0 ]; then
    average_score=$((total_score / test_count))
    echo "Average score: $average_score"
    echo "Maximum score: $max_score (case: $max_case)"
    echo "Total tests: $test_count"
    echo "Failed tests: $failed_count"
    
    # Sort scores and show bottom 10
    if [ ${#all_scores[@]} -gt 0 ]; then
        echo ""
        echo "=== BOTTOM 10 CASES ==="
        printf '%s\n' "${all_scores[@]}" | sort -t: -k2 -n | head -10 | while IFS=':' read -r case_num score; do
            echo "Case $case_num: Score $score"
        done
    fi
else
    echo "No successful tests completed"
fi

