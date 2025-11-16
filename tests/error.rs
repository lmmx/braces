use braces::{brace_paths, BraceConfig, BraceError};

#[test]
fn test_error_display() {
    let empty_err = BraceError::EmptyInput;
    assert_eq!(format!("{}", empty_err), "No paths provided");

    let mixed_err = BraceError::MixedSeparators {
        found: vec!["\\".to_string()],
        expected: "/".to_string(),
    };
    assert!(format!("{}", mixed_err).contains("Mixed path separators"));

    let brace_err = BraceError::InvalidBraceInput {
        path: "foo/{bar}".to_string(),
        reason: "test".to_string(),
    };
    assert!(format!("{}", brace_err).contains("Invalid braces input"));

    let depth_err = BraceError::DepthLimitExceeded { limit: 5 };
    assert!(format!("{}", depth_err).contains("depth limit"));
}

#[test]
fn test_error_is_error_trait() {
    let err = BraceError::EmptyInput;
    let _: &dyn std::error::Error = &err;
}

#[test]
fn test_result_type() {
    let paths: Vec<String> = vec![];
    let result = brace_paths(&paths, &BraceConfig::default());
    assert!(result.is_err());
}

#[test]
fn test_error_equality() {
    let err1 = BraceError::EmptyInput;
    let err2 = BraceError::EmptyInput;
    assert_eq!(err1, err2);

    let err3 = BraceError::DepthLimitExceeded { limit: 5 };
    let err4 = BraceError::DepthLimitExceeded { limit: 5 };
    assert_eq!(err3, err4);
}

#[test]
fn test_error_clone() {
    let err = BraceError::EmptyInput;
    let cloned = err.clone();
    assert_eq!(err, cloned);
}
