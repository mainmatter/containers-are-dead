//! ⚠️ **DO NOT MODIFY THE TESTS** ⚠️
#![cfg(test)]

use std::path::Path;
use workshop_harness::{request, start};

fn app_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
}

fn eval(input: &str) -> f64 {
    let response = request("POST", "/", input);
    assert_eq!(response.status, 200, "{input}: {}", response.body);
    response.body.trim().parse().unwrap()
}

#[test]
fn simple_arithmetic() {
    start(app_dir(), &[], &[]);
    assert_eq!(eval("2 + 3"), 5.0);
    assert_eq!(eval("10 - 4"), 6.0);
    assert_eq!(eval("3 * 4"), 12.0);
    assert_eq!(eval("15 / 3"), 5.0);
}

#[test]
fn precedence() {
    start(app_dir(), &[], &[]);
    assert_eq!(eval("2 + 3 * 4"), 14.0);
    assert_eq!(eval("(2 + 3) * 4"), 20.0);
}

#[test]
fn negative_numbers() {
    start(app_dir(), &[], &[]);
    assert_eq!(eval("-5 + 3"), -2.0);
    assert_eq!(eval("10 + -5"), 5.0);
}

#[test]
fn maaany_parens() {
    start(app_dir(), &[], &[]);
    assert_eq!(eval("((((((((8))))))))"), 8.0);
}

#[test]
fn only_post_is_allowed() {
    start(app_dir(), &[], &[]);
    assert_eq!(request("GET", "/", "").status, 405);
}

#[test]
fn invalid_input_is_a_client_error() {
    start(app_dir(), &[], &[]);
    assert_eq!(request("POST", "/", "2 +").status, 400);
    assert_eq!(request("POST", "/", "(2 + 3").status, 400);
    assert_eq!(request("POST", "/", "2 $ 3").status, 400);
}
