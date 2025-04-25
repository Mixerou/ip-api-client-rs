use ip_api_client::generate_empty_config;

#[tokio::test]
async fn make_empty_request() {
    let response = generate_empty_config()
        .make_request("1.1.1.1")
        .await
        .unwrap();

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

#[tokio::test]
async fn make_all_included_request() {
    let response = generate_empty_config()
        .include_continent()
        .include_continent_code()
        .include_country()
        .include_country_code()
        .include_region()
        .include_region_name()
        .include_city()
        .include_district()
        .include_zip()
        .include_lat()
        .include_lon()
        .include_timezone()
        .include_offset()
        .include_currency()
        .include_isp()
        .include_org()
        .include_as_field()
        .include_asname()
        .include_reverse()
        .include_mobile()
        .include_proxy()
        .include_hosting()
        .include_query()
        .make_request("1.1.1.1")
        .await
        .unwrap();

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
