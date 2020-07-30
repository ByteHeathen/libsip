use crate::{SipMessage, SipMessageError};

pub trait RequestExt {
    fn uri_username(&self) -> Result<&String, SipMessageError>;
}

impl RequestExt for SipMessage {
    fn uri_username(&self) -> Result<&String, SipMessageError> {
        if let SipMessage::Request { uri, .. } = self {
            if let Some(auth) = &uri.auth {
                Ok(&auth.username)
            } else {
                Err(SipMessageError::MissingUriUsername)
            }
        } else {
            Err(SipMessageError::MissingUriUsername)
        }
    }
}
