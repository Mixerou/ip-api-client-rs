//! The client (based on [ip-api.com API](https://ip-api.com/docs/api:json))
//! allows you to get information about the IP address
//!
//! # Example
//!
//! ```rust
//! use ip_api_client as Client;
//! use ip_api_client::{IpApiLanguage, IpData};
//!
//! // You can
//! // `generate_empty_config` (to create your own config from scratch)
//! // `generate_minimum_config` (that includes only important fields)
//! // `generate_maximum_config` (that includes all fields)
//! let ip_data: IpData = Client::generate_empty_config()
//!     // or `exclude_country` if this field is already included
//!     // in the generated config
//!     .include_country()
//!     // or `exclude_currency` if this field is already included in
//!     // the generated config
//!     .include_currency()
//!     // available languages: de/en (default)/es/fr/ja/pt-Br/ru/zh-CN
//!     .set_language(IpApiLanguage::De)
//!     // `make_request` takes
//!     // "ip"/"domain"/"empty string (if you want to request your ip)"
//!     .make_request("1.1.1.1").unwrap();
//!
//! println!(
//!     "{}'s national currency is {}",
//!     ip_data.country.unwrap(),
//!     ip_data.currency.unwrap(),
//! );
//!
//! // If you want to request more than one ip, you can use `make_batch_request`
//! let ip_batch_data: Vec<IpData> = Client::generate_empty_config()
//!     .include_isp()
//! // `make_batch_request` takes "IPv4"/"IPv6"
//! .make_batch_request(vec!["1.1.1.1", "8.8.8.8"]).unwrap();
//!
//! println!(
//!     "1.1.1.1 belongs to `{}` and 8.8.8.8 belongs to `{}`",
//!     ip_batch_data.get(0).unwrap().isp.as_ref().unwrap(),
//!     ip_batch_data.get(1).unwrap().isp.as_ref().unwrap(),
//! );
//! ```

#![deny(missing_docs)]

use hyper::body::HttpBody;
use hyper::{Body, Client, Method, Request, Response};
use serde::Deserialize;
use serde_json::json;

#[cfg(test)]
mod tests {
    use crate::{generate_empty_config, IpData};

    #[test]
    fn make_request() {
        assert_eq!(
            generate_empty_config()
                .include_query()
                .make_request("1.1.1.1").unwrap()
                .query.unwrap(),
            String::from("1.1.1.1")
        );
    }

    #[test]
    fn make_batch_request() {
        let ips: Vec<IpData> = generate_empty_config()
            .include_query()
            .make_batch_request(vec!["1.1.1.1", "8.8.8.8"]).unwrap();

        assert_eq!(ips.get(0).unwrap().query, Some(String::from("1.1.1.1")));
        assert_eq!(ips.get(1).unwrap().query, Some(String::from("8.8.8.8")))
    }
}

/// Represents all the ways that a request can fail
#[derive(Debug)]
pub enum IpApiError {
    /// Incorrect IP address or non-existent domain
    ///
    /// # Example
    ///
    /// 1.1.1.one **OR** test.google.com
    InvalidQuery,

    /// IPs in your network
    ///
    /// # Example
    ///
    /// 192.168.1.1
    PrivateRange,

    /// [ip-api.com API](https://ip-api.com/docs/api:json) is limited to 45 requests per minute
    /// from one IP address
    ///
    /// Contains the remaining time before a possible re-request in seconds
    RateLimit(u8),

    /// Reserved Range
    ///
    /// # Example
    ///
    /// 127.0.0.1 **OR** localhost
    ReservedRange,

    /// Unexpected Error
    ///
    /// May contain additional information
    UnexpectedError(Option<String>),
}

/// Represents all available languages for [`IpData`]
pub enum IpApiLanguage {
    /// Deutsch (German)
    De,

    /// English (default)
    En,

    /// Español (Spanish)
    Es,

    /// Français (French)
    Fr,

    /// 日本語 (Japanese)
    Ja,

    /// Português - Brasil (Portuguese - Brasil)
    PtBr,

    /// Русский (Russian)
    Ru,

    /// 中国 (Chinese)
    ZhCn,
}

#[derive(Deserialize)]
struct IpApiMessage {
    message: Option<String>,
}

/// The data that will be received after the making a request
///
/// # Example response
///
/// ```rust
/// # use ip_api_client::IpData;
/// #
/// IpData {
///     continent: Some("Oceania".to_string()),
///     continent_code: Some("OC".to_string()),
///     country: Some("Australia".to_string()),
///     country_code: Some("AU".to_string()),
///     region: Some("QLD".to_string()),
///     region_name: Some("Queensland".to_string()),
///     city: Some("South Brisbane".to_string()),
///     district: Some("".to_string()),
///     zip: Some("4101".to_string()),
///     lat: Some(-27.4766),
///     lon: Some(153.0166),
///     timezone: Some("Australia/Brisbane".to_string()),
///     offset: Some(36000),
///     currency: Some("AUD".to_string()),
///     isp: Some("Cloudflare, Inc".to_string()),
///     org: Some("APNIC and Cloudflare DNS Resolver project".to_string()),
///     as_field: Some("AS13335 Cloudflare, Inc.".to_string()),
///     asname: Some("CLOUDFLARENET".to_string()),
///     reverse: Some("one.one.one.one".to_string()),
///     mobile: Some(false),
///     proxy: Some(false),
///     hosting: Some(true),
///     query: Some("1.1.1.1".to_string()),
/// };
/// ```
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpData {
    /// Continent name
    pub continent: Option<String>,

    /// Two-letter continent code
    pub continent_code: Option<String>,

    /// Country name
    pub country: Option<String>,

    /// Two-letter country code
    /// [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
    pub country_code: Option<String>,

    /// Region/state short code (FIPS or ISO)
    pub region: Option<String>,

    /// Region/state
    pub region_name: Option<String>,

    /// City
    pub city: Option<String>,

    /// District (subdivision of city)
    pub district: Option<String>,

    /// Zip code
    pub zip: Option<String>,

    /// Latitude
    pub lat: Option<f32>,

    /// Longitude
    pub lon: Option<f32>,

    /// Timezone (tz)
    pub timezone: Option<String>,

    /// Timezone UTC DST offset in seconds
    pub offset: Option<i32>,

    /// National currency
    pub currency: Option<String>,

    /// ISP name
    pub isp: Option<String>,

    /// Organization name
    pub org: Option<String>,

    /// AS number and organization, separated by space (RIR).
    /// Empty for IP blocks not being announced in BGP tables.
    ///
    /// # Notice
    ///
    /// We use `as_field` instead of `as`
    /// (As stated in the [ip-api.com API documentation](https://ip-api.com/docs/api:json#as))
    /// since it's a
    /// [strict keyword](https://doc.rust-lang.org/reference/keywords.html#strict-keywords) in rust,
    /// such as `pub`, `impl` or `struct`.
    #[serde(rename = "as")]
    pub as_field: Option<String>,

    /// AS name (RIR). Empty for IP blocks not being announced in BGP tables.
    pub asname: Option<String>,

    /// Reverse DNS of the IP (can delay response)
    pub reverse: Option<String>,

    /// Mobile (cellular) connection
    pub mobile: Option<bool>,

    /// Proxy, VPN or Tor exit address
    pub proxy: Option<bool>,

    /// Hosting, colocated or data center
    pub hosting: Option<bool>,

    /// IP/Domain used for the query
    pub query: Option<String>,
}

/// Configuration structure allows you to customize the requested fields in the request
/// to save traffic
pub struct IpApiConfig {
    numeric_field: u32,
    is_continent_included: bool,
    is_continent_code_included: bool,
    is_country_included: bool,
    is_country_code_included: bool,
    is_region_included: bool,
    is_region_name_included: bool,
    is_city_included: bool,
    is_district_included: bool,
    is_zip_included: bool,
    is_lat_included: bool,
    is_lon_included: bool,
    is_timezone_included: bool,
    is_offset_included: bool,
    is_currency_included: bool,
    is_isp_included: bool,
    is_org_included: bool,
    is_as_field_included: bool,
    is_asname_included: bool,
    is_reverse_included: bool,
    is_mobile_included: bool,
    is_proxy_included: bool,
    is_hosting_included: bool,
    is_query_included: bool,
    language: IpApiLanguage,
}

impl IpApiConfig {
    fn build_uri(
        resource: &str,
        target: Option<&str>,
        fields: u32,
        language: IpApiLanguage,
    ) -> String {
        format!("http://ip-api.com/{}/{}?fields={}{}",
                resource,
                target.unwrap_or(""),
                fields,
                match language {
                    IpApiLanguage::De => "&lang=de",
                    IpApiLanguage::Es => "&lang=es",
                    IpApiLanguage::Fr => "&lang=fr",
                    IpApiLanguage::Ja => "&lang=ja",
                    IpApiLanguage::PtBr => "&lang=pt-BR",
                    IpApiLanguage::Ru => "&lang=ru",
                    IpApiLanguage::ZhCn => "&lang=zh-CN",
                    _ => "",
                }
        )
    }

    fn check_response(response: &Response<Body>) -> Result<(), IpApiError> {
        if response.status() == 429 {
            return Err(IpApiError::RateLimit(
                response
                    .headers().get("X-Ttl").unwrap()
                    .to_str().unwrap()
                    .parse().unwrap()
            ));
        }

        Ok(())
    }

    fn check_error_message(message: Option<String>) -> Result<(), IpApiError> {
        if let Some(message) = message {
            return match message.as_str() {
                "invalid query" => Err(IpApiError::InvalidQuery),
                "private range" => Err(IpApiError::PrivateRange),
                "reserved range" => Err(IpApiError::ReservedRange),
                message => Err(IpApiError::UnexpectedError(Some(message.to_string()))),
            };
        }

        Ok(())
    }

    async fn parse_response_body(response: &mut Response<Body>) -> String {
        let body = response.body_mut().data().await;
        let body = body.unwrap().unwrap().to_vec();
        let body = std::str::from_utf8(&body).unwrap();

        body.to_string()
    }

    /// Making a request to [ip-api.com API](https://ip-api.com/docs/api:json)
    ///
    /// `target` can be "ip"/"domain"/"empty string (if you want to request your ip)"
    #[tokio::main]
    pub async fn make_request(self, target: &str) -> Result<IpData, IpApiError> {
        let uri = Self::build_uri(
            "json",
            Some(target),
            self.numeric_field,
            self.language,
        );

        let client = Client::new();
        let response = &mut client.get(uri.parse().unwrap()).await.unwrap();

        Self::check_response(response)?;

        let body = Self::parse_response_body(response).await;
        let ip_data: IpApiMessage = serde_json::from_str(body.as_str()).unwrap();

        Self::check_error_message(ip_data.message)?;

        let ip_data: IpData = serde_json::from_str(body.as_str()).unwrap();

        Ok(ip_data)
    }

    /// Making a batch request to [ip-api.com API](https://ip-api.com/docs/api:batch)
    ///
    /// `target` can be "IPv4"/"IPv6"
    #[tokio::main]
    pub async fn make_batch_request(self, targets: Vec<&str>) -> Result<Vec<IpData>, IpApiError> {
        let uri = Self::build_uri(
            "batch",
            None,
            self.numeric_field,
            self.language,
        );

        let request = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(json!(targets).to_string())).unwrap();

        let client = Client::new();
        let response = &mut client.request(request).await.unwrap();

        Self::check_response(response)?;

        let body = Self::parse_response_body(response).await;
        let ip_batch_data: Vec<IpApiMessage> = serde_json::from_str(body.as_str()).unwrap();

        for ip_data in ip_batch_data {
            Self::check_error_message(ip_data.message)?;
        }

        let ip_batch_data: Vec<IpData> = serde_json::from_str(body.as_str()).unwrap();

        Ok(ip_batch_data)
    }

    /// Include [`continent`](struct.IpData.html#structfield.continent) in request
    pub fn include_continent(mut self) -> Self {
        if !self.is_continent_included {
            self.is_continent_included = true;
            self.numeric_field += 1048576;
        }

        self
    }

    /// Include [`continent_code`](struct.IpData.html#structfield.continent_code) in request
    pub fn include_continent_code(mut self) -> Self {
        if !self.is_continent_code_included {
            self.is_continent_code_included = true;
            self.numeric_field += 2097152;
        }

        self
    }

    /// Include [`country`](struct.IpData.html#structfield.country) in request
    pub fn include_country(mut self) -> Self {
        if !self.is_country_included {
            self.is_country_included = true;
            self.numeric_field += 1;
        }

        self
    }

    /// Include [`country_code`](struct.IpData.html#structfield.country_code) in request
    pub fn include_country_code(mut self) -> Self {
        if !self.is_country_code_included {
            self.is_country_code_included = true;
            self.numeric_field += 2;
        }

        self
    }

    /// Include [`region`](struct.IpData.html#structfield.region) in request
    pub fn include_region(mut self) -> Self {
        if !self.is_region_included {
            self.is_region_included = true;
            self.numeric_field += 4;
        }

        self
    }

    /// Include [`region_name`](struct.IpData.html#structfield.region_name) in request
    pub fn include_region_name(mut self) -> Self {
        if !self.is_region_name_included {
            self.is_region_name_included = true;
            self.numeric_field += 8;
        }

        self
    }

    /// Include [`city`](struct.IpData.html#structfield.city) in request
    pub fn include_city(mut self) -> Self {
        if !self.is_city_included {
            self.is_city_included = true;
            self.numeric_field += 16;
        }

        self
    }

    /// Include [`district`](struct.IpData.html#structfield.district) in request
    pub fn include_district(mut self) -> Self {
        if !self.is_district_included {
            self.is_district_included = true;
            self.numeric_field += 524288;
        }

        self
    }

    /// Include [`zip`](struct.IpData.html#structfield.zip) in request
    pub fn include_zip(mut self) -> Self {
        if !self.is_zip_included {
            self.is_zip_included = true;
            self.numeric_field += 32;
        }

        self
    }

    /// Include [`lat`](struct.IpData.html#structfield.lat) in request
    pub fn include_lat(mut self) -> Self {
        if !self.is_lat_included {
            self.is_lat_included = true;
            self.numeric_field += 64;
        }

        self
    }

    /// Include [`lon`](struct.IpData.html#structfield.lon) in request
    pub fn include_lon(mut self) -> Self {
        if !self.is_lon_included {
            self.is_lon_included = true;
            self.numeric_field += 128;
        }

        self
    }

    /// Include [`timezone`](struct.IpData.html#structfield.timezone) in request
    pub fn include_timezone(mut self) -> Self {
        if !self.is_timezone_included {
            self.is_timezone_included = true;
            self.numeric_field += 256;
        }

        self
    }

    /// Include [`offset`](struct.IpData.html#structfield.offset) in request
    pub fn include_offset(mut self) -> Self {
        if !self.is_offset_included {
            self.is_offset_included = true;
            self.numeric_field += 33554432;
        }

        self
    }

    /// Include [`currency`](struct.IpData.html#structfield.currency) in request
    pub fn include_currency(mut self) -> Self {
        if !self.is_currency_included {
            self.is_currency_included = true;
            self.numeric_field += 8388608;
        }

        self
    }

    /// Include [`isp`](struct.IpData.html#structfield.isp) in request
    pub fn include_isp(mut self) -> Self {
        if !self.is_isp_included {
            self.is_isp_included = true;
            self.numeric_field += 512;
        }

        self
    }

    /// Include [`org`](struct.IpData.html#structfield.org) in request
    pub fn include_org(mut self) -> Self {
        if !self.is_org_included {
            self.is_org_included = true;
            self.numeric_field += 1024;
        }

        self
    }

    /// Include [`as_field`](struct.IpData.html#structfield.as_field) in request
    pub fn include_as_field(mut self) -> Self {
        if !self.is_as_field_included {
            self.is_as_field_included = true;
            self.numeric_field += 2048;
        }

        self
    }

    /// Include [`asname`](struct.IpData.html#structfield.asname) in request
    pub fn include_asname(mut self) -> Self {
        if !self.is_asname_included {
            self.is_asname_included = true;
            self.numeric_field += 4194304;
        }

        self
    }

    /// Include [`reverse`](struct.IpData.html#structfield.reverse) in request
    pub fn include_reverse(mut self) -> Self {
        if !self.is_reverse_included {
            self.is_reverse_included = true;
            self.numeric_field += 4096;
        }

        self
    }

    /// Include [`mobile`](struct.IpData.html#structfield.mobile) in request
    pub fn include_mobile(mut self) -> Self {
        if !self.is_mobile_included {
            self.is_mobile_included = true;
            self.numeric_field += 65536;
        }

        self
    }

    /// Include [`proxy`](struct.IpData.html#structfield.proxy) in request
    pub fn include_proxy(mut self) -> Self {
        if !self.is_proxy_included {
            self.is_proxy_included = true;
            self.numeric_field += 131072;
        }

        self
    }

    /// Include [`hosting`](struct.IpData.html#structfield.hosting) in request
    pub fn include_hosting(mut self) -> Self {
        if !self.is_hosting_included {
            self.is_hosting_included = true;
            self.numeric_field += 16777216;
        }

        self
    }

    /// Include [`query`](struct.IpData.html#structfield.query) in request
    pub fn include_query(mut self) -> Self {
        if !self.is_query_included {
            self.is_query_included = true;
            self.numeric_field += 8192;
        }

        self
    }

    /// Exclude [`continent`](struct.IpData.html#structfield.continent) from request
    pub fn exclude_continent(mut self) -> Self {
        if self.is_continent_included {
            self.is_continent_included = false;
            self.numeric_field -= 1048576;
        }

        self
    }

    /// Exclude [`continent_code`](struct.IpData.html#structfield.continent_code) from request
    pub fn exclude_continent_code(mut self) -> Self {
        if self.is_continent_code_included {
            self.is_continent_code_included = false;
            self.numeric_field -= 2097152;
        }

        self
    }

    /// Exclude [`country`](struct.IpData.html#structfield.country) from request
    pub fn exclude_country(mut self) -> Self {
        if self.is_country_included {
            self.is_country_included = false;
            self.numeric_field -= 1;
        }

        self
    }

    /// Exclude [`country_code`](struct.IpData.html#structfield.country_code) from request
    pub fn exclude_country_code(mut self) -> Self {
        if self.is_country_code_included {
            self.is_country_code_included = false;
            self.numeric_field -= 2;
        }

        self
    }

    /// Exclude [`region`](struct.IpData.html#structfield.region) from request
    pub fn exclude_region(mut self) -> Self {
        if self.is_region_included {
            self.is_region_included = false;
            self.numeric_field -= 4;
        }

        self
    }

    /// Exclude [`region_name`](struct.IpData.html#structfield.region_name) from request
    pub fn exclude_region_name(mut self) -> Self {
        if self.is_region_name_included {
            self.is_region_name_included = false;
            self.numeric_field -= 8;
        }

        self
    }

    /// Exclude [`city`](struct.IpData.html#structfield.city) from request
    pub fn exclude_city(mut self) -> Self {
        if self.is_city_included {
            self.is_city_included = false;
            self.numeric_field -= 16;
        }

        self
    }

    /// Exclude [`district`](struct.IpData.html#structfield.district) from request
    pub fn exclude_district(mut self) -> Self {
        if self.is_district_included {
            self.is_district_included = false;
            self.numeric_field -= 524288;
        }

        self
    }

    /// Exclude [`zip`](struct.IpData.html#structfield.zip) from request
    pub fn exclude_zip(mut self) -> Self {
        if self.is_zip_included {
            self.is_zip_included = false;
            self.numeric_field -= 32;
        }

        self
    }

    /// Exclude [`lat`](struct.IpData.html#structfield.lat) from request
    pub fn exclude_lat(mut self) -> Self {
        if self.is_lat_included {
            self.is_lat_included = false;
            self.numeric_field -= 64;
        }

        self
    }

    /// Exclude [`lon`](struct.IpData.html#structfield.lon) from request
    pub fn exclude_lon(mut self) -> Self {
        if self.is_lon_included {
            self.is_lon_included = false;
            self.numeric_field -= 128;
        }

        self
    }

    /// Exclude [`timezone`](struct.IpData.html#structfield.timezone) from request
    pub fn exclude_timezone(mut self) -> Self {
        if self.is_timezone_included {
            self.is_timezone_included = false;
            self.numeric_field -= 256;
        }

        self
    }

    /// Exclude [`offset`](struct.IpData.html#structfield.offset) from request
    pub fn exclude_offset(mut self) -> Self {
        if self.is_offset_included {
            self.is_offset_included = false;
            self.numeric_field -= 33554432;
        }

        self
    }

    /// Exclude [`currency`](struct.IpData.html#structfield.currency) from request
    pub fn exclude_currency(mut self) -> Self {
        if self.is_currency_included {
            self.is_currency_included = false;
            self.numeric_field -= 8388608;
        }

        self
    }

    /// Exclude [`isp`](struct.IpData.html#structfield.isp) from request
    pub fn exclude_isp(mut self) -> Self {
        if self.is_isp_included {
            self.is_isp_included = false;
            self.numeric_field -= 512;
        }

        self
    }

    /// Exclude [`org`](struct.IpData.html#structfield.org) from request
    pub fn exclude_org(mut self) -> Self {
        if self.is_org_included {
            self.is_org_included = false;
            self.numeric_field -= 1024;
        }

        self
    }

    /// Exclude [`as_field`](struct.IpData.html#structfield.as_field) from request
    pub fn exclude_as_field(mut self) -> Self {
        if self.is_as_field_included {
            self.is_as_field_included = false;
            self.numeric_field -= 2048;
        }

        self
    }

    /// Exclude [`asname`](struct.IpData.html#structfield.asname) from request
    pub fn exclude_asname(mut self) -> Self {
        if self.is_asname_included {
            self.is_asname_included = false;
            self.numeric_field -= 4194304;
        }

        self
    }

    /// Exclude [`reverse`](struct.IpData.html#structfield.reverse) from request
    pub fn exclude_reverse(mut self) -> Self {
        if self.is_reverse_included {
            self.is_reverse_included = false;
            self.numeric_field -= 4096;
        }

        self
    }

    /// Exclude [`mobile`](struct.IpData.html#structfield.mobile) from request
    pub fn exclude_mobile(mut self) -> Self {
        if self.is_mobile_included {
            self.is_mobile_included = false;
            self.numeric_field -= 65536;
        }

        self
    }

    /// Exclude [`proxy`](struct.IpData.html#structfield.proxy) from request
    pub fn exclude_proxy(mut self) -> Self {
        if self.is_proxy_included {
            self.is_proxy_included = false;
            self.numeric_field -= 131072;
        }

        self
    }

    /// Exclude [`hosting`](struct.IpData.html#structfield.hosting) from request
    pub fn exclude_hosting(mut self) -> Self {
        if self.is_hosting_included {
            self.is_hosting_included = false;
            self.numeric_field -= 16777216;
        }

        self
    }

    /// Exclude [`query`](struct.IpData.html#structfield.query) from request
    pub fn exclude_query(mut self) -> Self {
        if self.is_query_included {
            self.is_query_included = false;
            self.numeric_field -= 8192;
        }

        self
    }

    /// Set custom language for [`IpData`]
    pub fn set_language(mut self, language: IpApiLanguage) -> Self {
        self.language = language;

        self
    }
}

/// Create an empty config to create your own from scratch
pub fn generate_empty_config() -> IpApiConfig {
    IpApiConfig {
        numeric_field: 32768,
        is_continent_included: false,
        is_continent_code_included: false,
        is_country_included: false,
        is_country_code_included: false,
        is_region_included: false,
        is_region_name_included: false,
        is_city_included: false,
        is_district_included: false,
        is_zip_included: false,
        is_lat_included: false,
        is_lon_included: false,
        is_timezone_included: false,
        is_offset_included: false,
        is_currency_included: false,
        is_isp_included: false,
        is_org_included: false,
        is_as_field_included: false,
        is_asname_included: false,
        is_reverse_included: false,
        is_mobile_included: false,
        is_proxy_included: false,
        is_hosting_included: false,
        is_query_included: false,
        language: IpApiLanguage::En,
    }
}

/// Generate minimum config that includes only important fields
pub fn generate_minimum_config() -> IpApiConfig {
    IpApiConfig {
        numeric_field: 41976594,
        is_continent_included: false,
        is_continent_code_included: false,
        is_country_included: false,
        is_country_code_included: true,
        is_region_included: false,
        is_region_name_included: false,
        is_city_included: true,
        is_district_included: false,
        is_zip_included: false,
        is_lat_included: false,
        is_lon_included: false,
        is_timezone_included: true,
        is_offset_included: true,
        is_currency_included: true,
        is_isp_included: true,
        is_org_included: false,
        is_as_field_included: false,
        is_asname_included: false,
        is_reverse_included: false,
        is_mobile_included: false,
        is_proxy_included: false,
        is_hosting_included: false,
        is_query_included: false,
        language: IpApiLanguage::En,
    }
}

/// Generate maximum config that includes all fields
pub fn generate_maximum_config() -> IpApiConfig {
    IpApiConfig {
        numeric_field: 66830335,
        is_continent_included: true,
        is_continent_code_included: true,
        is_country_included: true,
        is_country_code_included: true,
        is_region_included: true,
        is_region_name_included: true,
        is_city_included: true,
        is_district_included: true,
        is_zip_included: true,
        is_lat_included: true,
        is_lon_included: true,
        is_timezone_included: true,
        is_offset_included: true,
        is_currency_included: true,
        is_isp_included: true,
        is_org_included: true,
        is_as_field_included: true,
        is_asname_included: true,
        is_reverse_included: true,
        is_mobile_included: true,
        is_proxy_included: true,
        is_hosting_included: true,
        is_query_included: true,
        language: IpApiLanguage::En,
    }
}
