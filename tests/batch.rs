use ip_api_client::{generate_empty_config, IpData};

#[tokio::test]
async fn make_batch_request() {
    let ips: Vec<IpData> = generate_empty_config()
        .include_query()
        .make_batch_request(vec!["1.1.1.1", "8.8.8.8"])
        .await
        .unwrap();

    assert_eq!(ips.first().unwrap().query, Some(String::from("1.1.1.1")));
    assert_eq!(ips.last().unwrap().query, Some(String::from("8.8.8.8")))
}
