#!/bin/bash

# Comprehensive Compression Benchmark Comparison
# Compares parallel-mengene with popular compression tools

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Test files
TEST_FILES=(
    "test_files/test_10mb.bin"
    "test_files/test_10mb_random.bin" 
    "test_files/test_10mb_repetitive.txt"
    "test_files/test_100mb.bin"
    "test_files/test_1000mb.bin"
)

# Results directory
RESULTS_DIR="benchmark_results"
mkdir -p "$RESULTS_DIR"

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get file size in bytes
get_file_size() {
    stat -c%s "$1" 2>/dev/null || echo "0"
}

# Function to measure compression time and size
measure_compression() {
    local tool="$1"
    local input_file="$2"
    local output_file="$3"
    local command="$4"
    
    local input_size=$(get_file_size "$input_file")
    local start_time=$(date +%s.%N)
    
    eval "$command" 2>/dev/null
    
    local end_time=$(date +%s.%N)
    local duration=$(echo "$end_time - $start_time" | bc -l)
    local output_size=$(get_file_size "$output_file")
    local compression_ratio=$(echo "scale=4; $output_size * 100 / $input_size" | bc -l)
    local speed=$(echo "scale=2; $input_size / $duration / 1024 / 1024" | bc -l)
    
    echo "$duration,$output_size,$compression_ratio,$speed"
}

# Function to measure decompression time
measure_decompression() {
    local tool="$1"
    local compressed_file="$2"
    local output_file="$3"
    local command="$4"
    
    local start_time=$(date +%s.%N)
    
    eval "$command" 2>/dev/null
    
    local end_time=$(date +%s.%N)
    local duration=$(echo "$end_time - $start_time" | bc -l)
    local output_size=$(get_file_size "$output_file")
    local speed=$(echo "scale=2; $output_size / $duration / 1024 / 1024" | bc -l)
    
    echo "$duration,$speed"
}

echo -e "${BLUE}=== COMPRESSION TOOL BENCHMARK COMPARISON ===${NC}"
echo -e "${BLUE}Testing against popular compression tools${NC}"
echo

# Check if required tools are available
echo -e "${YELLOW}Checking available compression tools...${NC}"

TOOLS=()
if command_exists gzip; then TOOLS+=("gzip"); fi
if command_exists bzip2; then TOOLS+=("bzip2"); fi
if command_exists xz; then TOOLS+=("xz"); fi
if command_exists 7z; then TOOLS+=("7z"); fi
if command_exists zstd; then TOOLS+=("zstd"); fi
if command_exists lz4; then TOOLS+=("lz4"); fi
if command_exists zip; then TOOLS+=("zip"); fi

# Add parallel-mengene
if [ -f "./target/release/parallel-mengene" ]; then
    TOOLS+=("parallel-mengene")
else
    echo -e "${RED}Warning: parallel-mengene binary not found. Building...${NC}"
    cargo build --release
    TOOLS+=("parallel-mengene")
fi

echo -e "${GREEN}Available tools: ${TOOLS[*]}${NC}"
echo

# Create results CSV
CSV_FILE="$RESULTS_DIR/benchmark_results.csv"
echo "Tool,File,Input_Size,Compression_Time,Compressed_Size,Compression_Ratio,Compression_Speed_MBps,Decompression_Time,Decompression_Speed_MBps" > "$CSV_FILE"

# Run benchmarks for each tool and file
for tool in "${TOOLS[@]}"; do
    echo -e "${CYAN}Testing $tool...${NC}"
    
    for test_file in "${TEST_FILES[@]}"; do
        if [ ! -f "$test_file" ]; then
            echo -e "${RED}Skipping $test_file (not found)${NC}"
            continue
        fi
        
        filename=$(basename "$test_file")
        input_size=$(get_file_size "$test_file")
        compressed_file="$RESULTS_DIR/${filename}.${tool}"
        decompressed_file="$RESULTS_DIR/${filename}.${tool}.decompressed"
        
        echo -e "  ${YELLOW}Testing $filename ($(numfmt --to=iec $input_size))${NC}"
        
        # Define compression commands for each tool
        case "$tool" in
            "gzip")
                comp_cmd="gzip -c '$test_file' > '$compressed_file'"
                decomp_cmd="gunzip -c '$compressed_file' > '$decompressed_file'"
                ;;
            "bzip2")
                comp_cmd="bzip2 -c '$test_file' > '$compressed_file'"
                decomp_cmd="bunzip2 -c '$compressed_file' > '$decompressed_file'"
                ;;
            "xz")
                comp_cmd="xz -c '$test_file' > '$compressed_file'"
                decomp_cmd="unxz -c '$compressed_file' > '$decompressed_file'"
                ;;
            "7z")
                comp_cmd="7z a -mx=9 '$compressed_file' '$test_file' >/dev/null 2>&1"
                decomp_cmd="7z x -so '$compressed_file' > '$decompressed_file' 2>/dev/null"
                ;;
            "zstd")
                comp_cmd="zstd -c -19 '$test_file' > '$compressed_file'"
                decomp_cmd="zstd -d -c '$compressed_file' > '$decompressed_file'"
                ;;
            "lz4")
                comp_cmd="lz4 -c '$test_file' > '$compressed_file'"
                decomp_cmd="lz4 -d -c '$compressed_file' > '$decompressed_file'"
                ;;
            "zip")
                comp_cmd="zip -9 -q '$compressed_file' '$test_file'"
                decomp_cmd="unzip -p '$compressed_file' > '$decompressed_file'"
                ;;
            "parallel-mengene")
                comp_cmd="./target/release/parallel-mengene compress '$test_file' '$compressed_file' >/dev/null"
                decomp_cmd="./target/release/parallel-mengene decompress '$compressed_file' '$decompressed_file' >/dev/null"
                ;;
        esac
        
        # Measure compression
        comp_result=$(measure_compression "$tool" "$test_file" "$compressed_file" "$comp_cmd")
        IFS=',' read -r comp_time comp_size comp_ratio comp_speed <<< "$comp_result"
        
        # Measure decompression
        decomp_result=$(measure_decompression "$tool" "$compressed_file" "$decompressed_file" "$decomp_cmd")
        IFS=',' read -r decomp_time decomp_speed <<< "$decomp_result"
        
        # Verify decompression integrity
        if [ -f "$decompressed_file" ]; then
            if cmp -s "$test_file" "$decompressed_file"; then
                echo -e "    ${GREEN}✓ Integrity verified${NC}"
            else
                echo -e "    ${RED}✗ Integrity check failed${NC}"
            fi
        fi
        
        # Write to CSV
        echo "$tool,$filename,$input_size,$comp_time,$comp_size,$comp_ratio,$comp_speed,$decomp_time,$decomp_speed" >> "$CSV_FILE"
        
        # Clean up
        rm -f "$compressed_file" "$decompressed_file"
        
        echo -e "    ${GREEN}Compression: ${comp_speed} MB/s, Ratio: ${comp_ratio}%${NC}"
        echo -e "    ${GREEN}Decompression: ${decomp_speed} MB/s${NC}"
    done
    echo
done

echo -e "${BLUE}=== BENCHMARK COMPLETE ===${NC}"
echo -e "${GREEN}Results saved to: $CSV_FILE${NC}"

# Generate summary report
echo -e "${YELLOW}Generating summary report...${NC}"
python3 << 'EOF'
import csv
import sys
from collections import defaultdict

# Read CSV data
data = []
with open('benchmark_results/benchmark_results.csv', 'r') as f:
    reader = csv.DictReader(f)
    for row in reader:
        data.append(row)

# Group by tool
tool_stats = defaultdict(list)
for row in data:
    tool_stats[row['Tool']].append(row)

# Calculate averages
print("\n=== COMPRESSION PERFORMANCE SUMMARY ===")
print(f"{'Tool':<20} {'Avg Speed (MB/s)':<18} {'Avg Ratio (%)':<15} {'Files Tested':<12}")
print("-" * 70)

for tool, results in tool_stats.items():
    if not results:
        continue
    
    # Filter out results with empty values
    valid_results = [r for r in results if r['Compression_Speed_MBps'] and r['Compression_Ratio']]
    if not valid_results:
        continue
    
    avg_speed = sum(float(r['Compression_Speed_MBps']) for r in valid_results) / len(valid_results)
    avg_ratio = sum(float(r['Compression_Ratio']) for r in valid_results) / len(valid_results)
    
    print(f"{tool:<20} {avg_speed:<18.2f} {avg_ratio:<15.2f} {len(results):<12}")

print("\n=== DETAILED RESULTS ===")
for tool, results in tool_stats.items():
    if not results:
        continue
    
    print(f"\n{tool.upper()}:")
    print(f"{'File':<25} {'Size (MB)':<10} {'Speed (MB/s)':<12} {'Ratio (%)':<10} {'Decomp (MB/s)':<12}")
    print("-" * 80)
    
    for result in results:
        file_size = float(result['Input_Size']) / (1024 * 1024)
        
        # Handle empty values
        speed = float(result['Compression_Speed_MBps']) if result['Compression_Speed_MBps'] else 0
        ratio = float(result['Compression_Ratio']) if result['Compression_Ratio'] else 0
        decomp_speed = float(result['Decompression_Speed_MBps']) if result['Decompression_Speed_MBps'] else 0
        
        print(f"{result['File']:<25} {file_size:<10.2f} {speed:<12.2f} {ratio:<10.2f} {decomp_speed:<12.2f}")

EOF

echo -e "\n${GREEN}Benchmark comparison complete!${NC}"
echo -e "${BLUE}Check $CSV_FILE for detailed results${NC}"
