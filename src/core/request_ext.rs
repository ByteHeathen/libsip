use crate::{MissingUsernameError, SipMessage};

pub trait RequestExt {
    fn uri_username(&self) -> Result<&String, MissingUsernameError>;
}

impl RequestExt for SipMessage {
    fn uri_username(&self) -> Result<&String, MissingUsernameError> {
        if let SipMessage::Request { uri, .. } = self {
            if let Some(auth) = &uri.auth {
                Ok(&auth.username)
            } else {
                Err(MissingUsernameError::Uri)
            }
        } else {
            Err(MissingUsernameError::Uri)
        }
    }
}
