#!/bin/bash

# Create a temporary 1GB file
temp_file=$(mktemp)
dd if=/dev/zero of=$temp_file bs=1M count=4096

# Build the release version of your code
cargo build --release

# Run your code with perf and time
echo "Running your code (ricat):"
# perf stat -e cache-misses,cache-references ../target/release/ricat $temp_file > /dev/null
time ../target/release/ricat $temp_file > /dev/null

# Run the cat command with perf and time
echo "Running cat command:"
# perf stat -e cache-misses,cache-references cat $temp_file > /dev/null
time cat $temp_file > /dev/null

# Run the ricat command with perf and time
echo "Running ricat command:"
# perf stat -e cache-misses,cache-references ricat $temp_file > /dev/null
time ricat $temp_file > /dev/null

# Clean up the temporary file
rm $temp_file
