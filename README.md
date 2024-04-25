# ricat: A Rust-Based `cat` Command Implementation

This project is a Rust-based reimagining of the classic Unix `cat` command, drawing inspiration from its original implementation in GNU Core Utilities. It demonstrates the power and flexibility of Rust for system utilities development.

## About `ricat`
`ricat` is designed with flexibility in mind, making it a valuable tool in the command line toolkit. One of its powerful features is the ability to seamlessly integrate with the pipe `|` operator, allowing users to pipe the output of any command directly into ricat for further processing. This means you can easily use ricat to view, modify, or extend the output of other commands in real-time. Whether it's adding line numbers, appending symbols to each line, or leveraging any of ricat's other features, integrating ricat into your command line workflows can significantly enhance your productivity and command line capabilities.


## Installation

Ensure you have Rust and Cargo installed on your system. If you don't have Rust installed, you can install it from [the official site](https://www.rust-lang.org/tools/install).

You can install `ricat` directly from crates.io by running:

```bash
cargo install ricat
```
This command installs the `ricat` binary, making it available for use in your terminal

## Update existing installation

If you have `ricat` already installed, you can update it by force installing the newer version over the older version:

```bash
cargo install ricat --force
```

## Features

- **Line Numbering**: Add line numbers to the output with the `-n` flag.
- **Dollar Sign Appending**: Append a `$` sign at the end of each line using the `-d` flag.
- **Tab Space Replacement**: Replace tab spaces with `^I` using the `-t` flag.
- **Empty Line Compression**: Compress multiple consecutive empty lines into a single empty line with the `-s` flag.
- **Text Search**: Search for lines containing a specific text or regular expression pattern using the `--search` and `--text` flags.
  - Case-insensitive search is supported with the `--ignore-case` or `-i` flag.
  - For Regular Expression search, append the regex with `reg:`. For example, `ricat --search --text "reg:\\d+" my_file.txt`
- **Base64 Encoding**: Encode the input text using Base64 with the `--encode-base64` flag.
- **Base64 Decoding**: Decode Base64 encoded text using the `--decode-base64` flag.
- **Multiple File Support**: Concatenate and process multiple files specified as command-line arguments.
- **Standard Input Processing**: Read from standard input when no file arguments are provided, allowing `ricat` to be used in command pipelines.
- **Pagination**: Display the output in a paginated manner based on the terminal window size using the `--pages` flag.

These features make `ricat` a versatile tool for text processing and manipulation, providing a range of functionalities to enhance your command-line workflows.

## Usage

### Read a File Without Line Numbering

```bash
ricat my_file.txt
```

### Read a File With Line Numbering Enabled

```bash
ricat my_file.txt -n
```

### Read a File with `$` sign at the end of each Line

```bash
ricat -d my_file.txt
```

### Read Multiple Files

To read and concatenate the contents of multiple files, specify each file path separated by a space:

```bash
ricat file1.txt file2.txt file3.txt
```

### Search Text within a File

For searching text within a file, use the `--search` flag, along with the `--text` flag to specify the text to search for.

To search for and return lines containing a specific pattern or word within a single file:

```bash
ricat --search --text "string_to_search" my_file.txt
```

For regular expression searches, ensure the pattern is a valid regex. For example, to find lines containing digits:

```bash
ricat --search --text "reg:\d+" my_file.txt
```

For ignoring case sensitivity, use the `--ignore-case` or `-i` flag:

```bash
ricat --search --text "string_to_search" --ignore-case my_file.txt
```

### Read from the Standard Input by User

* This behavior allows `ricat` to be used in a pipeline of commands, where it can receive input from a previous command and pass its output to the next command. Without any arguments, `ricat` effectively acts as a simple text editor that displays what you type in real-time, making it useful for creating short text files directly from the command line by redirecting the output to a file using the > operator.

```bash
ricat
```

### Piping output from another command to `ricat`

```bash
df | ricat --search --text "/dev/nvme0"
```
This command will show the disk space usage for all the partitions of drive nvme0

### Base64 Encoding-Decoding

```bash
ricat --encode-base64 message.txt
```
This will convert all the contents of message.txt via `base-64 encoding`

```bash
ricat --decode-base64 encoded_message.txt
```
This command will convert the contents of encoded_message.txt via `base-64 decoding`

```bash
echo -n "And" | base64 | ricat --decode-base64
```
Example of encoding and then decoding the string "And" 

### Show all features currently implemented for `ricat`

```bash
ricat --help
```
## Testing `ricat`

To test the `ricat` features, you can run the following command:

```bash
./test-ricat.sh
```
Run this command in the root of the project

## Extending ricat

Adding new features to `ricat` is straightforward. Implement the `LineTextFeature` trait for any struct to create a new feature. For example, to add a feature that highlights TODO comments in your text files, define a struct implementing `LineTextFeature` that scans each line for the pattern and applies the desired formatting.

## TBD
[x] Remove the need of `-f` flag for reading the filename input

[x] Return all the lines with a given pattern/word.

[x] Feature: Adding Encoding and Decoding ability (`base64`)

[ ] Non-UTF8 Support: To be Done. See Issue [#18](https://github.com/adityanav123/ricat/issues/18)

[ ] Stand-in replacement for cat.

## Contributing

Contributions are most welcome! If you have ideas for new features or improvements, please feel free to submit a pull request or open an issue.

## Bug Reporting

To report bugs, you can go to the [GitHub Discussions](https://github.com/adityanav123/ricat/discussions/11#discussion-6424900)

## Release Notes

### 0.4.1
- Added custom testing file for testing the `ricat` features.
- Fixed Bug with `ricat` not working with the standard input mode: [#21](https://github.com/adityanav123/ricat/issues/21)

    -- To run the test cases, run ./test-ricat.sh in the root directory of the project.

### 0.4.0
- Updated searching using Regex, now you can search for a regular expression pattern in the file via using `reg:` prefix.
- Optimised Performance for features, using Buffered Writer for output, makes less System Calls for writing to the output.
- Optimised Regex Searching via Caching the Compiled Regex, earlier was compiling for each search.
- Code Refactored for easier understanding and maintainability.
- Bug Fixes: Fixed issue with regex search.

### 0.3.3 to 0.3.6
- Added Ability to search text/regular expression in the file with `--ignore-case` flag.
- Added Ability to encode and decode in base64 format [ `--encode-base64` & `--decode-base64` flags]
- Bug Fixes: `ricat` without any file input was not applying feature on standard input mode.

### 0.3.2
- Added ability to do pagination according to your current terminal window size [`--pages` flag]

### 0.3.0
- Added ability to combine multiple files in output.
- Added ability to search text/regular expression in the file.

### 0.1.2 to 0.2.0
- Added dollar sign appending at the end of each line feature.
- Implemented tab space replacement with `^I`.
- Compresses multiple consecutive empty lines into a single empty line.
- Initial support for standard input processing without file input.

### 0.1.1
- Added features to number the lines.

### 0.1.0
- Released with printing from standard input to standard output functionality.
