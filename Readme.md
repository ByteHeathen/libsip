# libsip

libsip is a library implementation of the sip protocol as described in
[rfc3261](https://tools.ietf.org/html/rfc3261). libsip intends to implement
parsing the entire SIP Protocol, but will only provide helpers for certain
common use cases.

**WIP** This library is still very much under construction.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://travis-ci.org/bytebuddha/libsip.svg?branch=master)](https://travis-ci.org/bytebuddha/libsip)
[![Crates.io](https://img.shields.io/crates/v/libsip.svg)](https://crates.io/crates/libsip)
[![Docs.rs](https://docs.rs/libsip/badge.svg)](https://docs.rs/libsip)

#### Running the examples
```bash
git clone https://github.com/Bytebuddha/libsip
cd /libsip
# This example expects a server with the credentials in examples/udp_register.rs
# to be running without it will fail.
cargo run --example udp_register
# This example expects a server with the credentials in examples/console.rs
# to be running without it will fail. It prints all requests received to the terminal.
cargo run --example console
```

#### dependencies
  - **[nom](https://crates.io/crates/nom) 5.0.1**
  - **[rand](https://crates.io/crates/rand) 0.7.2**
  - **[bytes](https://crates.io/crates/bytes) 0.4.12**
  - **[md5](https://crates.io/crates/md5) 0.7.0**
  - **[serde](https://crates.io/crates/serde) 1.0.102**

#### Alternatives
  - **[parsip](https://crates/crates/parsip)**
  Is only for parsing SIP messages. I wanted libsip to be able to handle some user case's
  specifically Registration.
  - **[sip-codec](https://crates/crates/sip-codec)**
  I attempted to use this library first, lots of features are not implemented like writing
  sip requests and a few other fairly common things. I also wanted SIP Headers to be in the form of an enum witch would have required
  basically rewriting the whole crate.
  - **[sip](https://crates/crates/sip)**
  This crate appears to be empty
  - **[tokio-sip](https://crates.io/crates/tokio-sip)**
  This crate also appears to be empty
