//! Test tags

mod common;

use common::*;

#[test]
/// Test tags getting.
fn test_get_tags() {
    let client = test_client().lock().unwrap();
    let response = client.get("/api/tags").dispatch();

    let value = response_json_value(response);
    value.get("tags").expect("must have 'tags' field");
}
