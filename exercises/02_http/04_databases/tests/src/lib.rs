#![cfg(test)]

use serde::Deserialize;
use std::path::Path;
use workshop_harness::{request, start};

fn setup() {
    let app_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    start(app_dir, &["--sqlite", "@migration.sql"], &[])
}

#[derive(Debug, Deserialize)]
struct EvalResponse {
    session: i64,
    result: Option<f64>,
}

fn eval(input: &str, session: Option<i64>) -> EvalResponse {
    let path = match session {
        Some(session) => format!("/?session={session}"),
        None => "/".to_string(),
    };
    let response = request("POST", &path, input);
    assert_eq!(response.status, 200, "{input}: {}", response.body);
    serde_json::from_str(&response.body).unwrap()
}

#[test]
fn simple_arithmetic() {
    setup();
    assert_eq!(eval("2 + 3", None).result, Some(5.0));
    assert_eq!(eval("10 - 4", None).result, Some(6.0));
    assert_eq!(eval("3 * 4", None).result, Some(12.0));
    assert_eq!(eval("15 / 3", None).result, Some(5.0));
}

#[test]
fn precedence() {
    setup();
    assert_eq!(eval("2 + 3 * 4", None).result, Some(14.0));
    assert_eq!(eval("(2 + 3) * 4", None).result, Some(20.0));
}

#[test]
fn assign_same_session() {
    setup();
    let res = eval("a = 10", None);
    assert!(res.result.is_none());
    let res2 = eval("b = 10 * (7 + a)", Some(res.session));
    assert_eq!(res.session, res2.session);
    assert!(res2.result.is_none());
    assert_eq!(eval("b", Some(res.session)).result, Some(170.0));
}

#[test]
fn reassign_same_session() {
    setup();
    let res = eval("a = 10", None);
    eval("a = 3 * (7 + a)", Some(res.session));
    assert_eq!(eval("a", Some(res.session)).result, Some(51.0));
}

#[test]
fn assign_different_sessions() {
    setup();
    let res = eval("a = 10", None);
    let res2 = eval("a = 7", None);
    assert_ne!(res.session, res2.session);

    eval("b = 10 * (7 + a)", Some(res.session));
    eval("b = 10 * (7 + a)", Some(res2.session));

    assert_eq!(eval("b", Some(res.session)).result, Some(170.0));
    assert_eq!(eval("b", Some(res2.session)).result, Some(140.0));
}

#[test]
fn unknown_session_is_a_client_error() {
    setup();
    assert_eq!(request("POST", "/?session=999999999", "1 + 1").status, 404);
}
