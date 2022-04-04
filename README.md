# IP API Client

The client (based on [ip-api.com](https://ip-api.com/) api) allows you to get information about the IP address

# Usage

Add to project

```toml
[dependencies]
ip-api-client = "0.2.0"
```

Write some Rust

```rust
use ip_api_client as Client;
use ip_api_client::IpData;

fn main() {
    // You can
    // `generate_empty_config` (to create your own config from scratch)
    // `generate_minimum_config` (that includes only important fields)
    // `generate_maximum_config` (that includes all fields)
    let ip_data: IpData = Client::generate_empty_config()
        // or `exclude_country` if this field is already included in the generated config
        .include_country()
        // or `exclude_currency` if this field is already included in the generated config
        .include_currency()
        // `make_request` takes "ip"/"domain"/"empty string (if you want to request your ip)"
        .make_request("1.1.1.1").unwrap();

    println!("{}'s national currency is {}", ip_data.country.unwrap(), ip_data.currency.unwrap());
}
```

# Peculiarities

- We use `as_field` instead of `as`
  (As stated in the [ip-api.com API documentation](https://ip-api.com/docs/api:json#as))
  since it is a [strict keyword](https://doc.rust-lang.org/reference/keywords.html#strict-keywords) in rust,
  such as `pub`, `impl` or `struct`.

# Development Progress

- [x] Request IP address information with a configuration structure that allows you to customize the requested fields in the request to save traffic.
- [ ] Block all requests until the end of the limit if the last request was rate-limited.
- [ ] Ability to cache all responses with automatic removal of old ip-data when the maximum cache size is reached.


# License

This library (ip-api-client) is available under the MIT license. See the LICENSE file for more info.
