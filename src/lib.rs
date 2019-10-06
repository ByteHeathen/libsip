//! libsip has three basic components a parser and managers.
//!
//! Managers are utility struct's meant to ease one specifc asspect of the sip protocol,
//! the only manager currently implemented is for endpoint registration.
//!
//! ### Parsing
//! libsip exposes many parsing function though only one `parse_message` is needed.
//! ```rust,compile_fail
//!   let packet = "SIP/2.0 200 OK\r\n\r\n";
//!   let output = parse_message(packet)?;
//! ```
//!
//! ### Registration
//! The registration manager is used to generate REGISTER requests. Once
//! that is sent to the server you must wait for the Challange response pass
//! it to the `set_challenge` method of the RegistrationManager.
//! ```rust,compile_fail
//!    let manager = RegistrationManager::new(account_uri, local_uri, Default::default());
//!    let req = manager.get_request()?;
//!    // send request
//!    // get response as res
//!    manager.set_challenge(res)?;
//!    let final_req = manager.get_request()?;
//!    // send request and expect 200 Ok.
//! ```

#[macro_use]
extern crate nom;

#[macro_use]
mod macros;

pub mod core;
pub mod uri;
pub mod headers;
pub mod response;
pub mod registration;
pub(crate) mod parse;

pub use crate::core::Version;
pub use crate::core::Method;
pub use crate::headers::Header;
pub use crate::headers::Headers;

pub use crate::uri::Uri;
pub use crate::uri::Domain;

pub use crate::core::SipMessage;
pub use crate::core::message::parse_message;
pub use crate::registration::RegistrationManager;
pub use crate::response::ResponseGenerator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
