use crate::{
    ContactHeader, Header, Method, MissingContactExpiresError, MissingHeaderError, MissingTagError,
    MissingUsernameError, MissingViaBranchError, NamedHeader, SipMessage, ViaHeader,
};

pub trait SipMessageExt {
    fn from_header(&self) -> Result<&NamedHeader, MissingHeaderError>;

    fn from_header_mut(&mut self) -> Result<&mut NamedHeader, MissingHeaderError>;

    fn from_header_tag(&self) -> Result<&String, MissingTagError>;

    fn set_from_header_tag(&mut self, tag: String);

    fn from_header_username(&self) -> Result<&String, MissingUsernameError>;

    fn to_header(&self) -> Result<&NamedHeader, MissingHeaderError>;

    fn to_header_mut(&mut self) -> Result<&mut NamedHeader, MissingHeaderError>;

    fn to_header_tag(&self) -> Result<&String, MissingTagError>;

    fn set_to_header_tag(&mut self, tag: String);

    fn to_header_username(&self) -> Result<&String, MissingUsernameError>;

    fn via_header(&self) -> Result<&ViaHeader, MissingHeaderError>;

    fn via_header_mut(&mut self) -> Result<&mut ViaHeader, MissingHeaderError>;

    fn via_header_branch(&self) -> Result<&String, MissingViaBranchError>;

    fn call_id(&self) -> Result<&String, MissingHeaderError>;

    fn call_id_mut(&mut self) -> Result<&mut String, MissingHeaderError>;

    fn cseq(&self) -> Result<(u32, Method), MissingHeaderError>;

    fn cseq_mut(&mut self) -> Result<(&mut u32, &mut Method), MissingHeaderError>;

    fn contact_header(&self) -> Result<&ContactHeader, MissingHeaderError>;

    fn contact_header_mut(&mut self) -> Result<&mut ContactHeader, MissingHeaderError>;

    fn contact_header_username(&self) -> Result<&String, MissingUsernameError>;

    /// Returns number of seconds if it's specified in the Contact header
    fn contact_header_expires(&self) -> Result<u32, MissingContactExpiresError>;

    fn expires_header(&self) -> Result<u32, MissingHeaderError>;

    fn expires_header_mut(&mut self) -> Result<&mut u32, MissingHeaderError>;
}

#[macro_export]
macro_rules! header {
    ($iter:expr, $header:path, $error:expr) => {
        $iter
            .find_map(|header| {
                if let $header(header) = header {
                    Some(header)
                } else {
                    None
                }
            })
            .ok_or($error)
    };
}

macro_rules! named_header_param {
    ($header:expr, $param:expr, $error:expr) => {
        if let Ok(header) = $header {
            if let Some(Some(param)) = header.parameters.get($param) {
                Ok(param)
            } else {
                Err($error)
            }
        } else {
            Err($error)
        }
    };
}

macro_rules! named_header_username {
    ($header:expr, $error:expr) => {
        if let Ok(header) = $header {
            if let Some(auth) = &header.uri.auth {
                Ok(&auth.username)
            } else {
                Err($error)
            }
        } else {
            Err($error)
        }
    };
}

impl SipMessageExt for SipMessage {
    fn from_header(&self) -> Result<&NamedHeader, MissingHeaderError> {
        header!(
            self.headers().0.iter(),
            Header::From,
            MissingHeaderError::From
        )
    }

    fn from_header_mut(&mut self) -> Result<&mut NamedHeader, MissingHeaderError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::From,
            MissingHeaderError::From
        )
    }

    fn from_header_tag(&self) -> Result<&String, MissingTagError> {
        named_header_param!(self.from_header(), "tag", MissingTagError::From)
    }

    fn set_from_header_tag(&mut self, tag: String) {
        if let Ok(header) = self.from_header_mut() {
            header.set_param("tag", Some(tag));
        }
    }

    fn from_header_username(&self) -> Result<&String, MissingUsernameError> {
        named_header_username!(self.from_header(), MissingUsernameError::From)
    }

    fn to_header(&self) -> Result<&NamedHeader, MissingHeaderError> {
        header!(self.headers().0.iter(), Header::To, MissingHeaderError::To)
    }

    fn to_header_mut(&mut self) -> Result<&mut NamedHeader, MissingHeaderError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::To,
            MissingHeaderError::To
        )
    }

    fn to_header_tag(&self) -> Result<&String, MissingTagError> {
        named_header_param!(self.to_header(), "tag", MissingTagError::To)
    }

    fn set_to_header_tag(&mut self, tag: String) {
        if let Ok(header) = self.to_header_mut() {
            header.set_param("tag", Some(tag));
        }
    }

    fn to_header_username(&self) -> Result<&String, MissingUsernameError> {
        named_header_username!(self.to_header(), MissingUsernameError::To)
    }

    fn via_header(&self) -> Result<&ViaHeader, MissingHeaderError> {
        header!(
            self.headers().0.iter(),
            Header::Via,
            MissingHeaderError::Via
        )
    }

    fn via_header_mut(&mut self) -> Result<&mut ViaHeader, MissingHeaderError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::Via,
            MissingHeaderError::Via
        )
    }

    fn via_header_branch(&self) -> Result<&String, MissingViaBranchError> {
        if let Ok(header) = self.via_header() {
            header.branch().ok_or(MissingViaBranchError)
        } else {
            Err(MissingViaBranchError)
        }
    }

    fn call_id(&self) -> Result<&String, MissingHeaderError> {
        header!(
            self.headers().0.iter(),
            Header::CallId,
            MissingHeaderError::CallId
        )
    }

    fn call_id_mut(&mut self) -> Result<&mut String, MissingHeaderError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::CallId,
            MissingHeaderError::CallId
        )
    }

    fn cseq(&self) -> Result<(u32, Method), MissingHeaderError> {
        self.headers()
            .0
            .iter()
            .find_map(|header| {
                if let Header::CSeq(cseq, method) = header {
                    Some((*cseq, *method))
                } else {
                    None
                }
            })
            .ok_or(MissingHeaderError::CSeq)
    }

    fn cseq_mut(&mut self) -> Result<(&mut u32, &mut Method), MissingHeaderError> {
        self.headers_mut()
            .0
            .iter_mut()
            .find_map(|header| {
                if let Header::CSeq(cseq, method) = header {
                    Some((cseq, method))
                } else {
                    None
                }
            })
            .ok_or(MissingHeaderError::CSeq)
    }

    fn contact_header(&self) -> Result<&ContactHeader, MissingHeaderError> {
        header!(
            self.headers().0.iter(),
            Header::Contact,
            MissingHeaderError::Contact
        )
    }

    fn contact_header_mut(&mut self) -> Result<&mut ContactHeader, MissingHeaderError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::Contact,
            MissingHeaderError::Contact
        )
    }

    fn contact_header_username(&self) -> Result<&String, MissingUsernameError> {
        if let Ok(header) = self.contact_header() {
            if let Some(auth) = &header.uri.auth {
                Ok(&auth.username)
            } else {
                Err(MissingUsernameError::Contact)
            }
        } else {
            Err(MissingUsernameError::Contact)
        }
    }

    fn contact_header_expires(&self) -> Result<u32, MissingContactExpiresError> {
        // https://tools.ietf.org/html/rfc3261#page-228 "c-p-expires" defines that it must be unsigned number
        named_header_param!(self.contact_header(), "expires", MissingContactExpiresError).and_then(
            |expires| {
                expires
                    .to_string()
                    .parse::<u32>()
                    .map_err(|_| MissingContactExpiresError)
            },
        )
    }

    fn expires_header(&self) -> Result<u32, MissingHeaderError> {
        header!(
            self.headers().0.iter(),
            Header::Expires,
            MissingHeaderError::Expires
        )
        .map(Clone::clone)
    }

    fn expires_header_mut(&mut self) -> Result<&mut u32, MissingHeaderError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::Expires,
            MissingHeaderError::Expires
        )
    }
}
