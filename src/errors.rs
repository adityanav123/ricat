use std::io;

use thiserror::Error;

/// Represents the possible errors that can occur in the `ricat` application.
#[derive(Error, Debug)]
pub enum RicatError {
    /// Represents an I/O error.
    ///
    /// This error variant is used to wrap and propagate `std::io::Error` instances.
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),

    /// Represents an error that occurs when opening a file.
    ///
    /// This error variant is used when there is a failure to open a file for reading.
    /// It includes a string message providing more details about the error.
    #[error("Failed to open file: {0}")]
    FileOpenError(String),

    /// Represents an error that occurs during the processing of a line.
    ///
    /// This error variant is used when there is an error while processing a line of text.
    /// It includes a string message providing more details about the error.
    #[error("Error processing line: {0}")]
    LineProcessingError(String),

    /// Represents an error that occurs during pagination.
    ///
    /// This error variant is used when there is an error related to pagination functionality.
    /// It includes a string message providing more details about the error.
    #[error("Pagination error: {0}")]
    PaginationError(String),

    /// Represents an error that occurs within a feature.
    ///
    /// This error variant is used when there is an error specific to a feature implementation.
    /// It includes a string message providing more details about the error.
    #[error("Feature error: {0}")]
    FeatureError(String),

    /// Represents an error that occurs when writing a line.
    ///
    /// This error variant is used when there is an error while writing a line to the output.
    /// It includes a string message providing more details about the error.
    #[error("Error writing line: {0}")]
    LineWriteError(String),

    /// Represents an error that occurs when flushing the output.
    ///
    /// This error variant is used when there is an error while flushing the output stream.
    /// It includes a string message providing more details about the error.
    #[error("Error flushing output: {0}")]
    OutputFlushError(String),

    /// Represents an error that occurs when enabling raw mode.
    ///
    /// This error variant is used when there is an error while enabling raw mode for input handling.
    /// It includes a string message providing more details about the error.
    #[error("Error enabling raw mode: {0}")]
    RawModeEnableError(String),

    /// Represents an error that occurs when disabling raw mode.
    ///
    /// This error variant is used when there is an error while disabling raw mode for input handling.
    /// It includes a string message providing more details about the error.
    #[error("Error disabling raw mode: {0}")]
    RawModeDisableError(String),

    /// Represents an error that occurs when reading input.
    ///
    /// This error variant is used when there is an error while reading input from the user.
    /// It includes a string message providing more details about the error.
    #[error("Error reading input: {0}")]
    InputReadError(String),

    /// Represents an error that occurs when hiding the cursor.
    ///
    /// This error variant is used when there is an error while hiding the cursor.
    /// It includes a string message providing more details about the error.
    #[error("Error hiding cursor: {0}")]
    CursorHideError(String),

    /// Represents an error that occurs when showing the cursor.
    ///
    /// This error variant is used when there is an error while showing the cursor.
    /// It includes a string message providing more details about the error.
    #[error("Error showing cursor: {0}")]
    CursorShowError(String),

    /// Represents an error that occurs when clearing the current line.
    ///
    /// This error variant is used when there is an error while clearing the current line in the terminal.
    /// It includes a string message providing more details about the error.
    #[error("Error clearing current line: {0}")]
    ClearLineError(String),

    /// Represents an error that occurs when moving the cursor to the beginning of the line.
    ///
    /// This error variant is used when there is an error while moving the cursor to the beginning of the line.
    /// It includes a string message providing more details about the error.
    #[error("Error moving cursor to the beginning of the line: {0}")]
    CursorMoveError(String),

    /// Represents an error that occurs when Regex provided for searching inside a text is invalid.
    ///
    /// This error variant is used when there is an error while compiling regex and caching it.
    /// It includes a string message providing more details about the error.
    #[error("Error compiling Regex: {0}")]
    RegexCompilationError(String),

    /// Represents an error that occurs when acquiring a lock on the regex cache.
    ///
    /// This error variant is used when there is an error while trying to acquire a lock on the regex cache.
    /// It includes a string message providing more details about the error.
    #[error("Error acquiring lock on regex cache: {0}")]
    RegexCacheError(String),

    /// Represents an error that occurs when failed to create memory map from file
    ///
    /// This error variant is used when there is an error while creating memory map from file
    /// It includes a string message providing more details about the error.
    #[error("Error writing data to writer via Memory Mapped IO: {0}")]
    MemoryMapError(String),

    /// Represents an error when writing contents of memory map to writer
    ///
    /// This error variant is used when there is an error while writing contents of memory map to writer
    /// It includes a string message providing more details about the error.
    #[error("Error writing data to writer via Memory Mapped IO: {0}")]
    MemoryMapWriteError(String),

    /// Represents an Error when reading the config file
    #[error("Error reading config file: {0}")]
    ConfigReadError(String),

    /// User Quits the Pagination Mode by pressing 'q'
    #[error("User Quit Pagination Mode")]
    UserQuit,
}
