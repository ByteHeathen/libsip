use crate::{Header, Method, NamedHeader, SipMessage, UriParam, ViaHeader};

pub trait SipMessageExt {
    fn from_header(&self) -> Option<&NamedHeader>;

    fn from_header_mut(&mut self) -> Option<&mut NamedHeader>;

    fn from_header_tag(&self) -> Option<&String>;

    fn from_header_username(&self) -> Option<&String>;

    fn to_header(&self) -> Option<&NamedHeader>;

    fn to_header_mut(&mut self) -> Option<&mut NamedHeader>;

    fn to_header_tag(&self) -> Option<&String>;

    fn to_header_username(&self) -> Option<&String>;

    fn via_header(&self) -> Option<&ViaHeader>;

    fn via_header_mut(&mut self) -> Option<&mut ViaHeader>;

    fn via_header_branch(&self) -> Option<&String>;

    fn call_id(&self) -> Option<&String>;

    fn call_id_mut(&mut self) -> Option<&mut String>;

    fn cseq(&self) -> Option<(u32, Method)>;

    fn cseq_mut(&mut self) -> Option<(&mut u32, &mut Method)>;

    fn contact_header(&self) -> Option<&NamedHeader>;

    fn contact_header_mut(&mut self) -> Option<&mut NamedHeader>;

    /// Returns number of seconds if it's specified in the Contact header
    fn contact_header_expires(&self) -> Option<u32>;
}

macro_rules! header {
    ($iter:expr, $header:path) => {
        $iter.find_map(|header| {
            if let $header(header) = header {
                Some(header)
            } else {
                None
            }
        })
    };
}

macro_rules! named_header_param {
    ($header:expr, $param:expr) => {
        $header.and_then(|header| {
            if let Some(param) = header.parameters.get($param) {
                param.as_ref()
            } else {
                None
            }
        })
    };
}

impl SipMessageExt for SipMessage {
    fn from_header(&self) -> Option<&NamedHeader> {
        header!(self.headers().0.iter(), Header::From)
    }

    fn from_header_mut(&mut self) -> Option<&mut NamedHeader> {
        header!(self.headers_mut().0.iter_mut(), Header::From)
    }

    fn from_header_tag(&self) -> Option<&String> {
        named_header_param!(self.from_header(), "tag")
    }

    fn from_header_username(&self) -> Option<&String> {
        self.from_header()
            .and_then(|header| header.uri.auth.as_ref().map(|auth| &auth.username))
    }

    fn to_header(&self) -> Option<&NamedHeader> {
        header!(self.headers().0.iter(), Header::To)
    }

    fn to_header_mut(&mut self) -> Option<&mut NamedHeader> {
        header!(self.headers_mut().0.iter_mut(), Header::To)
    }

    fn to_header_tag(&self) -> Option<&String> {
        named_header_param!(self.to_header(), "tag")
    }

    fn to_header_username(&self) -> Option<&String> {
        self.to_header()
            .and_then(|header| header.uri.auth.as_ref().map(|auth| &auth.username))
    }

    fn via_header(&self) -> Option<&ViaHeader> {
        header!(self.headers().0.iter(), Header::Via)
    }

    fn via_header_mut(&mut self) -> Option<&mut ViaHeader> {
        header!(self.headers_mut().0.iter_mut(), Header::Via)
    }

    fn via_header_branch(&self) -> Option<&String> {
        self.via_header().and_then(|header| {
            header.uri.parameters.iter().find_map(|param| {
                if let UriParam::Branch(branch) = param {
                    Some(branch)
                } else {
                    None
                }
            })
        })
    }

    fn call_id(&self) -> Option<&String> {
        header!(self.headers().0.iter(), Header::CallId)
    }

    fn call_id_mut(&mut self) -> Option<&mut String> {
        header!(self.headers_mut().0.iter_mut(), Header::CallId)
    }

    fn cseq(&self) -> Option<(u32, Method)> {
        self.headers().0.iter().find_map(|header| {
            if let Header::CSeq(cseq, method) = header {
                Some((*cseq, *method))
            } else {
                None
            }
        })
    }

    fn cseq_mut(&mut self) -> Option<(&mut u32, &mut Method)> {
        self.headers_mut().0.iter_mut().find_map(|header| {
            if let Header::CSeq(cseq, method) = header {
                Some((cseq, method))
            } else {
                None
            }
        })
    }

    fn contact_header(&self) -> Option<&NamedHeader> {
        header!(self.headers().0.iter(), Header::Contact)
    }

    fn contact_header_mut(&mut self) -> Option<&mut NamedHeader> {
        header!(self.headers_mut().0.iter_mut(), Header::Contact)
    }

    fn contact_header_expires(&self) -> Option<u32> {
        // https://tools.ietf.org/html/rfc3261#page-228 "c-p-expires" defines that it must be unsigned number
        named_header_param!(self.contact_header(), "expires")
            .and_then(|expires| expires.parse::<u32>().ok())
    }
}
