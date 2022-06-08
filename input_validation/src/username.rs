use lazy_static::lazy_static;
use regex::Regex;

static REGEX_ALPHABETIC: &str = r"[[:alpha:]]+";
static REGEX_USERNAME: &str = r"[[[:alnum:]]-_]{4,64}";

/// Validate a username
/// Accept only characters alphanumeric and - _
/// Must contain 1 alphabetic character
/// Min length: 4
/// Max length: 64
/// # Arguments
/// * `username_input` - username to validate
/// # Returns
/// * `bool` - True if the username is valid, false otherwise
pub fn validate_username(username_input: &str) -> bool {
    lazy_static! {
        static ref RE_ALPHA: Regex = Regex::new(&format!("{}", REGEX_ALPHABETIC)).unwrap();
        static ref RE: Regex = Regex::new(&format!("^{}$", REGEX_USERNAME)).unwrap();
    }
    RE.is_match(username_input) && RE_ALPHA.is_match(username_input)
}

#[cfg(test)]
mod tests {
    use super::validate_username;

    #[test]
    fn validate_username_length() {
        // Pass
        assert!(validate_username("teeeeest")); // 8

        // Fail
        assert!(!validate_username("te")); // 2
        assert!(!validate_username("teeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\
        eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeest")); // 100

        // Corner cases
        assert!(validate_username("test")); // 4
        assert!(validate_username("teeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\
        eeeeeeeeeeeest")); // 64

        assert!(!validate_username("tes")); // 3
        assert!(!validate_username("teeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\
        eeeeeeeeeeeeest")); // 65

    }

    #[test]
    fn validate_username_characters() {
        // Pass
        assert!(validate_username("teeeeest"));
        assert!(validate_username("TEEEEEST"));
        assert!(validate_username("TEEEeest"));
        assert!(validate_username("test1234"));
        assert!(validate_username("TEST1234"));
        assert!(validate_username("TEst1234"));
        assert!(validate_username("TEst-234"));
        assert!(validate_username("TEst_234"));
        assert!(validate_username("TEst-2_4"));

        // Corner cases & Fail
        assert!(!validate_username("12345678"));
        assert!(!validate_username("test123$"));
        assert!(!validate_username("test123<"));
        assert!(!validate_username("test123 "));
    }
}