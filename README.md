# IP API Client

The client (based on [ip-api.com](https://ip-api.com/) api) allows you to get information about the IP address

## Usage

Add to project

```toml
[dependencies]
ip-api-client = "0.4.1"
```

Write some Rust

```rust
use ip_api_client as Client;
use ip_api_client::{IpApiLanguage, IpData};

fn main() {
  // You can
  // `generate_empty_config` (to create your own config from scratch)
  // `generate_minimum_config` (that includes only important fields)
  // `generate_maximum_config` (that includes all fields)
  let ip_data: IpData = Client::generate_empty_config()
          // or `exclude_country` if this field is already included
          // in the generated config
          .include_country()
          // or `exclude_currency` if this field is already included in
          // the generated config
          .include_currency()
          // available languages: de/en (default)/es/fr/ja/pt-Br/ru/zh-CN
          .set_language(IpApiLanguage::De)
          // `make_request` takes
          // "ip"/"domain"/"empty string (if you want to request your ip)"
          .make_request("1.1.1.1").unwrap();

  println!(
    "{}'s national currency is {}",
    ip_data.country.unwrap(),
    ip_data.currency.unwrap(),
  );

  // If you want to request more than one ip, you can use `make_batch_request`
  let ip_batch_data: Vec<IpData> = Client::generate_empty_config()
          .include_isp()
          // `make_batch_request` takes "IPv4"/"IPv6"
          .make_batch_request(vec!["1.1.1.1", "8.8.8.8"]).unwrap();

  println!(
    "1.1.1.1 belongs to `{}` and 8.8.8.8 belongs to `{}`",
    ip_batch_data.get(0).unwrap().isp.as_ref().unwrap(),
    ip_batch_data.get(1).unwrap().isp.as_ref().unwrap(),
  );
}
```

## Peculiarities

- We use `as_field` instead of `as`
  (As stated in the [ip-api.com API documentation](https://ip-api.com/docs/api:json#as))
  since it is a [strict keyword](https://doc.rust-lang.org/reference/keywords.html#strict-keywords) in rust,
  such as `pub`, `impl` or `struct`.

## Development Progress

- [x] Request IP address information with a configuration structure that allows you to customize the requested fields in the request to save traffic.
- [x] Get information about Ip in different languages
- [x] Query multiple IP addresses in one HTTP request.
- [ ] Block all requests until the end of the limit if the last request was rate-limited.
- [ ] Ability to cache all responses with automatic removal of old ip-data when the maximum cache size is reached.

## License

This library (ip-api-client) is available under the MIT license. See the LICENSE file for more info.
