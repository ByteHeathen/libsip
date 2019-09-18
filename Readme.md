# libsip

libsip is a library implementation of the sip protocol as described in
[rfc3261](https://tools.ietf.org/html/rfc3261). libsip intends to implement
parsing the entire SIP Protocol, but will only provide helpers for certain
common use cases.

**WIP** This library is still very much under construction.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

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

#### Running the examples
```bash
git clone https://github.com/Bytebuddha/libsip
cd /libsip
# This example expects a server with the credentials in examples/udp_register.rs
# to be running without it will fail.
cargo run --example udp_register
```

#### dependencies
  - **[nom](https://crates.io/crates/nom) 5.0.1**
  - **[failure](https://crates.io/crates/failure) 0.1.5**
  - **[rand](https://crates.io/crates/rand) 0.7.2**
