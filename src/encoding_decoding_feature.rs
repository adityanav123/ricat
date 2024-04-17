use base64::engine::general_purpose;
use base64::prelude::*;

/// DataEncoding Trait : for Encoding and Decoding Files
pub trait DataEncoding {
    fn encode(data: &str) -> Option<String>;
    fn decode(text: &str) -> Option<String>;
}

/// Base64 Encoding-Decoding
pub struct Base64;

impl DataEncoding for Base64 {
    fn encode(data: &str) -> Option<String> {
        Some(general_purpose::STANDARD.encode(data.as_bytes()))
    }

    fn decode(text: &str) -> Option<String> {
        let decoded_message = general_purpose::STANDARD.decode(text).ok()?;
        String::from_utf8(decoded_message).ok()
    }
}
