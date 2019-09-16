#[macro_use]
extern crate nom;
extern crate failure;

#[macro_use]
mod macros;

pub mod core;
pub mod uri;
pub mod headers;
pub mod registration;
pub(crate) mod parse;

pub use crate::core::Version;
pub use crate::core::Method;
pub use crate::headers::Header;

pub use crate::uri::Uri;
pub use crate::uri::Domain;

pub use crate::core::SipMessage;
pub use crate::core::message::parse_message;
pub use crate::registration::RegistrationManager;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
