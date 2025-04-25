use ip_api_client::generate_empty_config;

#[tokio::test]
async fn make_request_with_invalid_query() {
    let response = generate_empty_config().make_request("1.1.1.one").await;

    assert!(response.is_err());
}

#[tokio::test]
async fn make_request_with_private_range() {
    let response = generate_empty_config().make_request("10.0.0.1").await;

    assert!(response.is_err());
}

#[tokio::test]
async fn make_request_with_reversed_range() {
    let response = generate_empty_config().make_request("127.0.0.1").await;

    assert!(response.is_err());
}
