#!/bin/bash

for file in tools/in/*.txt; do
  basename=$(basename "$file" .txt)
  echo "Processing $basename..."
  rustc main.rs && ./main < "$file" > "out.txt" && cd tools && cargo run -r --bin vis "in/$basename.txt" "../out.txt" && cd ..
  echo "Completed $basename"
  echo "---"
done