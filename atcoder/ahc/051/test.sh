#!/bin/bash

total_score=0
count=0
rustc src/bin/main.rs -o main
for i in {0..49}; do
  padded_i=$(printf "%04d" $i)
  file="in/${padded_i}.txt"
  if [ -f "$file" ]; then
    echo "Processing $padded_i..."
    ./main < "$file" > "out.txt" 2>&1
    
    if [ $? -eq 0 ]; then
      score=$(cargo run -r --bin vis "in/${padded_i}.txt" "out.txt" 2>/dev/null | grep -oE "Score = [0-9]+" | grep -oE "[0-9]+")
      
      if [ ! -z "$score" ]; then
        total_score=$((total_score + score))
        count=$((count + 1))
        echo "File $padded_i: Score = $score"
      else
        echo "File $padded_i: Failed to extract score"
      fi
    else
      echo "File $padded_i: Compilation/execution failed"
    fi
  else
    echo "File $padded_i: Not found"
  fi
  echo "---"
done

if [ $count -gt 0 ]; then
  average_score=$(echo "scale=2; $total_score / $count" | bc)
  echo "============================="
  echo "Total files processed: $count"
  echo "Total score: $total_score"
  echo "Average score: $average_score"
else
  echo "No scores collected"
fi