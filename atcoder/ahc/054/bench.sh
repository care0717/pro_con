#!/bin/bash


total_score=0
test_count=0
failed_count=0
min_score=999999999999999999
max_score=0
min_case=""
max_case=""

echo "Building binaries..."

# Build the binaries first
cargo build --bin main --bin tester

if [ $? -ne 0 ]; then
    echo "Build failed, exiting..."
    exit 1
fi


for i in $(seq -f "%04g" 0 100); do
    input_file="in/${i}.txt"
    output_file="out_${i}.txt"
    
    # Check if input file exists
    if [ ! -f "$input_file" ]; then
        echo "Warning: Input file $input_file not found, skipping..."
        continue
    fi
    
    echo -n "Testing case $i... "
    
    score_output=$(./target/debug/tester ./target/debug/main < "$input_file" 2>&1 >"$output_file")
    
    # Extract score from output (assumes "Score = XXXX" format)
    score=$(echo "$score_output" | grep -o "Score = [0-9]*" | grep -o "[0-9]*")
    
    if [ -n "$score" ]; then
        total_score=$((total_score + score))
        test_count=$((test_count + 1))
        
        # Update min and max scores
        if [ "$score" -lt "$min_score" ]; then
            min_score=$score
            min_case=$i
        fi
        if [ "$score" -gt "$max_score" ]; then
            max_score=$score
            max_case=$i
        fi
        
        echo "Score: $score"
    else
        echo "Failed to parse score"
        failed_count=$((failed_count + 1))
    fi

    
    # Clean up output file
    rm -f "$output_file"
done

echo "=== BENCHMARK RESULTS ==="
if [ $test_count -gt 0 ]; then
    average_score=$((total_score / test_count))
    echo "Average score: $average_score"
    echo "Minimum score: $min_score (case: $min_case)"
    echo "Maximum score: $max_score (case: $max_case)"
else
    echo "No successful tests completed"
fi

