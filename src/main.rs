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
//! - **Line Numbering**: Display line numbers for each line of the input.
//! - **Dollar Symbol at End**: Append a `$` symbol at the end of each line.
//! - **Replace Tab Spaces**: Replace tab spaces in the text with `^I`.
//! - **Compress Empty Lines**: Compress multiple consecutive empty lines into a single empty line.
//! - **Search Text**: Search for lines containing a specific text or regular expression pattern. Prefix the search text with 'reg:' to treat it as a regular expression, e.g., 'reg:\\\\d+' to search for digits.
//! - **Case-Insensitive Search**: Perform case-insensitive search for lines containing a specific text.
//! - **Base64 Encoding**: Encode the input text using Base64.
//! - **Base64 Decoding**: Decode Base64 encoded text.
//! - **Pagination**: Display the output in a paginated manner, allowing user to navigate through pages.
//!
//! ## Usage
//!
//! `ricat` supports various command-line options to enable different features and customize the output. Here are some common usage examples:
//!
//! ### Read a File directly 
//! ```bash
//! ricat my_file.txt
//! ```
//!
//! ### Read a File With Line Numbering Enabled
//! ```bash
//! ricat -n my_file.txt
//! ```
//!
//! ### Read a File with `$` at End of Each Line
//! ```bash
//! ricat -d my_file.txt
//! ```
//!
//! ### Replace Tab Spaces with `^I`
//! ```bash
//! ricat -t my_file.txt
//! ```
//!
//! ### Compress Empty Lines
//! ```bash
//! ricat -s my_file.txt
//! ```
//!
//! ### Search for Lines Containing a Specific Text
//! ```bash
//! ricat --search --text "search_text" my_file.txt
//! ```
//!
//! ### Search for Lines Matching a Regular Expression
//! ```bash
//! ricat --search --text "reg:\\d+" my_file.txt
//! ```
//!
//! ### Perform Case-Insensitive Search
//! ```bash
//! ricat --search --text "search_text" -i my_file.txt
//! ```
//!
//! ### Encode Input Text Using Base64
//! ```bash
//! ricat --encode-base64 my_file.txt
//! ```
//!
//! ### Decode Base64 Encoded Text
//! ```bash
//! ricat --decode-base64 my_encoded_file.txt
//! ```
//!
//! ### Enable Pagination
//! ```bash
//! ricat --pages my_large_file.txt
//! ```
//!
//! ## Extending ricat
//!
//! Adding a new feature to `ricat` is as simple as implementing the `LineTextFeature` trait for any struct. This modular approach encourages experimentation and customization.
//!
//! For example, to add a feature that highlights TODO comments in your text files, define a struct implementing `LineTextFeature` that scans each line for the pattern and applies the desired formatting.
//!
//! ## Testing
//!
//! `ricat` includes a comprehensive test suite to ensure the correctness and reliability of its functionality. The tests cover various scenarios and edge cases, including:
//!
//! - Basic functionality of each feature
//! - Interaction between multiple features
//! - Handling of empty input
//! - Proper resetting of state between input sources
//! - Encoding and decoding of text using Base64
//! - Case-insensitive search functionality
//!
//! To run the tests, clone the repo and use the `cargo test` command.
//!
//! ## Contributing
//!
//! Contributions to `ricat` are welcome! If you have an idea for a new feature or improvement, please open an issue or submit a pull request on the project's GitHub repository.
//!
//! When contributing, please ensure that your changes are well-tested and adhere to the project's coding style and conventions.
//!
//! ## License
//!
//! `ricat` is open-source software licensed under the [MIT License](https://opensource.org/licenses/MIT).



pub mod encoding_decoding_feature;
pub mod errors;

use clap::Parser;
use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event},
    execute,
    terminal::{self, Clear, ClearType},
};
use errors::RicatError;
use memmap2::Mmap;
use regex::Regex;
use std::{
    fs::File, io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Write}, process
};


// Encoding-Decoding Module
pub use encoding_decoding_feature::{Base64, DataEncoding as _};

/// get current user terminal height for pagination
fn get_terminal_height() -> u16 {
    match terminal::size() {
        Ok((_, height)) => height,
        Err(_) => 24, //default
    }
}

/// Trait defining a text feature that can be applied to lines of input.
pub trait LineTextFeature {
    /// Applies a specific feature to a line of text and returns the modified line or None to omit the line.
    fn apply_feature(&mut self, line: &str) -> Option<String>;
}

/// Feature: adding line numbers to each line of text.
pub struct LineNumbering {
    current_line: usize,
}

impl LineNumbering {
    pub fn new() -> Self {
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
pub struct DollarSymbolAtLast;

impl DollarSymbolAtLast {
    pub fn new() -> Self {
        Self
    }
}

impl LineTextFeature for DollarSymbolAtLast {
    fn apply_feature(&mut self, line: &str) -> Option<String> {
        Some(format!("{}$", line))
    }
}

/// Feature: adding `^I` in place of all the tab-spaces used in the text.
pub struct ReplaceTabspaces;
impl ReplaceTabspaces {
    pub fn new() -> Self {
        Self
    }
}

impl LineTextFeature for ReplaceTabspaces {
    fn apply_feature(&mut self, line: &str) -> Option<String> {
        Some(line.replace('\t', "^I"))
    }
}

/// Feature: Compresses multiple consecutive empty lines into a single empty line
pub struct CompressEmptyLines {
    was_last_line_empty: bool,
}

impl CompressEmptyLines {
    pub fn new() -> Self {
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
pub struct LineWithGivenText {
    /// search pattern or string input
    search_pattern: String,
    /// ignore case for search
    _ignore_case: bool,
    /// compiled regex; is cached.
    regex: Option<Regex>,
}

impl LineWithGivenText {
    pub fn new(text: &str, ignore_case: bool) -> Self {
        let (is_regex, clean_text) = if text.starts_with("reg:") {
            (true, &text["reg:".len()..]) // Strip the prefix and treat the rest as a regex
        } else {
            (false, text) // literal text
        };

        let pattern = if is_regex {
            if ignore_case {
                format!("(?i){}", clean_text)
            } else {
                clean_text.to_string()
            }
        } else {
            let escaped_text = regex::escape(clean_text);
            if ignore_case {
                format!("(?i){}", escaped_text)
            } else {
                escaped_text
            }
        };

        Self {
            search_pattern: pattern,
            _ignore_case: ignore_case,
            regex: None,
        }
    }
}

impl LineTextFeature for LineWithGivenText {
    fn apply_feature(&mut self, line: &str) -> Option<String> {
        if self.regex.is_none() {
            self.regex = Regex::new(&self.search_pattern)
                .map_err(|err| RicatError::RegexCompilationError(format!("Invalid regex '{}': {}", self.search_pattern, err)))
                .ok();
        }

        if let Some(ref regex) = self.regex {
            if regex.is_match(line) {
                return Some(line.to_string());
            }
        }
        None
    }
}

/// Base64 Encoding Feature Integration
pub struct Base64Encoding;

impl Base64Encoding {
    pub fn new() -> Self {
        Self
    }
}

impl LineTextFeature for Base64Encoding {
    fn apply_feature(&mut self, line: &str) -> Option<String> {
        Base64::encode(line)
    }
}

/// Base64 Decoding Feature Integration
pub struct Base64Decoding;

impl Base64Decoding {
    pub fn new() -> Self {
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
    version = "0.4.1",
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

    #[clap(
        long = "search", 
        action = clap::ArgAction::SetTrue, 
        help = "Search text inside the file. Returns all lines containing the text."
    )]
    search_flag: bool,
    
    #[clap(
        long = "text",
        help = "Search text: only considered when --search flag is used. Use 'reg:' prefix for regex search, e.g., 'reg:\\\\w+' for words."
    )]
    search_text: Option<String>,
    
    #[clap(
        short = 'i',
        long = "ignore-case",
        help = "Ignore case for searching text: only considered when --search flag is used.",
        action=clap::ArgAction::SetTrue,
    )]
    ignore_case: bool,
    

    #[clap(long = "pages", action = clap::ArgAction::SetTrue, help = "Apply Pagination to the output")]
    pagination: bool,

    #[clap(long = "encode-base64", action = clap::ArgAction::SetTrue, help = "Encode the input text using Base64")]
    encode: bool,

    #[clap(long = "decode-base64", action = clap::ArgAction::SetTrue, help = "Decode the input text using Base64")]
    decode: bool,

    /// Optional file path to read from instead of standard input.
    #[clap(help = "File(s) you want to read, multiple files will be appended one after another")]
    files: Vec<String>,
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    }
}

/// Starts Executing Ricat
fn run() -> Result<(), RicatError> {
    let arguments = Cli::parse();
    let mut features = add_features_from_args(&arguments); // stores the implemented features

    // Determine the input source based on command line arguments
    match (arguments.files.is_empty(), features.is_empty()) {
        (true, true) => handle_via_std_output(&arguments),
        (true, false) | (false, false) => handle_files_or_features(&arguments, &mut features),
        (false, true) => handle_files_without_features(&arguments),
    }
}

/// handling empty files and features
fn handle_via_std_output(_arguments: &Cli) -> Result<(), RicatError> {
    let input = stdin();
    let output = stdout();
    copy(input, output)?;

    Ok(())
}

/// handle files or features : features are enabled, files can/cannot be passed
fn handle_files_or_features(
    arguments: &Cli,
    features: &mut [Box<dyn LineTextFeature>],
) -> Result<(), RicatError> {
    if arguments.files.is_empty() {
        process_input_stdout(stdin(), features, false).map_err(|error| {
            RicatError::LineProcessingError(format!("Error processing line: {}", error))
        })?;
    } else {
        let reader_sources: Result<Vec<Box<dyn Read>>, RicatError> = arguments
            .files
            .iter()
            .map(|file_path| {
                File::open(file_path)
                    .map(|file| Box::new(file) as Box<dyn Read>)
                    .map_err(|error| {
                        RicatError::FileOpenError(format!(
                            "Failed to open {}: {}",
                            file_path, error
                        ))
                    })
            })
            .collect();

        let reader_sources = reader_sources?;

        let mut all_processed_lines = Vec::<String>::new();

        for source in reader_sources {
            let processed_lines = process_input_ret(source, features).map_err(|error| {
                RicatError::LineProcessingError(format!("Error processing line: {}", error))
            })?;
            all_processed_lines.extend(processed_lines);
        }

        if arguments.pagination {
            paginate_output(all_processed_lines, stdout()).map_err(|error| {
                RicatError::PaginationError(format!("Error paginating: {}", error))
            })?;
        } else {
            let stdout = stdout();
            let mut buf_writer = BufWriter::new(stdout.lock());

            for line in all_processed_lines {
                writeln!(buf_writer, "{}", line).map_err(|error| {
                    RicatError::LineProcessingError(format!("Error writing line: {}", error))
                })?;
            }

            buf_writer.flush().map_err(|error| {
                RicatError::OutputFlushError(format!("Error flushing output: {}", error))
            })?;
        }
    }

    Ok(())
}

/// handle files without features
fn handle_files_without_features(arguments: &Cli) -> Result<(), RicatError> {
    if arguments.pagination {
        let mut all_lines = Vec::<String>::new();
        for file_path in &arguments.files {
            let file = File::open(file_path).map_err(|error| {
                RicatError::FileOpenError(format!("Error opening file {}: {}", file_path, error))
            })?;
            let processed_lines =
                process_input_ret(BufReader::new(file), &mut []).map_err(|error| {
                    RicatError::LineProcessingError(format!("Error processing line: {}", error))
                })?;

            all_lines.extend(processed_lines);
        }
        paginate_output(all_lines, stdout())
            .map_err(|error| RicatError::PaginationError(format!("Error paginating: {}", error)))?;
    } else {
        // Directly copy files to standard output
        for file_path in &arguments.files {
            /*let file = File::open(file_path).map_err(|error| {
                RicatError::FileOpenError(format!("Error opening file {}: {}", file_path, error))
            })?;
            copy(BufReader::new(file), stdout()).map_err(|error| error)?;
              */
            copy_mmap(file_path, stdout()).map_err(|error| error)?;
        }
    }

    Ok(())
}

/// Generate Feature Vector: Will Add Features based on arguments passed
fn add_features_from_args(arguments: &Cli) -> Vec<Box<dyn LineTextFeature>> {
    let mut features = Vec::<Box<dyn LineTextFeature>>::new();
    if arguments.squeeze_blank {
        features.push(Box::new(CompressEmptyLines::new()));
    }

    if arguments.encode {
        features.push(Box::new(Base64Encoding::new()));
    }

    if arguments.decode {
        features.push(Box::new(Base64Decoding::new()));
    }

    if arguments.search_flag {
        let text_to_search = match &arguments.search_text {
            None => "",
            Some(text) => text,
        };
        features.push(Box::new(LineWithGivenText::new(
            text_to_search.trim(),
            arguments.ignore_case,
        )));
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

    features
}

/// Copies data from the reader to the writer without modification.
/* Less System Calls: the number of read and write system calls is reduced */
pub fn copy<R: Read, W: Write>(mut reader: R, mut writer: W) -> Result<(), RicatError> {
    // buffer to hold chunks of the file
    const BUFFER_SIZE: usize = 4096;
    let mut buffer = vec![0_u8; BUFFER_SIZE];

    loop {
        let len = reader.read(&mut buffer)?;
        if len == 0 {
            break; // End of file or stream
        }
        writer.write_all(&buffer[..len])?;
    }
    Ok(())
}

/*In Memory Copy: Via Memory Mapped IO */
// TODO: Read and implement it.
pub fn copy_mmap<W:Write>(file_path: &str, mut writer: W) -> Result<(), RicatError> {
    println!("Copying via Memory Mapped IO");
    let file = File::open(file_path).map_err(|error| {
        RicatError::FileOpenError(format!("Error opening file {}: {}", file_path, error))
    })?;

    let mmap = unsafe { Mmap::map(&file) }.map_err(|error| {
        RicatError::MemoryMapError(format!("Error mapping file to memory: {}", error))
    })?;

    writer.write_all(&mmap).map_err(|error| {
        RicatError::MemoryMapWriteError(format!("Error writing to output: {}", error))
    })?;

    Ok(())
}

/// Processing input and flushing to standard output
pub fn process_input_stdout<R: Read>(
    reader: R,
    features: &mut [Box<dyn LineTextFeature>],
    is_live: bool,
) -> Result<(), RicatError> {
    let buf_reader = BufReader::new(reader);
    let stdout = stdout();
    let stdout_lock = stdout.lock();

    let mut writer: Box<dyn Write> = if is_live {
        Box::new(BufWriter::new(stdout_lock))
    } else {
        Box::new(stdout_lock)
    };

    for line_result in buf_reader.lines() {
        let line = line_result?;
        let mut processed_line = Some(line);

        for feature in features.iter_mut() {
            if let Some(curr_line) = processed_line {
                processed_line = feature.apply_feature(&curr_line);
            } else {
                break;
            }
        }

        if let Some(curr_line) = processed_line {
            writeln!(writer, "{}", curr_line).map_err(|error| {
                RicatError::LineProcessingError(format!("Error writing line: {}", error))
            })?;
        }
    }

    writer.flush().map_err(|error| {
        RicatError::OutputFlushError(format!("Error flushing output: {}", error))
    })?;

    Ok(())
}/// Processes input by applying each configured text feature to every line.
pub fn process_input_ret<R: Read>(
    reader: R,
    features: &mut [Box<dyn LineTextFeature>],
) -> Result<Vec<String>, RicatError> {
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
pub fn paginate_output<W: Write>(lines: Vec<String>, mut writer: W) -> Result<(), RicatError> {
    let terminal_height = get_terminal_height() as usize;
    let page_size = terminal_height.saturating_sub(1);

    for (index, current_line) in lines.iter().enumerate() {
        writeln!(writer, "{}", current_line).map_err(|error| {
            RicatError::PaginationError(format!("Error writing line: {}", error))
        })?;
        if (index + 1) % page_size == 0 {
            wait_for_user_input(&mut writer).map_err(|error| {
                RicatError::PaginationError(format!("Error waiting for user input: {}", error))
            })?;
        }
    }
    Ok(())
}

/// Waiting for User Input
pub fn wait_for_user_input<W: Write>(writer: &mut W) -> Result<(), RicatError> {
    execute!(writer, Hide).map_err(|error| RicatError::CursorHideError(error.to_string()))?;

    write!(writer, "--More--(press any key)")
        .map_err(|error| RicatError::LineWriteError(error.to_string()))?;
    writer
        .flush()
        .map_err(|error| RicatError::OutputFlushError(error.to_string()))?;

    crossterm::terminal::enable_raw_mode()
        .map_err(|error| RicatError::RawModeEnableError(error.to_string()))?;

    loop {
        match read() {
            Ok(Event::Key(_)) => break,
            Ok(_) => continue,
            Err(error) => {
                return Err(RicatError::InputReadError(error.to_string()));
            }
        }
    }

    crossterm::terminal::disable_raw_mode()
        .map_err(|error| RicatError::RawModeDisableError(error.to_string()))?;
    execute!(writer, Show).map_err(|error| RicatError::CursorShowError(error.to_string()))?;

    execute!(writer, Clear(ClearType::CurrentLine))
        .map_err(|error| RicatError::ClearLineError(error.to_string()))?;
    write!(writer, "\r").map_err(|error| RicatError::CursorMoveError(error.to_string()))?;

    Ok(())
}

mod tests;
