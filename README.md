# BitGo API

 Rustlang implementation of the BitGo API.



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