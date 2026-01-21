/// Integration tests for expression parsing that don't require Redis allocator
///
/// These tests verify that the iterative `parse_expr` implementation works correctly
/// for expressions that can be parsed without function resolution.
use graph::cypher::Parser;
use graph::runtime::functions::init_functions;
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize functions once for all tests
fn setup() {
    INIT.call_once(|| {
        let _ = init_functions();
    });
}

#[test]
fn test_simple_literals() {
    // These should parse without needing Redis context
    let expressions = vec![
        "1", "123", "-456", "0.5", "3.14", "'hello'", "true", "false", "null",
    ];

    for expr in expressions {
        let query = format!("RETURN {expr}");
        let mut parser = Parser::new(&query);
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse simple literal: '{}'\nError: {:?}",
            expr,
            result.err()
        );
    }
}

#[test]
fn test_arithmetic_without_functions() {
    let expressions = vec![
        "1 + 2",
        "1 - 2",
        "2 * 3",
        "10 / 2",
        "9 % 5",
        "2^3",
        "1 + 2 * 3",
        "(1 + 2) * 3",
        "-5 + 2",
        "-1 < 1",
    ];

    for expr in expressions {
        let query = format!("RETURN {expr}");
        let mut parser = Parser::new(&query);
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse arithmetic: '{}'\nError: {:?}",
            expr,
            result.err()
        );
    }
}

#[test]
fn test_comparisons() {
    let expressions = vec![
        "1 = 1",
        "1 <> 2",
        "1 < 2",
        "1 <= 2",
        "1 > 0",
        "2 >= 1",
        "10 >= 1.5",
        "true = 5",
        "true <> 'str'",
    ];

    for expr in expressions {
        let query = format!("RETURN {expr}");
        let mut parser = Parser::new(&query);
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse comparison: '{}'\nError: {:?}",
            expr,
            result.err()
        );
    }
}

#[test]
fn test_boolean_operators() {
    let expressions = vec![
        "true AND false",
        "true OR false",
        "NOT true",
        "true XOR false",
        "(true AND false) OR true",
        "NOT (true AND false)",
    ];

    for expr in expressions {
        let query = format!("RETURN {expr}");
        let mut parser = Parser::new(&query);
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse boolean: '{}'\nError: {:?}",
            expr,
            result.err()
        );
    }
}

#[test]
fn test_lists() {
    let expressions = vec![
        "[]",
        "[1]",
        "[1, 2, 3]",
        "['a', 'b']",
        "[true, false]",
        "[1,2,3][0]",
        "[1,2,3][-1]",
        "[1,2,3][0..2]",
        "[1,2,3][..2]",
        "[1,2,3][1..]",
    ];

    for expr in expressions {
        let query = format!("RETURN {expr}");
        let mut parser = Parser::new(&query);
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse list: '{}'\nError: {:?}",
            expr,
            result.err()
        );
    }
}

#[test]
fn test_maps() {
    let expressions = vec![
        "{}",
        "{a: 1}",
        "{a: 1, b: 2}",
        "{name: 'John', age: 30}",
        "{x: 1, y: 2, z: 3}",
    ];

    for expr in expressions {
        let query = format!("RETURN {expr}");
        let mut parser = Parser::new(&query);
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse map: '{}'\nError: {:?}",
            expr,
            result.err()
        );
    }
}

#[test]
fn test_parentheses() {
    let expressions = vec!["(1)", "((1))", "(1 + 2)", "((1 + 2) * 3)", "(a + b) * c"];

    for expr in expressions {
        let query = format!("RETURN {expr}");
        let mut parser = Parser::new(&query);
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse parentheses: '{}'\nError: {:?}",
            expr,
            result.err()
        );
    }
}

#[test]
fn test_property_access() {
    setup();
    let expressions = vec!["n.name", "n.age", "a.val", "x.y.z"];

    for expr in expressions {
        let query = format!("RETURN {expr}");
        let mut parser = Parser::new(&query);
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse property access: '{}'\nError: {:?}",
            expr,
            result.err()
        );
    }
}

#[test]
fn test_null_checks() {
    setup();
    let expressions = vec!["x IS NULL", "x IS NOT NULL", "null IS NULL"];

    for expr in expressions {
        let query = format!("RETURN {expr}");
        let mut parser = Parser::new(&query);
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse null check: '{}'\nError: {:?}",
            expr,
            result.err()
        );
    }
}

#[test]
fn test_string_predicates() {
    let expressions = vec![
        "'hello' STARTS WITH 'he'",
        "'hello' ENDS WITH 'lo'",
        "'hello' CONTAINS 'll'",
        "3 IN [1,2,3]",
    ];

    for expr in expressions {
        let query = format!("RETURN {expr}");
        let mut parser = Parser::new(&query);
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse string predicate: '{}'\nError: {:?}",
            expr,
            result.err()
        );
    }
}

#[test]
fn test_operator_precedence() {
    let expressions = vec![
        "1 + 2 * 3",       // Should parse as 1 + (2 * 3)
        "2 * 3 + 1",       // Should parse as (2 * 3) + 1
        "1 = 2 AND 3 = 4", // Comparison before AND
        "NOT 1 = 2",       // NOT after comparison (NOT applies to whole expression)
        "1 + 2 = 3",       // Addition before comparison
        "1 < 2 AND 2 < 3", // Multiple comparisons with AND
    ];

    for expr in expressions {
        let query = format!("RETURN {expr}");
        let mut parser = Parser::new(&query);
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse precedence: '{}'\nError: {:?}",
            expr,
            result.err()
        );
    }
}

#[test]
fn test_complex_expressions() {
    setup();
    let expressions = vec![
        "1 + 2 * 3 - 4 / 2",
        "(1 + 2) * (3 - 4)",
        "-n.v + 5",
        "NOT (x > 5 AND y < 10)",
        "[1,2,3][0] + [4,5,6][1]",
    ];

    for expr in expressions {
        let query = format!("RETURN {expr}");
        let mut parser = Parser::new(&query);
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Failed to parse complex expression: '{}'\nError: {:?}",
            expr,
            result.err()
        );
    }
}

#[test]
fn test_integer_overflow() {
    setup();

    // These should fail with integer overflow errors
    let overflow_cases = vec![
        "RETURN 10000000000000000000000",
        "RETURN -10000000000000000000000",
        "RETURN 9223372036854775808",
        "RETURN -9223372036854775809",
    ];

    for query in overflow_cases {
        let mut parser = Parser::new(query);
        let result = parser.parse();
        assert!(
            result.is_err(),
            "Expected overflow error for query: '{query}', but got success"
        );
        let err = result.err().unwrap();
        assert!(
            err.contains("Integer overflow"),
            "Expected 'Integer overflow' error for query: '{query}', but got: '{err}'"
        );
    }

    // These should succeed - valid i64 values
    let valid_cases = vec![
        ("RETURN -9223372036854775808", -9_223_372_036_854_775_808_i64), // i64::MIN
        ("RETURN 9223372036854775807", 9_223_372_036_854_775_807_i64),   // i64::MAX
        ("RETURN 0", 0_i64),
    ];

    for (query, _expected) in valid_cases {
        let mut parser = Parser::new(query);
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Expected success for query: '{}', but got error: {:?}",
            query,
            result.err()
        );
    }
}
