//! Unit Tests for ricat : `cargo test` to run all the tests at once
//! for indiviual tests : `cargo test test-name` will run all those tests, which contain test-name
#[cfg(test)]
mod tests {
    use crate::*;

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
        let mut feature = LineWithGivenText::new("\\[aditya\\]");
        assert_eq!(
            feature.apply_feature("Line with [aditya]"),
            Some("Line with [aditya]".to_string())
        );
    }

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

    #[test]
    fn feature_application_on_empty_input() {
        let mut feature = DollarSymbolAtLast::new();
        let result = feature.apply_feature("");
        assert_eq!(result, Some("$".to_string()));
    }

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

    #[test]
    fn search_feature_with_regex() {
        let mut feature = LineWithGivenText::new(r"\d+"); // Matches any digit
        let line_with_number = feature.apply_feature("This is line 42");
        let line_without_number = feature.apply_feature("This line has no numbers");

        assert_eq!(line_with_number, Some("This is line 42".to_string()));
        assert!(line_without_number.is_none());
    }

    #[test]
    fn test_encode() {
        let text = "Hello, world!";
        let encoded = Base64::encode(text);
        assert_eq!(encoded, Some("SGVsbG8sIHdvcmxkIQ==".to_string()));
    }

    #[test]
    fn test_decode() {
        let encoded = "SGVsbG8sIHdvcmxkIQ==";
        let decoded = Base64::decode(encoded);
        assert_eq!(decoded, Some("Hello, world!".to_string()));
    }

    #[test]
    fn test_encode_empty_string() {
        let text = "";
        let encoded = Base64::encode(text);
        assert_eq!(encoded, Some("".to_string()));
    }

    #[test]
    fn test_decode_invalid_base64() {
        let encoded = "InvalidBase64==";
        let decoded = Base64::decode(encoded);
        assert!(decoded.is_none());
    }
}
