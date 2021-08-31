# BitGo API

 Rustlang implementation of the BitGo API.

 BitGo provides a simple and robust RESTful API and client SDK to integrate digital currency wallets with your application. In Platform V2, we have extended our API and SDK to allow the management of multiple digital currencies and wallets through a single, unified interface.

The BitGo SDK enables the following:

* Creation of multi-signature wallets
* Wallet balance and transaction listing
* Transaction creation and signing
* Transaction monitoring and notifications
* Secure user authentication
* Multi-user workflows for use in enterprise environments
* Policies and spending limits


## Example

Make sure you have the below line in your Cargo.toml:

```toml
[dependencies]
bitgo_api = {version="0.1.5"}
```

## How to mock

You can mock the APIs for testing purpose like this:

```
let mut mock = MockBitGo::new();
mock.expect_create_address().return_const(Ok(
    json!({ "address": "2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS" }),
));

let v = mock.create_address("any", " any").await.unwrap();
assert_eq!(
    value_or_error(v, "address").unwrap().to_owned(),
    "2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS"
);
```


## License

This project is licensed under the [MIT license].

[Apache license]: https://github.com/nagarajmanjunath/bitgo_api/blob/main/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in BitGo by you, shall be licensed as Apache, without any additional
terms or conditions.