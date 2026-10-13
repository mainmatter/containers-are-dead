//! ⚠️ **DO NOT MODIFY THE TESTS** ⚠️
//! `cargo test -p spin_tests` builds and starts the Spin application in the parent directory,
//! then sends requests to it.
#![cfg(test)]

use std::path::Path;
use workshop_harness::{request, start};

fn app_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
}

#[test]
fn hello_world() {
    start(app_dir(), &[], &[]);

    let response = request("GET", "/", "");
    assert_eq!(response.status, 200);
    assert_eq!(response.body, "Hello World!");
}
