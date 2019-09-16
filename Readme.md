# libsip

libsip is a library implementation of the sip protocol as described in
[rfc3261](https://tools.ietf.org/html/rfc3261). libsip intends to implement
parsing the entire SIP Protocol, but will only provide helpers for certain
common use cases.

#### Parsing

```rust
extern crate libsip;

use libsip::parse_message;

const SIP_MESSAGE: &'static str "SIP/2.0 200 OK\r\n\r\n";

fn main() {
  let res = parse_message(SIP_MESSAGE.as_ref());
  println!("{:?}", res);
}
```
