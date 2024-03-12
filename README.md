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
ricat -f my_file.txt
```

### Read a File With Line Numbering Enabled

```bash
ricat -f my_file.txt -n
```

## Extending ricat

Adding new features to `ricat` is straightforward. Implement the `LineTextFeature` trait for any struct to create a new feature. For example, to add a feature that highlights TODO comments in your text files, define a struct implementing `LineTextFeature` that scans each line for the pattern and applies the desired formatting.

## Contributing

Contributions are welcome! If you have ideas for new features or improvements, please feel free to submit a pull request or open an issue.
