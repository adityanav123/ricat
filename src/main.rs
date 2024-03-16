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
//! ricat my_file.txt
//! ```
//!
//! ### Read a File With Line Numbering Enabled
//! ```bash
//! ricat -n my_file.txt
//! ```
//!
//! ### Read a file with `$` at end of each line
//! ```bash
//! ricat -d my_file.txt
//! ```
//!
//! ## Extending ricat
//!
//! Adding a new feature to `ricat` is as simple as implementing the `LineTextFeature` for any struct. This modular approach encourages experimentation and customization.
//!
//! For example, to add a feature that highlights TODO comments in your text files, define a struct implementing `LineTextFeature` that scans each line for the pattern and applies the desired formatting.

use clap::Parser;
use regex::{escape, Regex};
use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Error, Read, Write},
    process::exit,
};

/// Trait defining a text feature that can be applied to lines of input.
trait LineTextFeature {
    /// Applies a specific feature to a line of text and returns the modified line or None to omit the line.
    fn apply_feature(&mut self, line: &str) -> Option<String>;
}

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
    fn apply_feature(&mut self, line: &str) -> Option<String> {
        let result = Some(format!("{:} {}", self.current_line, line));
        self.current_line += 1;
        result
    }
}

/// Feature: adding `$` at the last of the line
struct DollarSymbolAtLast;

impl DollarSymbolAtLast {
    fn new() -> Self {
        Self
    }
}

impl LineTextFeature for DollarSymbolAtLast {
    fn apply_feature(&mut self, line: &str) -> Option<String> {
        Some(format!("{}$", line))
    }
}

/// Feature: adding `^I` in place of all the tab-spaces used in the text.
struct ReplaceTabspaces;
impl ReplaceTabspaces {
    fn new() -> Self {
        Self
    }
}

impl LineTextFeature for ReplaceTabspaces {
    fn apply_feature(&mut self, line: &str) -> Option<String> {
        Some(line.replace("\t", "^I"))
    }
}

/// Feature: Compresses multiple consecutive empty lines into a single empty line
struct CompressEmptyLines {
    was_last_line_empty: bool,
}

impl CompressEmptyLines {
    fn new() -> Self {
        Self {
            was_last_line_empty: false,
        }
    }
}

impl LineTextFeature for CompressEmptyLines {
    fn apply_feature(&mut self, line: &str) -> Option<String> {
        if line.trim().is_empty() {
            if self.was_last_line_empty {
                None
            } else {
                self.was_last_line_empty = true;
                Some(String::new()) // Return an empty string to indicate a single empty line should be printed.
            }
        } else {
            self.was_last_line_empty = false;
            Some(line.to_string())
        }
    }
}

/// Feature: Returns Lines which contain a given text/regex
struct LineWithGivenText {
    search_regex: Regex,
}

impl LineWithGivenText {
    fn new(text: &str) -> Self {
        // Attempt to compile the text as a regular expression.
        // If it fails, escape special characters and compile it as plain text.
        let regex = Regex::new(text).unwrap_or_else(|_| {
            let escaped_text = escape(text);
            Regex::new(&escaped_text).unwrap()
        });

        Self {
            search_regex: regex,
        }
    }
}

impl LineTextFeature for LineWithGivenText {
    fn apply_feature(&mut self, line: &str) -> Option<String> {
        // Use the compiled regex to search in the line
        if self.search_regex.is_match(line) {
            Some(line.to_string())
        } else {
            None
        }
    }
}

/// Command line arguments struct, parsed using `clap`.
#[derive(Parser)]
#[clap(
    version = "0.3.0",
    author = "Aditya Navphule <adityanav@duck.com>",
    about = "ricat (Rust Implemented `cat`) : A custom implementation of cat command in Rust"
)]
struct Cli {
    /// Enables line numbering for each line of the input.
    #[clap(short = 'n', long, action = clap::ArgAction::SetTrue, help = "shows line numbers for each line")]
    numbers: bool,

    #[clap(short = 'd', long, action = clap::ArgAction::SetTrue, help = "adds `$` to mark end of each line")]
    dollar: bool,

    #[clap(short = 't', long, action = clap::ArgAction::SetTrue, help = "replaces the tab spaces in the text with ^I")]
    tabs: bool,

    #[clap(short = 's', long, action = clap::ArgAction::SetTrue, help = "suppress repeated empty output lines")]
    squeeze_blank: bool,

    #[clap(long = "search", action = clap::ArgAction::SetTrue, help = "Search text inside file, returns all the lines containing the text")]
    search_flag: bool,

    #[clap(
        long = "text",
        help = "search text: only considered when --search flag is used."
    )]
    search_text: Option<String>,

    /// Optional file path to read from instead of standard input.
    #[clap(help = "File(s) you want to read, multiple files will be appended one after another")]
    files: Vec<String>,
}

fn main() {
    let arguments = Cli::parse();
    let mut features: Vec<Box<dyn LineTextFeature>> = Vec::new(); // any implemented feature

    add_features_from_args(&arguments, &mut features);

    // Determine the input source based on command line arguments
    match (arguments.files.is_empty(), features.is_empty()) {
        (true, true) => {
            // direct copy from standard input and pipe to standard output
            let input = stdin();
            let output = stdout();

            copy(input, output).unwrap_or_else(|error| {
                eprintln!("Error copying standard input to standard output {}", error);
                exit(1);
            });
        }
        (true, false) => {
            let input = stdin();
            let output = stdout();

            // features still need to be processed from standard input
            process_input(Box::new(input), output, &mut features).unwrap_or_else(|error| {
                eprintln!("Error processing input {}", error);
                exit(1);
            });
        }
        (false, true) => {
            // Files are specified
            for file_path in &arguments.files {
                let file = File::open(file_path).unwrap_or_else(|error| {
                    eprintln!("Failed to open {}! {}", file_path, error);
                    exit(1);
                });
                copy(BufReader::new(file), stdout()).unwrap_or_else(|error| {
                    eprintln!(
                        "Error copying file {} to standard output {}",
                        file_path, error
                    );
                    exit(1);
                });
            }
        }
        (false, false) => {
            for file_path in &arguments.files {
                let file = File::open(file_path).unwrap_or_else(|error| {
                    eprintln!("Failed to open {}! {}", file_path, error);
                    exit(1);
                });
                process_input(Box::new(BufReader::new(file)), stdout(), &mut features)
                    .unwrap_or_else(|error| {
                        eprintln!(
                            "Error processing file {} with features {}",
                            file_path, error
                        );
                        exit(1);
                    });
            }
        }
    }
}

/// Will Add Features based on arguments passed
fn add_features_from_args(arguments: &Cli, features: &mut Vec<Box<dyn LineTextFeature>>) {
    if arguments.squeeze_blank {
        features.push(Box::new(CompressEmptyLines::new()));
    }

    if arguments.search_flag {
        let text_to_search = match &arguments.search_text {
            None => "",
            Some(text) => &text,
        };
        features.push(Box::new(LineWithGivenText::new(text_to_search.trim())));
    }

    if arguments.numbers {
        features.push(Box::new(LineNumbering::new()));
    }

    if arguments.dollar {
        features.push(Box::new(DollarSymbolAtLast::new()));
    }

    if arguments.tabs {
        features.push(Box::new(ReplaceTabspaces::new()));
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
fn process_input<R: Read, W: Write>(
    reader: R,
    mut writer: W,
    features: &mut [Box<dyn LineTextFeature>],
) -> Result<(), Error> {
    let buf_reader = BufReader::new(reader);
    for line_result in buf_reader.lines() {
        let line = line_result?;
        let mut processed_line = Some(line);

        for feature in features.iter_mut() {
            if let Some(current_line) = processed_line {
                // Apply each feature to the line if it's not None
                processed_line = feature.apply_feature(&current_line);
            } else {
                // If a feature returned None, stop processing this line and skip to the next one
                break;
            }
        }

        if let Some(current_line) = processed_line {
            writeln!(writer, "{}", current_line)?;
        }
    }
    Ok(())
}

/*

    UNIT TESTS FOR FEATURES

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_numbering_basic() {
        let mut feature = LineNumbering::new();
        let result = feature.apply_feature("Test line");
        assert_eq!(result, Some("1 Test line".to_string()));
    }

    #[test]
    fn line_numbering_increment() {
        let mut feature = LineNumbering::new();
        feature.apply_feature("First line");
        let result = feature.apply_feature("Second line");
        assert_eq!(result, Some("2 Second line".to_string()));
    }

    #[test]
    fn dollar_symbol_at_last_basic() {
        let mut feature = DollarSymbolAtLast::new();
        let result = feature.apply_feature("Test line");
        assert_eq!(result, Some("Test line$".to_string()));
    }

    #[test]
    fn dollar_symbol_at_last_empty() {
        let mut feature = DollarSymbolAtLast::new();
        let result = feature.apply_feature("");
        assert_eq!(result, Some("$".to_string()));
    }

    #[test]
    fn replace_tabspaces_basic() {
        let mut feature = ReplaceTabspaces::new();
        let result = feature.apply_feature("Test\tline");
        assert_eq!(result, Some("Test^Iline".to_string()));
    }

    #[test]
    fn replace_tabspaces_no_tabs() {
        let mut feature = ReplaceTabspaces::new();
        let result = feature.apply_feature("Test line");
        assert_eq!(result, Some("Test line".to_string()));
    }

    #[test]
    fn compress_empty_lines_multiple() {
        let mut feature = CompressEmptyLines::new();
        feature.apply_feature("First line");
        feature.apply_feature("");
        let result = feature.apply_feature("");
        assert!(result.is_none());
    }

    #[test]
    fn compress_empty_lines_single() {
        let mut feature = CompressEmptyLines::new();
        let result = feature.apply_feature("");
        assert_eq!(result, Some("".to_string()));
    }

    #[test]
    fn search_plain_text_found() {
        let mut feature = LineWithGivenText::new("aditya");
        assert_eq!(
            feature.apply_feature("This is a line with aditya in it."),
            Some("This is a line with aditya in it.".to_string())
        );
    }

    #[test]
    fn search_plain_text_not_found() {
        let mut feature = LineWithGivenText::new("nonexistent");
        assert!(feature
            .apply_feature("This line does not contain the search text.")
            .is_none());
    }

    #[test]
    fn search_regex_single_digit_found() {
        let mut feature = LineWithGivenText::new("\\d");
        assert_eq!(
            feature.apply_feature("This line has a 1 digit."),
            Some("This line has a 1 digit.".to_string())
        );
    }

    #[test]
    fn search_regex_single_digit_not_found() {
        let mut feature = LineWithGivenText::new("\\d");
        assert!(feature.apply_feature("No digits here.").is_none());
    }

    #[test]
    fn search_regex_exact_string() {
        let mut feature = LineWithGivenText::new("aditya");
        assert_eq!(
            feature.apply_feature("Exact match aditya"),
            Some("Exact match aditya".to_string())
        );
    }

    #[test]
    fn search_regex_special_characters() {
        // Escaping is handled within the new function, users don't need to escape in the pattern.
        let mut feature = LineWithGivenText::new("\\[aditya\\]");
        assert_eq!(
            feature.apply_feature("Line with [aditya]"),
            Some("Line with [aditya]".to_string())
        );
    }
}
