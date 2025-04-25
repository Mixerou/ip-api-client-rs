//! The client (based on [ip-api.com API](https://ip-api.com/docs/api:json))
//! allows you to get information about the IP address
//!
//! # Example
//!
//! ```rust
//! # #[tokio::main]
//! # async fn main() {
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
//!     .make_request("1.1.1.1")
//!     .await
//!     .unwrap();
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
//!     // `make_batch_request` takes "IPv4"/"IPv6"
//!     .make_batch_request(vec!["1.1.1.1", "8.8.8.8"])
//!     .await
//!     .unwrap();
//!
//! println!(
//!     "1.1.1.1 belongs to `{}` and 8.8.8.8 belongs to `{}`",
//!     ip_batch_data.first().unwrap().isp.as_ref().unwrap(),
//!     ip_batch_data.last().unwrap().isp.as_ref().unwrap(),
//! );
//! # }
//! ```

#![deny(missing_docs)]

use hyper::body::HttpBody;
use hyper::{Body, Client, Method, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[cfg(test)]
mod tests {
    use crate::{generate_empty_config, IpData};

    #[tokio::test]
    async fn make_request() {
        assert_eq!(
            generate_empty_config()
                .include_query()
                .make_request("1.1.1.1")
                .await
                .unwrap()
                .query
                .unwrap(),
            String::from("1.1.1.1")
        );
    }

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
}

/// Represents all the ways that a request can fail
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[repr(u32)]
enum IpDataField {
    // Status = 1 << 14, // Never used
    Message = 1 << 15,

    Continent = 1 << 20,
    ContinentCode = 1 << 21,
    Country = 1 << 0,
    CountryCode = 1 << 1,
    Region = 1 << 2,
    RegionName = 1 << 3,
    City = 1 << 4,
    District = 1 << 19,
    Zip = 1 << 5,
    Lat = 1 << 6,
    Lon = 1 << 7,
    Timezone = 1 << 8,
    Offset = 1 << 25,
    Currency = 1 << 23,
    Isp = 1 << 9,
    Org = 1 << 10,
    AsField = 1 << 11,
    Asname = 1 << 22,
    Reverse = 1 << 12,
    Mobile = 1 << 16,
    Proxy = 1 << 17,
    Hosting = 1 << 24,
    Query = 1 << 13,
}

/// Configuration structure allows you to customize the requested fields in the request
/// to save traffic
#[derive(Clone, Debug)]
pub struct IpApiConfig {
    numeric_field: u32,
    language: IpApiLanguage,
}

impl IpApiConfig {
    fn build_uri(
        resource: &str,
        target: Option<&str>,
        fields: u32,
        language: IpApiLanguage,
    ) -> String {
        format!(
            "http://ip-api.com/{}/{}?fields={}{}",
            resource,
            target.unwrap_or(""),
            fields,
            match language {
                IpApiLanguage::De => "&lang=de",
                IpApiLanguage::En => "",
                IpApiLanguage::Es => "&lang=es",
                IpApiLanguage::Fr => "&lang=fr",
                IpApiLanguage::Ja => "&lang=ja",
                IpApiLanguage::PtBr => "&lang=pt-BR",
                IpApiLanguage::Ru => "&lang=ru",
                IpApiLanguage::ZhCn => "&lang=zh-CN",
            }
        )
    }

    fn check_response(response: &Response<Body>) -> Result<(), IpApiError> {
        if response.status() == 429 {
            let Some(header) = response.headers().get("X-Ttl") else {
                return Err(IpApiError::UnexpectedError(Some(
                    "Failed to get `X-Ttl` header from the response".into(),
                )));
            };
            let Ok(header) = header.to_str() else {
                return Err(IpApiError::UnexpectedError(Some(
                    "Failed to convert `X-Ttl` header from the response to &str".into(),
                )));
            };
            let Ok(header) = header.parse() else {
                return Err(IpApiError::UnexpectedError(Some(
                    "Failed to parse `X-Ttl` header from the response".into(),
                )));
            };

            return Err(IpApiError::RateLimit(header));
        }

        Ok(())
    }

    fn check_error_message(message: Option<String>) -> Result<(), IpApiError> {
        if let Some(message) = message {
            return match message.as_str() {
                "invalid query" => Err(IpApiError::InvalidQuery),
                "private range" => Err(IpApiError::PrivateRange),
                "reserved range" => Err(IpApiError::ReservedRange),
                message => Err(IpApiError::UnexpectedError(Some(message.into()))),
            };
        }

        Ok(())
    }

    async fn parse_response_body(response: &mut Response<Body>) -> Result<String, IpApiError> {
        let Some(body) = response.body_mut().data().await else {
            return Err(IpApiError::UnexpectedError(Some(
                "Response is empty".into(),
            )));
        };
        let Ok(body) = body else {
            return Err(IpApiError::UnexpectedError(Some(
                "Failed to retrieve body from the response".into(),
            )));
        };
        let Ok(body) = String::from_utf8(body.to_vec()) else {
            return Err(IpApiError::UnexpectedError(Some(
                "Failed to convert body from the response to String".into(),
            )));
        };

        Ok(body)
    }

    /// Making a request to [ip-api.com API](https://ip-api.com/docs/api:json)
    ///
    /// `target` can be "ip"/"domain"/"empty string (if you want to request your ip)"
    pub async fn make_request(self, target: &str) -> Result<IpData, IpApiError> {
        let uri = Self::build_uri("json", Some(target), self.numeric_field, self.language);

        let client = Client::new();
        let Ok(uri) = uri.parse() else {
            return Err(IpApiError::UnexpectedError(Some(
                "Failed to parse request URI".into(),
            )));
        };
        let Ok(response) = &mut client.get(uri).await else {
            return Err(IpApiError::UnexpectedError(Some(
                "Failed to make a request".into(),
            )));
        };

        Self::check_response(response)?;

        let body = Self::parse_response_body(response).await?;
        let Ok(ip_data): Result<IpApiMessage, _> = serde_json::from_str(body.as_str()) else {
            return Err(IpApiError::UnexpectedError(Some(
                "Failed to parse body from the response".into(),
            )));
        };

        Self::check_error_message(ip_data.message)?;

        let Ok(ip_data): Result<IpData, _> = serde_json::from_str(body.as_str()) else {
            return Err(IpApiError::UnexpectedError(Some(
                "Failed to parse body from the response".into(),
            )));
        };

        Ok(ip_data)
    }

    /// Making a batch request to [ip-api.com API](https://ip-api.com/docs/api:batch)
    ///
    /// `target` can be "IPv4"/"IPv6"
    pub async fn make_batch_request(self, targets: Vec<&str>) -> Result<Vec<IpData>, IpApiError> {
        let uri = Self::build_uri("batch", None, self.numeric_field, self.language);

        let Ok(request) = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(json!(targets).to_string()))
        else {
            return Err(IpApiError::UnexpectedError(Some(
                "Failed to build a request".into(),
            )));
        };

        let client = Client::new();
        let Ok(response) = &mut client.request(request).await else {
            return Err(IpApiError::UnexpectedError(Some(
                "Failed to make a request".into(),
            )));
        };

        Self::check_response(response)?;

        let body = Self::parse_response_body(response).await?;
        let Ok(ip_batch_data): Result<Vec<IpApiMessage>, _> = serde_json::from_str(body.as_str())
        else {
            return Err(IpApiError::UnexpectedError(Some(
                "Failed to parse body from the response".into(),
            )));
        };

        for ip_data in ip_batch_data {
            Self::check_error_message(ip_data.message)?;
        }

        let Ok(ip_batch_data): Result<Vec<IpData>, _> = serde_json::from_str(body.as_str()) else {
            return Err(IpApiError::UnexpectedError(Some(
                "Failed to parse body from the response".into(),
            )));
        };

        Ok(ip_batch_data)
    }

    /// Include [`continent`](struct.IpData.html#structfield.continent) in request
    pub fn include_continent(mut self) -> Self {
        self.numeric_field |= IpDataField::Continent as u32;
        self
    }

    /// Include [`continent_code`](struct.IpData.html#structfield.continent_code) in request
    pub fn include_continent_code(mut self) -> Self {
        self.numeric_field |= IpDataField::ContinentCode as u32;
        self
    }

    /// Include [`country`](struct.IpData.html#structfield.country) in request
    pub fn include_country(mut self) -> Self {
        self.numeric_field |= IpDataField::Country as u32;
        self
    }

    /// Include [`country_code`](struct.IpData.html#structfield.country_code) in request
    pub fn include_country_code(mut self) -> Self {
        self.numeric_field |= IpDataField::CountryCode as u32;
        self
    }

    /// Include [`region`](struct.IpData.html#structfield.region) in request
    pub fn include_region(mut self) -> Self {
        self.numeric_field |= IpDataField::Region as u32;
        self
    }

    /// Include [`region_name`](struct.IpData.html#structfield.region_name) in request
    pub fn include_region_name(mut self) -> Self {
        self.numeric_field |= IpDataField::RegionName as u32;
        self
    }

    /// Include [`city`](struct.IpData.html#structfield.city) in request
    pub fn include_city(mut self) -> Self {
        self.numeric_field |= IpDataField::City as u32;
        self
    }

    /// Include [`district`](struct.IpData.html#structfield.district) in request
    pub fn include_district(mut self) -> Self {
        self.numeric_field |= IpDataField::District as u32;
        self
    }

    /// Include [`zip`](struct.IpData.html#structfield.zip) in request
    pub fn include_zip(mut self) -> Self {
        self.numeric_field |= IpDataField::Zip as u32;
        self
    }

    /// Include [`lat`](struct.IpData.html#structfield.lat) in request
    pub fn include_lat(mut self) -> Self {
        self.numeric_field |= IpDataField::Lat as u32;
        self
    }

    /// Include [`lon`](struct.IpData.html#structfield.lon) in request
    pub fn include_lon(mut self) -> Self {
        self.numeric_field |= IpDataField::Lon as u32;
        self
    }

    /// Include [`timezone`](struct.IpData.html#structfield.timezone) in request
    pub fn include_timezone(mut self) -> Self {
        self.numeric_field |= IpDataField::Timezone as u32;
        self
    }

    /// Include [`offset`](struct.IpData.html#structfield.offset) in request
    pub fn include_offset(mut self) -> Self {
        self.numeric_field |= IpDataField::Offset as u32;
        self
    }

    /// Include [`currency`](struct.IpData.html#structfield.currency) in request
    pub fn include_currency(mut self) -> Self {
        self.numeric_field |= IpDataField::Currency as u32;
        self
    }

    /// Include [`isp`](struct.IpData.html#structfield.isp) in request
    pub fn include_isp(mut self) -> Self {
        self.numeric_field |= IpDataField::Isp as u32;
        self
    }

    /// Include [`org`](struct.IpData.html#structfield.org) in request
    pub fn include_org(mut self) -> Self {
        self.numeric_field |= IpDataField::Org as u32;
        self
    }

    /// Include [`as_field`](struct.IpData.html#structfield.as_field) in request
    pub fn include_as_field(mut self) -> Self {
        self.numeric_field |= IpDataField::AsField as u32;
        self
    }

    /// Include [`asname`](struct.IpData.html#structfield.asname) in request
    pub fn include_asname(mut self) -> Self {
        self.numeric_field |= IpDataField::Asname as u32;
        self
    }

    /// Include [`reverse`](struct.IpData.html#structfield.reverse) in request
    pub fn include_reverse(mut self) -> Self {
        self.numeric_field |= IpDataField::Reverse as u32;
        self
    }

    /// Include [`mobile`](struct.IpData.html#structfield.mobile) in request
    pub fn include_mobile(mut self) -> Self {
        self.numeric_field |= IpDataField::Mobile as u32;
        self
    }

    /// Include [`proxy`](struct.IpData.html#structfield.proxy) in request
    pub fn include_proxy(mut self) -> Self {
        self.numeric_field |= IpDataField::Proxy as u32;
        self
    }

    /// Include [`hosting`](struct.IpData.html#structfield.hosting) in request
    pub fn include_hosting(mut self) -> Self {
        self.numeric_field |= IpDataField::Hosting as u32;
        self
    }

    /// Include [`query`](struct.IpData.html#structfield.query) in request
    pub fn include_query(mut self) -> Self {
        self.numeric_field |= IpDataField::Query as u32;
        self
    }

    /// Exclude [`continent`](struct.IpData.html#structfield.continent) from request
    pub fn exclude_continent(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Continent as u32);
        self
    }

    /// Exclude [`continent_code`](struct.IpData.html#structfield.continent_code) from request
    pub fn exclude_continent_code(mut self) -> Self {
        self.numeric_field &= !(IpDataField::ContinentCode as u32);
        self
    }

    /// Exclude [`country`](struct.IpData.html#structfield.country) from request
    pub fn exclude_country(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Country as u32);
        self
    }

    /// Exclude [`country_code`](struct.IpData.html#structfield.country_code) from request
    pub fn exclude_country_code(mut self) -> Self {
        self.numeric_field &= !(IpDataField::CountryCode as u32);
        self
    }

    /// Exclude [`region`](struct.IpData.html#structfield.region) from request
    pub fn exclude_region(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Region as u32);
        self
    }

    /// Exclude [`region_name`](struct.IpData.html#structfield.region_name) from request
    pub fn exclude_region_name(mut self) -> Self {
        self.numeric_field &= !(IpDataField::RegionName as u32);
        self
    }

    /// Exclude [`city`](struct.IpData.html#structfield.city) from request
    pub fn exclude_city(mut self) -> Self {
        self.numeric_field &= !(IpDataField::City as u32);
        self
    }

    /// Exclude [`district`](struct.IpData.html#structfield.district) from request
    pub fn exclude_district(mut self) -> Self {
        self.numeric_field &= !(IpDataField::District as u32);
        self
    }

    /// Exclude [`zip`](struct.IpData.html#structfield.zip) from request
    pub fn exclude_zip(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Zip as u32);
        self
    }

    /// Exclude [`lat`](struct.IpData.html#structfield.lat) from request
    pub fn exclude_lat(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Lat as u32);
        self
    }

    /// Exclude [`lon`](struct.IpData.html#structfield.lon) from request
    pub fn exclude_lon(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Lon as u32);
        self
    }

    /// Exclude [`timezone`](struct.IpData.html#structfield.timezone) from request
    pub fn exclude_timezone(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Timezone as u32);
        self
    }

    /// Exclude [`offset`](struct.IpData.html#structfield.offset) from request
    pub fn exclude_offset(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Offset as u32);
        self
    }

    /// Exclude [`currency`](struct.IpData.html#structfield.currency) from request
    pub fn exclude_currency(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Currency as u32);
        self
    }

    /// Exclude [`isp`](struct.IpData.html#structfield.isp) from request
    pub fn exclude_isp(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Isp as u32);
        self
    }

    /// Exclude [`org`](struct.IpData.html#structfield.org) from request
    pub fn exclude_org(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Org as u32);
        self
    }

    /// Exclude [`as_field`](struct.IpData.html#structfield.as_field) from request
    pub fn exclude_as_field(mut self) -> Self {
        self.numeric_field &= !(IpDataField::AsField as u32);
        self
    }

    /// Exclude [`asname`](struct.IpData.html#structfield.asname) from request
    pub fn exclude_asname(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Asname as u32);
        self
    }

    /// Exclude [`reverse`](struct.IpData.html#structfield.reverse) from request
    pub fn exclude_reverse(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Reverse as u32);
        self
    }

    /// Exclude [`mobile`](struct.IpData.html#structfield.mobile) from request
    pub fn exclude_mobile(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Mobile as u32);
        self
    }

    /// Exclude [`proxy`](struct.IpData.html#structfield.proxy) from request
    pub fn exclude_proxy(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Proxy as u32);
        self
    }

    /// Exclude [`hosting`](struct.IpData.html#structfield.hosting) from request
    pub fn exclude_hosting(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Hosting as u32);
        self
    }

    /// Exclude [`query`](struct.IpData.html#structfield.query) from request
    pub fn exclude_query(mut self) -> Self {
        self.numeric_field &= !(IpDataField::Query as u32);
        self
    }

    /// Set custom language for [`IpData`]
    pub fn set_language(mut self, language: IpApiLanguage) -> Self {
        self.language = language;
        self
    }
}

/// Create an empty config to create your own from scratch
pub const fn generate_empty_config() -> IpApiConfig {
    IpApiConfig {
        numeric_field: IpDataField::Message as u32,
        language: IpApiLanguage::En,
    }
}

/// Generate minimum config that includes only important fields
pub const fn generate_minimum_config() -> IpApiConfig {
    IpApiConfig {
        numeric_field: IpDataField::Message as u32
            | IpDataField::CountryCode as u32
            | IpDataField::City as u32
            | IpDataField::Timezone as u32
            | IpDataField::Offset as u32
            | IpDataField::Currency as u32
            | IpDataField::Isp as u32,
        language: IpApiLanguage::En,
    }
}

/// Generate maximum config that includes all fields
pub const fn generate_maximum_config() -> IpApiConfig {
    IpApiConfig {
        numeric_field: IpDataField::Message as u32
            | IpDataField::Continent as u32
            | IpDataField::ContinentCode as u32
            | IpDataField::Country as u32
            | IpDataField::CountryCode as u32
            | IpDataField::Region as u32
            | IpDataField::RegionName as u32
            | IpDataField::City as u32
            | IpDataField::District as u32
            | IpDataField::Zip as u32
            | IpDataField::Lat as u32
            | IpDataField::Lon as u32
            | IpDataField::Timezone as u32
            | IpDataField::Offset as u32
            | IpDataField::Currency as u32
            | IpDataField::Isp as u32
            | IpDataField::Org as u32
            | IpDataField::AsField as u32
            | IpDataField::Asname as u32
            | IpDataField::Reverse as u32
            | IpDataField::Mobile as u32
            | IpDataField::Proxy as u32
            | IpDataField::Hosting as u32
            | IpDataField::Query as u32,
        language: IpApiLanguage::En,
    }
}
