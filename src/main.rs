//! # ricat: A Rust-Based `cat` Command Implementation
//!
//! This project is a Rust-based reimagining of the classic Unix `cat` command, drawing inspiration from its original implementation in GNU Core Utilities. It demonstrates the power and flexibility of Rust for system utilities development.
//!
//! A key design principle of `ricat` is extensibility. By utilizing a trait, `LineTextFeature`, the application makes it straightforward to introduce new functionalities. Developers can add custom features by implementing the `apply_feature()` method for each line of text. The core logic of `ricat` seamlessly integrates these features without requiring additional modifications.
//!
//! ## Features
//!
//! - **Modular Design**: Easy to extend with new line-based text processing features.
//! - **Trait-Based Feature Implementation**: Implement the `LineTextFeature` trait to create new features.
//!
//! ## Usage
//!
//! Basic usage involves reading files directly or applying the line numbering feature. `ricat` can be used as follows:
//!
//! ### Read a File Without Line Numbering
//! ```bash
//! ricat -f my_file.txt
//! ```
//!
//! ### Read a File With Line Numbering Enabled
//! ```bash
//! ricat -f my_file.txt -n
//! ```
//!
//! ## Extending ricat
//!
//! Adding a new feature to `ricat` is as simple as implementing the `LineTextFeature` for any struct. This modular approach encourages experimentation and customization.
//!
//! For example, to add a feature that highlights TODO comments in your text files, define a struct implementing `LineTextFeature` that scans each line for the pattern and applies the desired formatting.

use clap::Parser;
use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Error, Read, Write},
    process::exit,
};

/// Trait defining a text feature that can be applied to lines of input.
trait LineTextFeature {
    /// Applies a specific feature to a line of text and returns the modified line.
    fn apply_feature(&mut self, line: &str) -> String;
}

// Line Numbering Feature
/// Feature: adding line numbers to each line of text.
struct LineNumbering {
    current_line: usize,
}

impl LineNumbering {
    /// Constructs a new `LineNumbering` feature.
    fn new() -> Self {
        Self { current_line: 1 }
    }
}
impl LineTextFeature for LineNumbering {
    /// Applies line numbering to the given line, prefixing it with the current line number.
    fn apply_feature(&mut self, line: &str) -> String {
        let result = format!("{:} {}", self.current_line, line);
        self.current_line += 1;
        result
    }
}

/// Command line arguments struct, parsed using `clap`.
#[derive(Parser)]
#[clap(
    version = "0.1.0",
    author = "Aditya Navphule <adityanav@duck.com>",
    about = "ricat (Rust Implemented `cat`) : A custom implementation of cat command in Rust"
)]
struct Cli {
    /// Enables line numbering for each line of the input.
    #[arg(short, long, action = clap::ArgAction::SetTrue, help = "shows line numbers for each line")]
    numbers: bool,

    /// Optional file path to read from instead of standard input.
    #[arg(short, long, help = "File you want to read")]
    file: Option<String>,
}

fn main() {
    let arguments = Cli::parse();
    let mut features: Vec<Box<dyn LineTextFeature>> = Vec::new(); // any implemented feature

    // Determine the input source based on command line arguments
    let input: Box<dyn Read> = match arguments.file {
        Some(file) => Box::new(BufReader::new(File::open(file).unwrap_or_else(|error| {
            eprintln!("failed to open file!: {}", error);
            exit(1);
        }))),
        None => Box::new(stdin()), // Default: read from standard input
    };

    // Apply line numbering feature if the -n flag is passed
    if arguments.numbers {
        features.push(Box::new(LineNumbering::new()));
        process_input_with_features(input, stdout(), &mut features).unwrap_or_else(|error| {
            eprintln!("Error processing input : {}", error);
            exit(1);
        });
    } else {
        // default behavior: simply copy input to output
        copy(input, stdout()).unwrap_or_else(|error| {
            eprintln!("Error processing input: {}", error);
            exit(1);
        });
    }
}

/// Copies data from the reader to the writer without modification.
fn copy<R: Read, W: Write>(mut reader: R, mut writer: W) -> Result<(), Error> {
    // buffer to hold chunks of the file
    let mut buffer = [0_u8; 1024];

    loop {
        let len = reader.read(&mut buffer)?;
        if len == 0 {
            break; // End of file or stream
        }
        writer.write_all(&buffer[..len])?;
    }
    Ok(())
}

/// Processes input by applying each configured text feature to every line.
fn process_input_with_features<R: Read, W: Write>(
    reader: R,
    mut writer: W,
    features: &mut [Box<dyn LineTextFeature>],
) -> Result<(), Error> {
    let buf_reader = BufReader::new(reader);
    for line in buf_reader.lines() {
        let mut line = line?;
        for feature in &mut *features {
            // Apply each feature to the line
            line = feature.apply_feature(&line);
        }
        // Write the modified line to output
        writeln!(writer, "{}", line)?;
    }
    Ok(())
}
