use ip_api_client::generate_minimum_config;

#[tokio::test]
async fn make_minimum_request() {
    let response = generate_minimum_config()
        .make_request("1.1.1.1")
        .await
        .unwrap();

    assert!(response.continent.is_none());
    assert!(response.continent_code.is_none());
    assert!(response.country.is_none());
    assert!(response.country_code.is_some());
    assert!(response.region.is_none());
    assert!(response.region_name.is_none());
    assert!(response.city.is_some());
    assert!(response.district.is_none());
    assert!(response.zip.is_none());
    assert!(response.lat.is_none());
    assert!(response.lon.is_none());
    assert!(response.timezone.is_some());
    assert!(response.offset.is_some());
    assert!(response.currency.is_some());
    assert!(response.isp.is_some());
    assert!(response.org.is_none());
    assert!(response.as_field.is_none());
    assert!(response.asname.is_none());
    assert!(response.reverse.is_none());
    assert!(response.mobile.is_none());
    assert!(response.proxy.is_none());
    assert!(response.hosting.is_none());
    assert!(response.query.is_none());
}
