use ip_api_client::{generate_empty_config, IpApiError};

#[tokio::test]
async fn make_request_with_invalid_query() {
    let error = generate_empty_config()
        .make_request("1.1.1.one")
        .await
        .expect_err("Failed to get an error from the made request");

    assert_eq!(error, IpApiError::InvalidQuery);
}

#[tokio::test]
async fn make_request_with_private_range() {
    let error = generate_empty_config()
        .make_request("10.0.0.1")
        .await
        .expect_err("Failed to get an error from the made request");

    assert_eq!(error, IpApiError::PrivateRange);
}

#[tokio::test]
async fn make_request_with_reserved_range() {
    let error = generate_empty_config()
        .make_request("127.0.0.1")
        .await
        .expect_err("Failed to get an error from the made request");

    assert_eq!(error, IpApiError::ReservedRange);
}
