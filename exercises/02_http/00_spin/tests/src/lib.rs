use spin_test_sdk::{bindings::wasi::http, spin_test};

#[spin_test]
fn send_get_request_without_key() {
    // Perform the request
    let request = http::types::OutgoingRequest::new(http::types::Headers::new());
    request.set_path_with_query(Some("/")).unwrap();
    let response = spin_test_sdk::perform_request(request);

    // Assert response status is 200
    assert_eq!(response.status(), 200);

    // Assert the body is "Hello World!"
    assert_eq!(
        String::from_utf8(response.body().unwrap()).unwrap(),
        "Hello World!"
    )
}
