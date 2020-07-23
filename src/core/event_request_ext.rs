use crate::{header, Header, SipMessage};

/// The extension for requests related to events, e.g., SUBSCRIBE, NOTIFY. [RFC6665](https://tools.ietf.org/html/rfc6665)
pub trait EventRequestExt {
    /// Returns the value of Event header if present
    fn event(&self) -> Result<&String, MissingEventHeaderError>;
}

pub struct MissingEventHeaderError;

impl EventRequestExt for SipMessage {
    fn event(&self) -> Result<&String, MissingEventHeaderError> {
        header!(
            self.headers().0.iter(),
            Header::Event,
            MissingEventHeaderError
        )
    }
}
