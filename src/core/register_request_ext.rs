use crate::{SipMessage, SipMessageExt};

pub trait RegisterRequestExt {
    /// Returns one of:
    /// * "expires" of "Contact"
    /// * "Expires"
    ///
    /// [RFC3261, Page 65](https://tools.ietf.org/html/rfc3261#page-65) defines this behavior
    fn expires(&self) -> Option<u32>;
}

impl RegisterRequestExt for SipMessage {
    fn expires(&self) -> Option<u32> {
        self.contact_header_expires().or(self.expires_header())
    }
}
