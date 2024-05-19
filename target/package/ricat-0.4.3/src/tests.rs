//! Unit Tests for ricat : `cargo test` to run all the tests at once
//! for indiviual tests : `cargo test test-name` will run all those tests, which contain test-name
#[cfg(test)]
mod tests {
    use crate::*;

    /// Tests the basic functionality of the `LineNumbering` feature.
    /// Ensures that the line number is added correctly to the line.
    #[test]
    fn line_numbering_basic() {
        let mut feature = LineNumbering::new();
        let result = feature.apply_feature("Test line");
        assert_eq!(result, Some("1 Test line".to_string()));
    }

    /// Tests the incrementing behavior of the `LineNumbering` feature.
    /// Verifies that the line number is incremented for each subsequent line.
    #[test]
    fn line_numbering_increment() {
        let mut feature = LineNumbering::new();
        feature.apply_feature("First line");
        let result = feature.apply_feature("Second line");
        assert_eq!(result, Some("2 Second line".to_string()));
    }

    /// Tests the basic functionality of the `DollarSymbolAtLast` feature.
    /// Ensures that a dollar symbol is appended to the end of the line.
    #[test]
    fn dollar_symbol_at_last_basic() {
        let mut feature = DollarSymbolAtLast::new();
        let result = feature.apply_feature("Test line");
        assert_eq!(result, Some("Test line$".to_string()));
    }

    /// Tests the basic functionality of the `ReplaceTabspaces` feature.
    /// Verifies that tab spaces are replaced with the "^I" character.
    #[test]
    fn replace_tabspaces_basic() {
        let mut feature = ReplaceTabspaces::new();
        let result = feature.apply_feature("Test\tline");
        assert_eq!(result, Some("Test^Iline".to_string()));
    }

    /// Tests the `ReplaceTabspaces` feature when no tab spaces are present.
    /// Ensures that the line remains unchanged.
    #[test]
    fn replace_tabspaces_no_tabs() {
        let mut feature = ReplaceTabspaces::new();
        let result = feature.apply_feature("Test line");
        assert_eq!(result, Some("Test line".to_string()));
    }

    /// Tests the `CompressEmptyLines` feature with multiple empty lines.
    /// Verifies that consecutive empty lines are compressed into a single empty line.
    #[test]
    fn compress_empty_lines_multiple() {
        let mut feature = CompressEmptyLines::new();
        feature.apply_feature("First line");
        feature.apply_feature("");
        let result = feature.apply_feature("");
        assert!(result.is_none());
    }

    /// Tests the `CompressEmptyLines` feature with a single empty line.
    /// Ensures that a single empty line is returned as an empty string.
    #[test]
    fn compress_empty_lines_single() {
        let mut feature = CompressEmptyLines::new();
        let result = feature.apply_feature("");
        assert_eq!(result, Some("".to_string()));
    }

    /// Tests the `LineWithGivenText` feature when the search text is found.
    /// Verifies that the line containing the search text is returned.
    #[test]
    fn search_plain_text_found() {
        let mut feature = LineWithGivenText::new("aditya", false);
        assert_eq!(
            feature.apply_feature("This is a line with aditya in it."),
            Some("This is a line with aditya in it.".to_string())
        );
    }

    /// Tests the `LineWithGivenText` feature when the search text is not found.
    /// Ensures that `None` is returned when the search text is not present in the line.
    #[test]
    fn search_plain_text_not_found() {
        let mut feature = LineWithGivenText::new("nonexistent", false);
        assert!(feature
            .apply_feature("This line does not contain the search text.")
            .is_none());
    }

    /// Tests the `LineWithGivenText` feature with a regex pattern for a single digit.
    /// Verifies that the line containing a single digit is returned.
    #[test]
    fn search_regex_single_digit_found() {
        let mut feature = LineWithGivenText::new("reg:\\d", false);
        assert_eq!(
            feature.apply_feature("This line has a 1 digit."),
            Some("This line has a 1 digit.".to_string())
        );
    }

    /// Tests the `LineWithGivenText` feature with a regex pattern for a single digit.
    /// Ensures that `None` is returned when no digits are found in the line.
    #[test]
    fn search_regex_single_digit_not_found() {
        let mut feature = LineWithGivenText::new("reg:\\d", false);
        assert!(feature.apply_feature("No digits here.").is_none());
    }

    /// Tests the `LineWithGivenText` feature with an exact string match.
    /// Verifies that the line with an exact match of the search text is returned.
    #[test]
    fn search_regex_exact_string() {
        let mut feature = LineWithGivenText::new("aditya", false);
        assert_eq!(
            feature.apply_feature("Exact match aditya"),
            Some("Exact match aditya".to_string())
        );
    }

    /// Tests the `LineWithGivenText` feature with special characters in the regex pattern.
    /// Ensures that the line containing the special characters is returned.
    #[test]
    fn search_regex_special_characters() {
        let mut feature = LineWithGivenText::new("reg:\\[aditya\\]", false);
        assert_eq!(
            feature.apply_feature("Line with [aditya]"),
            Some("Line with [aditya]".to_string())
        );
    }

    /// Tests the `paginate_output` function with a small number of lines.
    /// Verifies that all lines are present in the output and the pagination prompt is not displayed.
    #[test]
    fn pagination_with_few_lines() {
        let lines = (1..10).map(|i| i.to_string()).collect::<Vec<String>>();
        let mut output = Vec::new();
        paginate_output(lines, &mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();

        // Check that all lines are present in the output
        for i in 1..10 {
            assert!(output_str.contains(&i.to_string()));
        }

        // Ensure the pagination prompt does not appear
        assert!(!output_str.contains("--More--"));
    }

    /// Tests the application of a feature on an empty input line.
    /// Ensures that the feature is applied correctly to an empty line.
    #[test]
    fn feature_application_on_empty_input() {
        let mut feature = DollarSymbolAtLast::new();
        let result = feature.apply_feature("");
        assert_eq!(result, Some("$".to_string()));
    }

    /// Tests the resetting behavior of the `LineNumbering` feature.
    /// Verifies that the line numbering starts from 1 when processing a new input source.
    #[test]
    fn line_numbering_resets() {
        let mut feature = LineNumbering::new();
        feature.apply_feature("First line");
        feature.apply_feature("Second line");

        // Simulate processing a new input source by creating a new instance
        let mut feature_new = LineNumbering::new();
        let result = feature_new.apply_feature("New first line");
        assert_eq!(result, Some("1 New first line".to_string()));
    }

    /// Tests the `LineWithGivenText` feature with a regex pattern.
    /// Ensures that lines matching the regex pattern are returned.
    #[test]
    fn search_feature_with_regex() {
        let mut feature = LineWithGivenText::new("reg:\\d+", false); // Matches any digit
        let line_with_number = feature.apply_feature("This is line 42");
        let line_without_number = feature.apply_feature("This line has no numbers");

        assert_eq!(line_with_number, Some("This is line 42".to_string()));
        assert!(line_without_number.is_none());
    }

    /// Tests the `Base64::encode` function.
    /// Verifies that the input text is correctly encoded using Base64.
    #[test]
    fn test_encode() {
        let text = "Hello, world!";
        let encoded = Base64::encode(text);
        assert_eq!(encoded, Some("SGVsbG8sIHdvcmxkIQ==".to_string()));
    }

    /// Tests the `Base64::decode` function.
    /// Ensures that the Base64 encoded text is correctly decoded.
    #[test]
    fn test_decode() {
        let encoded = "SGVsbG8sIHdvcmxkIQ==";
        let decoded = Base64::decode(encoded);
        assert_eq!(decoded, Some("Hello, world!".to_string()));
    }

    /// Tests the `Base64::encode` function with an empty string.
    /// Verifies that encoding an empty string results in an empty string.
    #[test]
    fn test_encode_empty_string() {
        let text = "";
        let encoded = Base64::encode(text);
        assert_eq!(encoded, Some("".to_string()));
    }

    /// Tests the `Base64::decode` function with an invalid Base64 string.
    /// Ensures that decoding an invalid Base64 string returns `None`.
    #[test]
    fn test_decode_invalid_base64() {
        let encoded = "InvalidBase64==";
        let decoded = Base64::decode(encoded);
        assert!(decoded.is_none());
    }

    /// Tests the basic functionality of case-insensitive search.
    /// Verifies that the search is performed case-insensitively.
    #[test]
    fn search_case_insensitive_basic() {
        // Test basic case-insensitive search functionality
        let mut feature = LineWithGivenText::new("aditya", true);
        assert_eq!(
            feature.apply_feature("This line contains ADITYA."),
            Some("This line contains ADITYA.".to_string())
        );
    }

    /// Tests case-insensitive search with mixed-case text.
    /// Ensures that the search matches text regardless of the case.
    #[test]
    fn search_case_insensitive_mixed_case() {
        // Test case-insensitive search with mixed-case text
        let mut feature = LineWithGivenText::new("OpenSource", true);
        assert_eq!(
            feature.apply_feature("I love opensource projects."),
            Some("I love opensource projects.".to_string())
        );
    }

    /// Tests case-sensitive search when the search text is not found.
    /// Verifies that `None` is returned when the search text is not present in the line.
    #[test]
    fn search_case_insensitive_not_found() {
        // Test case-sensitive search when the search text is present
        let mut feature = LineWithGivenText::new("Rust", false);
        assert!(feature
            .apply_feature("I enjoy programming in rust.")
            .is_none());
    }
}
