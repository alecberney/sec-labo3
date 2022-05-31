use lazy_static::lazy_static;
use regex::Regex;

static REGEX_PHONE_NUMBER: &str = r"(0|00 ?|\+41 ?)\d{2} ?\d{3} ?\d{2} ?\d{2}";

// No semantical validation will be done
// But it could be done by generating a random pin and sending it by SMS for example

/// Validate a phone number for swiss format
/// Accepted format:
/// 011 222 33 44
/// 00 11 222 33 44
/// +41 11 222 33 44
/// 0112223344
/// 00112223344
/// +41112223344
/// # Arguments
/// * `phone_number_input` - phone number to validate
/// # Returns
/// * `bool` - True if the phone number is valid, false otherwise
pub fn validate_phone_number(phone_number_input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(&format!("^{}$", REGEX_PHONE_NUMBER)).unwrap();
    }
    RE.is_match(phone_number_input)
}

#[cfg(test)]
mod tests {
    use super::validate_phone_number;

    #[test]
    fn validate_phone_number_classic() {
        // Pass
        // With spaces
        assert!(validate_phone_number("011 222 33 44"));
        assert!(validate_phone_number("00 11 222 33 44"));
        assert!(validate_phone_number("+41 11 222 33 44"));

        // Without spaces
        assert!(validate_phone_number("0112223344"));
        assert!(validate_phone_number("00112223344"));
        assert!(validate_phone_number("+41112223344"));
    }

    #[test]
    fn validate_phone_number_without_length() {
        // Corner cases & Fail
        assert!(!validate_phone_number("011 222 33 444")); // + 1
        assert!(!validate_phone_number("011 222 33 4")); // - 1

        assert!(!validate_phone_number("00 11 222 33 444")); // + 1
        assert!(!validate_phone_number("00 11 222 33 4")); // - 1

        assert!(!validate_phone_number("+41 11 222 33 444")); // + 1
        assert!(!validate_phone_number("+41 11 222 33 4")); // - 1

        // Test each group
        assert!(!validate_phone_number("011 222 333 44"));
        assert!(!validate_phone_number("011 222 3 44"));

        assert!(!validate_phone_number("011 2222 33 44"));
        assert!(!validate_phone_number("011 22 33 44"));

        assert!(!validate_phone_number("0111 222 33 44"));
        assert!(!validate_phone_number("01 222 33 44"));
    }

    #[test]
    fn validate_phone_number_without_characters() {
        // Corner cases & Fail
        assert!(!validate_phone_number("0a1 222 33 44"));
        assert!(!validate_phone_number("00 11 A22 33 44"));
        assert!(!validate_phone_number("+41 11 222 $3 44"));
        assert!(!validate_phone_number("011 222 33 >4"));
    }
}