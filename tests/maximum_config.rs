use ip_api_client::generate_maximum_config;

#[tokio::test]
async fn make_maximum_request() {
    let response = generate_maximum_config()
        .make_request("1.1.1.1")
        .await
        .expect("Failed to make a request");

    assert!(response.continent.is_some());
    assert!(response.continent_code.is_some());
    assert!(response.country.is_some());
    assert!(response.country_code.is_some());
    assert!(response.region.is_some());
    assert!(response.region_name.is_some());
    assert!(response.city.is_some());
    assert!(response.district.is_some());
    assert!(response.zip.is_some());
    assert!(response.lat.is_some());
    assert!(response.lon.is_some());
    assert!(response.timezone.is_some());
    assert!(response.offset.is_some());
    assert!(response.currency.is_some());
    assert!(response.isp.is_some());
    assert!(response.org.is_some());
    assert!(response.as_field.is_some());
    assert!(response.asname.is_some());
    assert!(response.reverse.is_some());
    assert!(response.mobile.is_some());
    assert!(response.proxy.is_some());
    assert!(response.hosting.is_some());
    assert!(response.query.is_some());
}

#[tokio::test]
async fn make_all_excluded_request() {
    let response = generate_maximum_config()
        .exclude_continent()
        .exclude_continent_code()
        .exclude_country()
        .exclude_country_code()
        .exclude_region()
        .exclude_region_name()
        .exclude_city()
        .exclude_district()
        .exclude_zip()
        .exclude_lat()
        .exclude_lon()
        .exclude_timezone()
        .exclude_offset()
        .exclude_currency()
        .exclude_isp()
        .exclude_org()
        .exclude_as_field()
        .exclude_asname()
        .exclude_reverse()
        .exclude_mobile()
        .exclude_proxy()
        .exclude_hosting()
        .exclude_query()
        .make_request("1.1.1.1")
        .await
        .expect("Failed to make a request");

    assert!(response.continent.is_none());
    assert!(response.continent_code.is_none());
    assert!(response.country.is_none());
    assert!(response.country_code.is_none());
    assert!(response.region.is_none());
    assert!(response.region_name.is_none());
    assert!(response.city.is_none());
    assert!(response.district.is_none());
    assert!(response.zip.is_none());
    assert!(response.lat.is_none());
    assert!(response.lon.is_none());
    assert!(response.timezone.is_none());
    assert!(response.offset.is_none());
    assert!(response.currency.is_none());
    assert!(response.isp.is_none());
    assert!(response.org.is_none());
    assert!(response.as_field.is_none());
    assert!(response.asname.is_none());
    assert!(response.reverse.is_none());
    assert!(response.mobile.is_none());
    assert!(response.proxy.is_none());
    assert!(response.hosting.is_none());
    assert!(response.query.is_none());
}
