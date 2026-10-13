//! ⚠️ **DO NOT MODIFY THE TESTS** ⚠️
//! The tests stand in for `randomnumberapi.com` with a local server and point the component at
//! it through the `random_api` variable, so they work offline.
#![cfg(test)]

use std::path::Path;
use workshop_harness::{mock_server, request, start};

fn app_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
}

fn setup() {
    mock_server(3100, "[40]\n");
    start(
        app_dir(),
        &[],
        &[("SPIN_VARIABLE_RANDOM_API", "http://127.0.0.1:3100")],
    )
}

#[test]
fn returns_a_single_number() {
    setup();

    let response = request("GET", "/?min=30&max=50", "");
    assert_eq!(response.status, 200);
    let number: f64 = response.body.trim().parse().unwrap();
    assert!((30.0..=50.0).contains(&number));
}

#[test]
fn defaults_apply_without_query() {
    setup();

    let response = request("GET", "/", "");
    assert_eq!(response.status, 200);
    let number: f64 = response.body.trim().parse().unwrap();
    assert!((0.0..=1000.0).contains(&number));
}
