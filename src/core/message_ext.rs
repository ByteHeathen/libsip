use crate::{Header, Method, NamedHeader, SipMessage, ViaHeader};

pub trait SipMessageExt {
    fn from_header(&self) -> Result<&NamedHeader, MissingFromHeaderError>;

    fn from_header_mut(&mut self) -> Result<&mut NamedHeader, MissingFromHeaderError>;

    fn from_header_tag(&self) -> Result<&String, MissingFromTagError>;

    fn set_from_header_tag(&mut self, tag: String);

    fn from_header_username(&self) -> Result<&String, MissingFromUsernameError>;

    fn to_header(&self) -> Result<&NamedHeader, MissingToHeaderError>;

    fn to_header_mut(&mut self) -> Result<&mut NamedHeader, MissingToHeaderError>;

    fn to_header_tag(&self) -> Result<&String, MissingToTagError>;

    fn set_to_header_tag(&mut self, tag: String);

    fn to_header_username(&self) -> Result<&String, MissingToUsernameError>;

    fn via_header(&self) -> Result<&ViaHeader, MissingViaHeaderError>;

    fn via_header_mut(&mut self) -> Result<&mut ViaHeader, MissingViaHeaderError>;

    fn via_header_branch(&self) -> Result<&String, MissingViaBranchError>;

    fn call_id(&self) -> Result<&String, MissingCallIdHeaderError>;

    fn call_id_mut(&mut self) -> Result<&mut String, MissingCallIdHeaderError>;

    fn cseq(&self) -> Result<(u32, Method), MissingCSeqHeaderError>;

    fn cseq_mut(&mut self) -> Result<(&mut u32, &mut Method), MissingCSeqHeaderError>;

    fn contact_header(&self) -> Result<&NamedHeader, MissingContactHeaderError>;

    fn contact_header_mut(&mut self) -> Result<&mut NamedHeader, MissingContactHeaderError>;

    /// Returns number of seconds if it's specified in the Contact header
    fn contact_header_expires(&self) -> Result<u32, MissingContactExpiresError>;

    fn expires_header(&self) -> Result<u32, MissingExpiresHeaderError>;

    fn expires_header_mut(&mut self) -> Result<&mut u32, MissingExpiresHeaderError>;
}

pub struct MissingFromHeaderError;
pub struct MissingFromTagError;
pub struct MissingFromUsernameError;
pub struct MissingToHeaderError;
pub struct MissingToTagError;
pub struct MissingToUsernameError;
pub struct MissingViaHeaderError;
pub struct MissingViaBranchError;
pub struct MissingCallIdHeaderError;
pub struct MissingCSeqHeaderError;
pub struct MissingContactHeaderError;
pub struct MissingContactExpiresError;
pub struct MissingExpiresHeaderError;

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
    fn from_header(&self) -> Result<&NamedHeader, MissingFromHeaderError> {
        header!(
            self.headers().0.iter(),
            Header::From,
            MissingFromHeaderError
        )
    }

    fn from_header_mut(&mut self) -> Result<&mut NamedHeader, MissingFromHeaderError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::From,
            MissingFromHeaderError
        )
    }

    fn from_header_tag(&self) -> Result<&String, MissingFromTagError> {
        named_header_param!(self.from_header(), "tag", MissingFromTagError)
    }

    fn set_from_header_tag(&mut self, tag: String) {
        if let Ok(header) = self.from_header_mut() {
            header.set_param("tag", Some(tag));
        }
    }

    fn from_header_username(&self) -> Result<&String, MissingFromUsernameError> {
        named_header_username!(self.from_header(), MissingFromUsernameError)
    }

    fn to_header(&self) -> Result<&NamedHeader, MissingToHeaderError> {
        header!(self.headers().0.iter(), Header::To, MissingToHeaderError)
    }

    fn to_header_mut(&mut self) -> Result<&mut NamedHeader, MissingToHeaderError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::To,
            MissingToHeaderError
        )
    }

    fn to_header_tag(&self) -> Result<&String, MissingToTagError> {
        named_header_param!(self.to_header(), "tag", MissingToTagError)
    }

    fn set_to_header_tag(&mut self, tag: String) {
        if let Ok(header) = self.to_header_mut() {
            header.set_param("tag", Some(tag));
        }
    }

    fn to_header_username(&self) -> Result<&String, MissingToUsernameError> {
        named_header_username!(self.to_header(), MissingToUsernameError)
    }

    fn via_header(&self) -> Result<&ViaHeader, MissingViaHeaderError> {
        header!(self.headers().0.iter(), Header::Via, MissingViaHeaderError)
    }

    fn via_header_mut(&mut self) -> Result<&mut ViaHeader, MissingViaHeaderError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::Via,
            MissingViaHeaderError
        )
    }

    fn via_header_branch(&self) -> Result<&String, MissingViaBranchError> {
        if let Ok(header) = self.via_header() {
            header.branch().ok_or(MissingViaBranchError)
        } else {
            Err(MissingViaBranchError)
        }
    }

    fn call_id(&self) -> Result<&String, MissingCallIdHeaderError> {
        header!(
            self.headers().0.iter(),
            Header::CallId,
            MissingCallIdHeaderError
        )
    }

    fn call_id_mut(&mut self) -> Result<&mut String, MissingCallIdHeaderError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::CallId,
            MissingCallIdHeaderError
        )
    }

    fn cseq(&self) -> Result<(u32, Method), MissingCSeqHeaderError> {
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
            .ok_or(MissingCSeqHeaderError)
    }

    fn cseq_mut(&mut self) -> Result<(&mut u32, &mut Method), MissingCSeqHeaderError> {
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
            .ok_or(MissingCSeqHeaderError)
    }

    fn contact_header(&self) -> Result<&NamedHeader, MissingContactHeaderError> {
        header!(
            self.headers().0.iter(),
            Header::Contact,
            MissingContactHeaderError
        )
    }

    fn contact_header_mut(&mut self) -> Result<&mut NamedHeader, MissingContactHeaderError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::Contact,
            MissingContactHeaderError
        )
    }

    fn contact_header_expires(&self) -> Result<u32, MissingContactExpiresError> {
        // https://tools.ietf.org/html/rfc3261#page-228 "c-p-expires" defines that it must be unsigned number
        named_header_param!(self.contact_header(), "expires", MissingContactExpiresError).and_then(
            |expires| {
                expires
                    .parse::<u32>()
                    .map_err(|_| MissingContactExpiresError)
            },
        )
    }

    fn expires_header(&self) -> Result<u32, MissingExpiresHeaderError> {
        header!(
            self.headers().0.iter(),
            Header::Expires,
            MissingExpiresHeaderError
        )
        .map(Clone::clone)
    }

    fn expires_header_mut(&mut self) -> Result<&mut u32, MissingExpiresHeaderError> {
        header!(
            self.headers_mut().0.iter_mut(),
            Header::Expires,
            MissingExpiresHeaderError
        )
    }
}
