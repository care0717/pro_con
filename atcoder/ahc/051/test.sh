#!/bin/bash

# Function to process a single file
process_file() {
  i=$1
  padded_i=$(printf "%04d" $i)
  file="in/${padded_i}.txt"
  
  if [ -f "$file" ]; then
    # echo "Processing $padded_i..."
    output_file="out_${padded_i}.txt"
    ./target/debug/main < "$file" > "$output_file" 2>&1
    
    if [ $? -eq 0 ]; then
      score=$(cargo run -r --bin vis "$file" "$output_file" 2>/dev/null | grep -oE "Score = [0-9]+" | grep -oE "[0-9]+")
      
      if [ ! -z "$score" ]; then
        echo "$score" > "score_${padded_i}.txt"
        echo "File $padded_i: Score = $score"
      else
        echo "" > "score_${padded_i}.txt" # Write empty on failure
        echo "File $padded_i: Failed to extract score"
      fi
    else
      echo "" > "score_${padded_i}.txt" # Write empty on failure
      echo "File $padded_i: Execution failed"
    fi
  else
    echo "File $padded_i: Not found"
  fi
}

export -f process_file

cargo build --bin main

# Generate file numbers and pipe to xargs for parallel processing
seq 0 49 | xargs -I {} -P 1 bash -c 'process_file "{}"'

# Aggregate scores
total_score=0
count=0
for i in {0..49}; do
  padded_i=$(printf "%04d" $i)
  score_file="score_${padded_i}.txt"
  if [ -f "$score_file" ]; then
    score=$(cat "$score_file")
    if [ ! -z "$score" ]; then
        total_score=$((total_score + score))
        count=$((count + 1))
    fi
    rm "$score_file" # Clean up score file
    rm "out_${padded_i}.txt" # Clean up output file
  fi
done

if [ $count -gt 0 ]; then
  average_score=$(echo "scale=2; $total_score / $count" | bc)
  echo "============================="
  echo "Total files processed: $count"
  echo "Average score: $average_score"
else
  echo "No scores collected"
fi
