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

## Features

- **Modular Design**: Easily extended with new line-based text processing features.
- **Trait-Based Feature Implementation**: New features can be added by implementing the `LineTextFeature` trait.

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

To search for and return lines containing a specific pattern or word within a single file:

```bash
ricat --search --text "string_to_search" my_file.txt
```

For regular expression searches, ensure the pattern is a valid regex. For example, to find lines containing digits:

```bash
ricat --search --text "\d+" my_file.txt
```

### Read from the Standard Input by User

* This behavior allows `ricat` to be used in a pipeline of commands, where it can receive input from a previous command and pass its output to the next command. Without any arguments, `ricat` effectively acts as a simple text editor that displays what you type in real-time, making it useful for creating short text files directly from the command line by redirecting the output to a file using the > operator.

```bash
ricat
```

## Extending ricat

Adding new features to `ricat` is straightforward. Implement the `LineTextFeature` trait for any struct to create a new feature. For example, to add a feature that highlights TODO comments in your text files, define a struct implementing `LineTextFeature` that scans each line for the pattern and applies the desired formatting.

## TBD
[x] Remove the need of `-f` flag for reading the filename input

[x] Return all the lines with a given pattern/word.

[ ] Stand-in replacement for cat (if possible).


## Contributing

Contributions are welcome! If you have ideas for new features or improvements, please feel free to submit a pull request or open an issue.

## Release Notes

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
