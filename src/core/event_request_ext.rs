use crate::{header, Header, SipMessage, SipMessageError};

/// The extension for requests related to events, e.g., SUBSCRIBE, NOTIFY. [RFC6665](https://tools.ietf.org/html/rfc6665)
pub trait EventRequestExt {
    /// Returns the value of Event header if present
    fn event(&self) -> Result<&String, SipMessageError>;
}

impl EventRequestExt for SipMessage {
    fn event(&self) -> Result<&String, SipMessageError> {
        header!(
            self.headers().0.iter(),
            Header::Event,
            SipMessageError::MissingEventHeader
        )
    }
}
