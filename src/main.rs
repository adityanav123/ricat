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
mod encoding_decoding_feature;

use clap::Parser;
use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event},
    execute,
    terminal::{self, Clear, ClearType},
};
use regex::{escape, Regex};
use std::{
    fs::File,
    io::{self, stdin, stdout, BufRead, BufReader, Error, Read, Write},
    process::exit,
};

// Encoding-Decoding Module
use encoding_decoding_feature::{Base64, DataEncoding as _};

/// get current user terminal height for pagination
fn get_terminal_height() -> u16 {
    match terminal::size() {
        Ok((_, height)) => height,
        Err(_) => 24, //default
    }
}

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
        Some(line.replace('\t', "^I"))
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

/// Base64 Encoding Feature Integration
struct Base64Encoding;

impl Base64Encoding {
    fn new() -> Self {
        Self
    }
}

impl LineTextFeature for Base64Encoding {
    fn apply_feature(&mut self, line: &str) -> Option<String> {
        Base64::encode(line)
    }
}

/// Base64 Decoding Feature Integration
struct Base64Decoding;

impl Base64Decoding {
    fn new() -> Self {
        Self
    }
}

impl LineTextFeature for Base64Decoding {
    fn apply_feature(&mut self, line: &str) -> Option<String> {
        Base64::decode(line)
    }
}

/// Command line arguments struct, parsed using `clap`.
#[derive(Parser)]
#[clap(
    version = "0.3.4",
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

    #[clap(long = "pages", help = "apply pagination to the output")]
    pagination: bool,

    #[clap(long = "eb64", help = "encode the input text using Base64")]
    encode: bool,

    #[clap(long = "db64", help = "decode the input text using Base64")]
    decode: bool,

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
            // applying pagination
            if arguments.pagination {
                let input = stdin();
                let processed_lines = process_input_ret(input.lock(), &mut features)
                    .unwrap_or_else(|error| {
                        eprintln!("Error processing the features! {}", error);
                        exit(1);
                    });
                //paginate the feature processed lines
                paginate_output(processed_lines, stdout()).unwrap_or_else(|error| {
                    eprintln!("Error paginating. {}", error);
                    exit(1);
                });
            } else {
                // direct copy from standard input and pipe to standard output
                let input = stdin();
                let output = stdout();

                copy(input, output).unwrap_or_else(|error| {
                    eprintln!("Error copying standard input to standard output {}", error);
                    exit(1);
                });
            }
        }
        (true, false) | (false, false) => {
            let reader_sources: Vec<Box<dyn Read>> = if arguments.files.is_empty() {
                vec![Box::new(stdin())]
            } else {
                arguments
                    .files
                    .iter()
                    .map(|file_path| {
                        let file = File::open(file_path).unwrap_or_else(|error| {
                            eprintln!("Failed to open {}! Error: {}", file_path, error);
                            exit(1);
                        });
                        Box::new(file) as Box<dyn Read>
                    })
                    .collect()
            };

            // standard input
            if arguments.files.is_empty() {
                process_input_stdout(Box::new(stdin()), &mut features).unwrap_or_else(|error| {
                    eprintln!("Error processing Line! {}", error);
                    exit(1);
                });
            } else {
                let mut all_processed_lines = Vec::<String>::new();
                for source in reader_sources {
                    let processed_lines =
                        process_input_ret(source, &mut features).unwrap_or_else(|error| {
                            eprintln!("Error processing Line! {}", error);
                            exit(1);
                        });
                    all_processed_lines.extend(processed_lines);
                }

                // check if pagination enabled
                if arguments.pagination {
                    paginate_output(all_processed_lines, stdout()).unwrap_or_else(|error| {
                        eprintln!("Error: paginating = {}", error);
                        exit(1);
                    });
                } else {
                    for line in all_processed_lines {
                        println!("{}", line);
                    }
                }
            }
        }
        (false, true) => {
            // without features
            if arguments.pagination {
                let mut all_lines = Vec::<String>::new();
                for file_path in &arguments.files {
                    let file = File::open(file_path).unwrap_or_else(|error| {
                        eprintln!("Error in opening file {}! {}", file_path, error);
                        exit(1);
                    });
                    let processed_lines = process_input_ret(BufReader::new(file), &mut features)
                        .unwrap_or_else(|error| {
                            eprintln!("Error processing Line! {}", error);
                            exit(1);
                        });

                    all_lines.extend(processed_lines);
                }
                paginate_output(all_lines, stdout()).unwrap_or_else(|error| {
                    eprintln!("Error: paginating = {}", error);
                    exit(1);
                });
            } else {
                // Directly copy files to standard output
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
            Some(text) => text,
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

    if arguments.encode {
        features.push(Box::new(Base64Encoding::new()));
    }

    if arguments.decode {
        features.push(Box::new(Base64Decoding::new()));
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

/// Processing input over the standard output
fn process_input_stdout<R: Read>(
    reader: R,
    features: &mut [Box<dyn LineTextFeature>],
) -> Result<(), io::Error> {
    let buf_reader = BufReader::new(reader);

    for line_result in buf_reader.lines() {
        let line = line_result?;
        let mut processed_line = Some(line);

        for feature in features.iter_mut() {
            if let Some(curr_line) = processed_line {
                processed_line = feature.apply_feature(&curr_line);
            } else {
                break;
            }
        } // for - applying all the features over the current line

        if let Some(current_line) = processed_line {
            println!("{}", current_line);
        }
    } // for - read each line and apply feature
    Ok(())
}

/// Processes input by applying each configured text feature to every line.
fn process_input_ret<R: Read>(
    reader: R,
    features: &mut [Box<dyn LineTextFeature>],
) -> Result<Vec<String>, Error> {
    let buf_reader = BufReader::new(reader);
    let mut processed_lines = Vec::new();

    for line_result in buf_reader.lines() {
        let line = line_result?;
        let mut processed_line = Some(line);

        for feature in features.iter_mut() {
            if let Some(current_line) = processed_line {
                processed_line = feature.apply_feature(&current_line);
            } else {
                break;
            }
        }

        if let Some(current_line) = processed_line {
            processed_lines.push(current_line);
        }
    }
    Ok(processed_lines)
}

/// Paginate output
fn paginate_output<W: Write>(lines: Vec<String>, mut writer: W) -> io::Result<()> {
    let terminal_height = get_terminal_height() as usize;
    let page_size = terminal_height.saturating_sub(1);

    for (index, line) in lines.iter().enumerate() {
        writeln!(writer, "{}", line)?;
        if (index + 1) % page_size == 0 {
            wait_for_user_input(&mut writer)?;
        }
    }
    Ok(())
}

/// Waiting for User Input
fn wait_for_user_input<W: Write>(writer: &mut W) -> io::Result<()> {
    execute!(writer, Hide)?;

    write!(writer, "--More--(press any key)")?;
    writer.flush()?;

    // Enter raw mode to read key presses without echoing them.
    crossterm::terminal::enable_raw_mode()?;

    loop {
        match read()? {
            // Exit loop on any key press.
            Event::Key(_) => break,
            _ => continue,
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    execute!(writer, Show)?; // Show the cursor again.

    // Clear the "--More--" line.
    execute!(writer, Clear(ClearType::CurrentLine))?;
    write!(writer, "\r")?; // Move cursor to the beginning of the line

    Ok(())
}

mod tests;
