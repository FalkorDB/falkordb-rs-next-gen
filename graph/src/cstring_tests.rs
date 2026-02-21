//! Unit tests for CString handling safety checks

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    #[test]
    fn test_cstring_no_null_bytes() {
        // Valid strings without null bytes should work
        assert!(CString::new("hello").is_ok());
        assert!(CString::new("hello world").is_ok());
        assert!(CString::new("").is_ok());
        assert!(CString::new("UTF-8: 你好").is_ok());
    }

    #[test]
    fn test_cstring_with_null_bytes() {
        // Strings with null bytes should fail
        assert!(CString::new("hello\0world").is_err());
        assert!(CString::new("\0").is_err());
        assert!(CString::new("start\0middle\0end").is_err());
    }

    #[test]
    fn test_cstring_error_handling() {
        // Test the pattern we use for error handling
        let test_string = "test\0value";
        let result = CString::new(test_string).unwrap_or_else(|_| {
            CString::new("ERR Invalid string (contains null byte)")
                .expect("hardcoded string is valid")
        });

        assert_eq!(result.to_str().unwrap(), "ERR Invalid string (contains null byte)");
    }

    #[test]
    fn test_cstring_fallback_message() {
        // Verify our fallback error messages don't contain null bytes
        let fallback_messages = vec![
            "ERR Query execution failed (error message contains null byte)",
            "ERR Graph write queue unavailable",
            "Invalid field name '{}': contains null byte",
            "Invalid label name '{}': contains null byte",
            "Invalid query value '{}': contains null byte",
        ];

        for msg in fallback_messages {
            assert!(CString::new(msg).is_ok(), "Fallback message contains null byte: {}", msg);
        }
    }
}
