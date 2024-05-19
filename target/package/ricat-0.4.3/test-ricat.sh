#!/bin/bash

# Function to compare output with expected result
compare_output() {
    local expected="$1"
    local actual="$2"
    local feature="$3"

    if [[ "$actual" == "$expected" ]]; then
        echo "Test passed: $feature"
    else
        echo "Test failed: $feature"
        echo "Expected:"
        echo "$expected"
        echo "Actual:"
        echo "$actual"
    fi
}

# Create sample.txt file
create_sample_file() {
    cat <<EOF > sample.txt
Line 1
Line 2
Line 3
EOF
}

# Test case for line numbering feature
test_line_numbering() {
    expected=$(printf "1 Line 1\n2 Line 2\n3 Line 3\n")
    actual=$(cargo r -- -n sample.txt)
    compare_output "$expected" "$actual" "Line Numbering"
}

# Test case for dollar sign at the end of each line
test_dollar_sign() {
    expected=$(printf "Line 1$\nLine 2$\nLine 3$\n")
    actual=$(cargo r -- -d sample.txt)
    compare_output "$expected" "$actual" "Dollar Sign at End"
}

# Test case for replacing tab spaces with ^I
test_replace_tabs() {
    printf "Line 1\tLine 2\tLine 3\n" > sample_tabs.txt
    expected=$(printf "Line 1^ILine 2^ILine 3\n")
    actual=$(cargo r -- -t sample_tabs.txt)
    compare_output "$expected" "$actual" "Replace Tab Spaces"
    rm sample_tabs.txt
}

# Test case for compressing empty lines
test_compress_empty_lines() {
    printf "Line 1\n\n\nLine 2\n\nLine 3\n" > sample_empty_lines.txt
    expected=$(printf "Line 1\n\nLine 2\n\nLine 3\n")
    actual=$(cargo r -- -s sample_empty_lines.txt)
    compare_output "$expected" "$actual" "Compress Empty Lines"
    rm sample_empty_lines.txt
}

# Test case for searching lines containing a specific text
test_search_text() {
    expected=$(printf "Line 2\n")
    actual=$(cargo r -- --search --text "Line 2" sample.txt)
    compare_output "$expected" "$actual" "Search Text"
}

# Test case for case-insensitive search
test_case_insensitive_search() {
    expected=$(printf "Line 2\n")
    actual=$(cargo r -- --search --text "line 2" -i sample.txt)
    compare_output "$expected" "$actual" "Case-Insensitive Search"
}

# Test case for base64 encoding
test_base64_encoding() {
    expected=$(printf "TGluZSAx\nTGluZSAy\nTGluZSAz\n")
    actual=$(cargo r -- --encode-base64 sample.txt)
    compare_output "$expected" "$actual" "Base64 Encoding"
}

# Test case for base64 decoding
test_base64_decoding() {
    printf "TGluZSAx\nTGluZSAy\nTGluZSAz\n" > sample_base64.txt
    expected=$(printf "Line 1\nLine 2\nLine 3\n")
    actual=$(cargo r -- --decode-base64 sample_base64.txt)
    compare_output "$expected" "$actual" "Base64 Decoding"
    rm sample_base64.txt
}

# Test case for line numbering feature (command line input)
test_line_numbering_stdin() {
    expected=$(printf "1 Line 1\n2 Line 2\n3 Line 3\n")
    actual=$(printf "Line 1\nLine 2\nLine 3\n" | cargo r -- -n)
    compare_output "$expected" "$actual" "Line Numbering (stdin)"
}

# Test case for dollar sign at the end of each line (command line input)
test_dollar_sign_stdin() {
    expected=$(printf "Line 1$\nLine 2$\nLine 3$\n")
    actual=$(printf "Line 1\nLine 2\nLine 3\n" | cargo r -- -d)
    compare_output "$expected" "$actual" "Dollar Sign at End (stdin)"
}

# Test case for replacing tab spaces with ^I (command line input)
test_replace_tabs_stdin() {
    expected=$(printf "Line 1^ILine 2^ILine 3\n")
    actual=$(printf "Line 1\tLine 2\tLine 3\n" | cargo r -- -t)
    compare_output "$expected" "$actual" "Replace Tab Spaces (stdin)"
}

# Test case for compressing empty lines (command line input)
test_compress_empty_lines_stdin() {
    expected=$(printf "Line 1\n\nLine 2\n\nLine 3\n")
    actual=$(printf "Line 1\n\n\nLine 2\n\nLine 3\n" | cargo r -- -s)
    compare_output "$expected" "$actual" "Compress Empty Lines (stdin)"
}

# Test case for searching lines containing a specific text (command line input)
test_search_text_stdin() {
    expected=$(printf "Line 2\n")
    actual=$(printf "Line 1\nLine 2\nLine 3\n" | cargo r -- --search --text "Line 2")
    compare_output "$expected" "$actual" "Search Text (stdin)"
}

# Test case for case-insensitive search (command line input)
test_case_insensitive_search_stdin() {
    expected=$(printf "Line 2\n")
    actual=$(printf "Line 1\nLine 2\nLine 3\n" | cargo r -- --search --text "line 2" -i)
    compare_output "$expected" "$actual" "Case-Insensitive Search (stdin)"
}

# Test case for base64 encoding (command line input)
test_base64_encoding_stdin() {
    expected=$(printf "TGluZSAx\nTGluZSAy\nTGluZSAz\n")
    actual=$(printf "Line 1\nLine 2\nLine 3\n" | cargo r -- --encode-base64)
    compare_output "$expected" "$actual" "Base64 Encoding (stdin)"
}

# Test case for base64 decoding (command line input)
test_base64_decoding_stdin() {
    expected=$(printf "Line 1\nLine 2\nLine 3\n")
    actual=$(printf "TGluZSAx\nTGluZSAy\nTGluZSAz\n" | cargo r -- --decode-base64)
    compare_output "$expected" "$actual" "Base64 Decoding (stdin)"
}

# create sample file
create_sample_file

# Run all test cases
test_line_numbering
test_dollar_sign
test_replace_tabs
test_compress_empty_lines
test_search_text
test_case_insensitive_search
test_base64_encoding
test_base64_decoding

# command line input cases
test_line_numbering_stdin
test_dollar_sign_stdin
test_replace_tabs_stdin
test_compress_empty_lines_stdin
test_search_text_stdin
test_case_insensitive_search_stdin
test_base64_encoding_stdin
test_base64_decoding_stdin


# clean up
rm sample.txt
