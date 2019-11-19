#![recursion_limit = "256"]
//! libsip has three basic components a parser and managers.
//!
//! Managers are utility struct's meant to ease one specifc asspect of the sip protocol,
//! the only manager currently implemented is for endpoint registration.
//!
//! ### Parsing
//! libsip exposes many parsing function though only one `parse_message` is needed.
//! ```rust
//! extern crate libsip;
//!
//! use libsip::parse_message;
//!
//! let packet = "SIP/2.0 200 OK\r\n\r\n";
//! let output = libsip::parse_message(packet.as_ref()).unwrap();
//! ```
//!
//! ### Creating Messages
//! This crate provides 2 abstraction's to aid in building sip messages.
//! The `ResponseGenerator` is used to create sip response's and the
//! `RequestGenerator` is used to generate sip requests.
//!  ```rust
//!     extern crate libsip;
//!
//!     use libsip::ResponseGenerator;
//!     use libsip::RequestGenerator;
//!     use libsip::Method;
//!     use libsip::uri::parse_uri;
//!
//!     let _res = ResponseGenerator::new()
//!                         .code(200)
//!                         .build()
//!                         .unwrap();
//!
//!     let uri = parse_uri("sip:1@0.0.0.0:5060;transport=UDP".as_ref()).unwrap().1;
//!     let _req = RequestGenerator::new()
//!                         .method(Method::Invite)
//!                         .uri(uri)
//!                         .build()
//!                         .unwrap();
//!  ```
//!
//! ### Registration
//! The registration manager is used to generate REGISTER requests. Once
//! that is sent to the server you must wait for the Challange response pass
//! it to the `set_challenge` method of the RegistrationManager.
//! reqpeatedly calling the `get_request` method will cause the c_nonce
//! counter to be incremented and a new hash computed.

#[macro_use]
extern crate nom;
extern crate serde;

#[macro_use]
mod macros;

pub mod client;
pub mod core;
pub mod headers;
pub(crate) mod parse;
pub mod request;
pub mod response;
pub mod uri;

pub use crate::{
    core::{Method, Version},
    headers::{Header, Headers},
};

pub use crate::uri::{Domain, Uri};

pub use crate::{
    core::{message::parse_message, SipMessage},
    request::RequestGenerator,
    response::ResponseGenerator,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
