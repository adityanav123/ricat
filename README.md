# ricat: A Rust-Based `cat` Command Implementation

This project is a Rust-based reimagining of the classic Unix `cat` command, drawing inspiration from its original implementation in GNU Core Utilities. It demonstrates the power and flexibility of Rust for system utilities development.


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

### Read from the Standard Input by User
* This behavior allows `ricat` to be used in a pipeline of commands, where it can receive input from a previous command and pass its output to the next command. Without any arguments, `ricat` effectively acts as a simple text editor that displays what you type in real-time, making it useful for creating short text files directly from the command line by redirecting the output to a file using the > operator.

```bash
ricat
```

## Extending ricat

Adding new features to `ricat` is straightforward. Implement the `LineTextFeature` trait for any struct to create a new feature. For example, to add a feature that highlights TODO comments in your text files, define a struct implementing `LineTextFeature` that scans each line for the pattern and applies the desired formatting.

## TBD
[x] Remove the need of `-f` flag for reading the filename input

[ ] Stand-in replacement for cat (if possible).

[ ] return all the lines with a given pattern/word.

## Contributing

Contributions are welcome! If you have ideas for new features or improvements, please feel free to submit a pull request or open an issue.
