#!/bin/bash

# Check if the memory size argument is provided
if [ $# -eq 0 ]; then
    echo "Please provide the memory size (in MB) as a command-line argument."
    echo "Usage: $0 <memory_size>"
    exit 1
fi

memory_size=$1

# Number of iterations
iterations=10

# Create a temporary file
times_file=times.txt

# Initialize the times file
echo "Iteration,Your Code (ricat),cat,ricat" > $times_file

# Build the release version of your code
cargo build --release

for ((i=1; i<=iterations; i++)); do
    echo "Iteration $i"

    # Create a temporary file with the specified memory size
    temp_file=$(mktemp)
    dd if=/dev/zero of=$temp_file bs=1M count=$memory_size

    # Run your code and store the real time
    echo -n "$i," >> $times_file
    { time ../target/release/ricat $temp_file > /dev/null; } 2>&1 | grep real | awk '{print $2}' | tr -d '\n' >> $times_file
    echo -n "," >> $times_file

    # Run the cat command and store the real time
    { time cat $temp_file > /dev/null; } 2>&1 | grep real | awk '{print $2}' | tr -d '\n' >> $times_file
    echo -n "," >> $times_file

    # Run the ricat command and store the real time
    { time ricat $temp_file > /dev/null; } 2>&1 | grep real | awk '{print $2}' | tr -d '\n' >> $times_file
    echo "" >> $times_file

    # Clean up the temporary file
    rm $temp_file
done

# Run the Python script to plot the average times
python3 - <<EOF
import matplotlib.pyplot as plt
import csv

# Read the times from the file
with open('$times_file', 'r') as file:
    csv_reader = csv.DictReader(file)
    data = list(csv_reader)

# Calculate the average times for each executable
commands = ['Your Code (ricat)', 'cat', 'ricat']
avg_times = [0] * len(commands)

for row in data:
    for i, command in enumerate(commands):
        time_str = row[command]
        minutes, seconds = time_str.split('m')
        minutes = float(minutes)
        seconds = float(seconds[:-1])  # Remove the trailing 's'
        time_in_seconds = minutes * 60 + seconds
        avg_times[i] += time_in_seconds

avg_times = [time / len(data) for time in avg_times]

# Create a figure and axis
fig, ax = plt.subplots()

# Plot the bar graph
bars = ax.bar(commands, avg_times)

# Add labels and title
ax.set_xlabel('Commands')
ax.set_ylabel('Average Real Time (seconds)')
ax.set_title(f'Average Real Time Comparison\\nTested with {$memory_size}MB Memory')

# Add value labels on top of each bar
for bar in bars:
    height = bar.get_height()
    ax.annotate(f'{height:.3f}s',
                xy=(bar.get_x() + bar.get_width() / 2, height),
                xytext=(0, 3),  # 3 points vertical offset
                textcoords="offset points",
                ha='center', va='bottom')

# Adjust the layout and display the plot
plt.tight_layout()
plt.show()
EOF


# Remove the temporary file
rm $times_file
