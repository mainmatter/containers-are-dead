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

fn assign(input: &str) {
    let response = request("POST", "/", input);
    assert_eq!(response.status, 200, "{input}: {}", response.body);
    assert!(response.body.is_empty(), "{input}: {}", response.body);
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
fn assign_and_read() {
    start(app_dir(), &[], &[]);
    assign("a = 10");
    assign("b = 10 * (7 + a)");
    assert_eq!(eval("b"), 170.0);
    assert_eq!(eval("b / a"), 17.0);
}

#[test]
fn reassign() {
    start(app_dir(), &[], &[]);
    assign("c = 10");
    assign("c = 3 * (7 + c)");
    assert_eq!(eval("c"), 51.0);
}

#[test]
fn unknown_variable_is_a_client_error() {
    start(app_dir(), &[], &[]);
    assert_eq!(request("POST", "/", "no_such_variable + 1").status, 400);
}
