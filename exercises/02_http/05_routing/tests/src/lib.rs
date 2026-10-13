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
        Some(session) => format!("/eval?session={session}"),
        None => "/eval".to_string(),
    };
    let response = request("POST", &path, input);
    assert_eq!(response.status, 200, "{input}: {}", response.body);
    serde_json::from_str(&response.body).unwrap()
}

fn eval_status(input: &str, session: i64) -> u16 {
    request("POST", &format!("/eval?session={session}"), input).status
}

#[test]
fn eval_still_works() {
    setup();
    let res = eval("a = 10", None);
    assert!(res.result.is_none());
    assert_eq!(eval("a * 2", Some(res.session)).result, Some(20.0));
}

#[test]
fn delete_one_variable() {
    setup();
    let session = eval("a = 10", None).session;
    eval("b = 11", Some(session));

    let response = request("DELETE", &format!("/clear/a?session={session}"), "");
    assert_eq!(response.status, 200, "{}", response.body);

    assert_eq!(eval_status("a", session), 400);
    assert_eq!(eval("b", Some(session)).result, Some(11.0));
}

#[test]
fn clear_all_variables() {
    setup();
    let session = eval("a = 10", None).session;
    eval("b = 11", Some(session));
    let other = eval("a = 99", None).session;

    let response = request("DELETE", &format!("/clear?session={session}"), "");
    assert_eq!(response.status, 200, "{}", response.body);

    assert_eq!(eval_status("a", session), 400);
    assert_eq!(eval_status("b", session), 400);
    assert_eq!(eval("a", Some(other)).result, Some(99.0));
}

#[test]
fn session_is_required() {
    setup();
    assert_eq!(request("DELETE", "/clear/a", "").status, 400);
    assert_eq!(request("DELETE", "/clear", "").status, 400);
}

#[test]
fn unknown_routes_are_not_found() {
    setup();
    assert_eq!(request("GET", "/clear/a/b", "").status, 404);
}
