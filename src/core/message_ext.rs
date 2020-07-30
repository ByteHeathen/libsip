use crate::{ContactHeader, Header, Method, NamedHeader, SipMessage, SipMessageError, ViaHeader};

pub trait SipMessageExt {
    fn from_header(&self) -> Result<&NamedHeader, SipMessageError>;

    fn from_header_mut(&mut self) -> Result<&mut NamedHeader, SipMessageError>;

    fn from_header_tag(&self) -> Result<&String, SipMessageError>;

    fn set_from_header_tag(&mut self, tag: String);

    fn from_header_username(&self) -> Result<&String, SipMessageError>;

    fn to_header(&self) -> Result<&NamedHeader, SipMessageError>;

    fn to_header_mut(&mut self) -> Result<&mut NamedHeader, SipMessageError>;

    fn to_header_tag(&self) -> Result<&String, SipMessageError>;

    fn set_to_header_tag(&mut self, tag: String);

    fn to_header_username(&self) -> Result<&String, SipMessageError>;

    fn via_header(&self) -> Result<&ViaHeader, SipMessageError>;

    fn via_header_mut(&mut self) -> Result<&mut ViaHeader, SipMessageError>;

    fn via_header_branch(&self) -> Result<&String, SipMessageError>;

    fn call_id(&self) -> Result<&String, SipMessageError>;

    fn call_id_mut(&mut self) -> Result<&mut String, SipMessageError>;

    fn cseq(&self) -> Result<(u32, Method), SipMessageError>;

    fn cseq_mut(&mut self) -> Result<(&mut u32, &mut Method), SipMessageError>;

    fn contact_header(&self) -> Result<&ContactHeader, SipMessageError>;

    fn contact_header_mut(&mut self) -> Result<&mut ContactHeader, SipMessageError>;

    fn contact_header_username(&self) -> Result<&String, SipMessageError>;

    /// Returns number of seconds if it's specified in the Contact header
    fn contact_header_expires(&self) -> Result<u32, SipMessageError>;

    fn expires_header(&self) -> Result<u32, SipMessageError>;

    fn expires_header_mut(&mut self) -> Result<&mut u32, SipMessageError>;
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
    fn from_header(&self) -> Result<&NamedHeader, SipMessageError> {
        header!(
            self.headers().0.iter(),
            Header::From,
            SipMessageError::MissingFromHeader
        )
    }

    fn from_header_mut(&mut self) -> Result<&mut NamedHeader, SipMessageError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::From,
            SipMessageError::MissingFromHeader
        )
    }

    fn from_header_tag(&self) -> Result<&String, SipMessageError> {
        named_header_param!(self.from_header(), "tag", SipMessageError::MissingFromTag)
    }

    fn set_from_header_tag(&mut self, tag: String) {
        if let Ok(header) = self.from_header_mut() {
            header.set_param("tag", Some(tag));
        }
    }

    fn from_header_username(&self) -> Result<&String, SipMessageError> {
        named_header_username!(self.from_header(), SipMessageError::MissingFromUsername)
    }

    fn to_header(&self) -> Result<&NamedHeader, SipMessageError> {
        header!(
            self.headers().0.iter(),
            Header::To,
            SipMessageError::MissingToHeader
        )
    }

    fn to_header_mut(&mut self) -> Result<&mut NamedHeader, SipMessageError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::To,
            SipMessageError::MissingToHeader
        )
    }

    fn to_header_tag(&self) -> Result<&String, SipMessageError> {
        named_header_param!(self.to_header(), "tag", SipMessageError::MissingToTag)
    }

    fn set_to_header_tag(&mut self, tag: String) {
        if let Ok(header) = self.to_header_mut() {
            header.set_param("tag", Some(tag));
        }
    }

    fn to_header_username(&self) -> Result<&String, SipMessageError> {
        named_header_username!(self.to_header(), SipMessageError::MissingToUsername)
    }

    fn via_header(&self) -> Result<&ViaHeader, SipMessageError> {
        header!(
            self.headers().0.iter(),
            Header::Via,
            SipMessageError::MissingViaHeader
        )
    }

    fn via_header_mut(&mut self) -> Result<&mut ViaHeader, SipMessageError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::Via,
            SipMessageError::MissingViaHeader
        )
    }

    fn via_header_branch(&self) -> Result<&String, SipMessageError> {
        if let Ok(header) = self.via_header() {
            header.branch().ok_or(SipMessageError::MissingViaBranch)
        } else {
            Err(SipMessageError::MissingViaBranch)
        }
    }

    fn call_id(&self) -> Result<&String, SipMessageError> {
        header!(
            self.headers().0.iter(),
            Header::CallId,
            SipMessageError::MissingCallIdHeader
        )
    }

    fn call_id_mut(&mut self) -> Result<&mut String, SipMessageError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::CallId,
            SipMessageError::MissingCallIdHeader
        )
    }

    fn cseq(&self) -> Result<(u32, Method), SipMessageError> {
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
            .ok_or(SipMessageError::MissingCSeqHeader)
    }

    fn cseq_mut(&mut self) -> Result<(&mut u32, &mut Method), SipMessageError> {
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
            .ok_or(SipMessageError::MissingCSeqHeader)
    }

    fn contact_header(&self) -> Result<&ContactHeader, SipMessageError> {
        header!(
            self.headers().0.iter(),
            Header::Contact,
            SipMessageError::MissingContactHeader
        )
    }

    fn contact_header_mut(&mut self) -> Result<&mut ContactHeader, SipMessageError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::Contact,
            SipMessageError::MissingContactHeader
        )
    }

    fn contact_header_username(&self) -> Result<&String, SipMessageError> {
        if let Ok(header) = self.contact_header() {
            if let Some(auth) = &header.uri.auth {
                Ok(&auth.username)
            } else {
                Err(SipMessageError::MissingContactUsername)
            }
        } else {
            Err(SipMessageError::MissingContactUsername)
        }
    }

    fn contact_header_expires(&self) -> Result<u32, SipMessageError> {
        // https://tools.ietf.org/html/rfc3261#page-228 "c-p-expires" defines that it must be unsigned number
        named_header_param!(
            self.contact_header(),
            "expires",
            SipMessageError::MissingContactExpires
        )
        .and_then(|expires| {
            expires
                .to_string()
                .parse::<u32>()
                .map_err(|_| SipMessageError::MissingContactExpires)
        })
    }

    fn expires_header(&self) -> Result<u32, SipMessageError> {
        header!(
            self.headers().0.iter(),
            Header::Expires,
            SipMessageError::MissingExpiresHeader
        )
        .map(Clone::clone)
    }

    fn expires_header_mut(&mut self) -> Result<&mut u32, SipMessageError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::Expires,
            SipMessageError::MissingExpiresHeader
        )
    }
}
