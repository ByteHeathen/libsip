#![allow(dead_code)]
use std::io::{
    Result as IoResult,
    Error as IoError,
    ErrorKind as IoErrorKind
};

use crate::*;

macro_rules! impl_simple_header_method {
    ($name:ident, $variant:ident, $ty: ident) => {
        /// Retrieve value of the $variant header.
        pub fn $name(&self) -> IoResult<$ty> {
            if let Some(Header::$variant(header)) = self.headers.$name() {
                Ok(header)
            } else {
                Err(IoError::new(IoErrorKind::InvalidInput, format!("invitiation doesnt contain a {} header", stringify!($variant))))
            }
        }
    }
}

/// Structure to ease getting data from a Sip INVITE request.
/// Also is used to generate the appropiate Ringing and Ok
/// responses.
#[derive(Debug)]
pub struct InviteHelper {
    pub uri: Uri,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl InviteHelper {

    impl_simple_header_method!(from, From, NamedHeader);

    impl_simple_header_method!(to, To, NamedHeader);

    impl_simple_header_method!(call_id, CallId, String);

    impl_simple_header_method!(via, Via, ViaHeader);

    /// Create a InviteHelper from the given SipMessage.
    pub fn new(msg: SipMessage) -> IoResult<InviteHelper> {
        match msg {
            SipMessage::Request { uri, headers, body, .. } => {
                InviteHelper::new_from_vars(uri, headers, body)
            },
            SipMessage::Response { .. } => {
                Err(IoError::new(IoErrorKind::InvalidData, "Expected a SIP request"))
            }
        }
    }
    /// Create an InviteHelper from the given variables.
    pub fn new_from_vars(uri: Uri, headers: Headers, body: Vec<u8>) -> IoResult<InviteHelper> {
        Ok(InviteHelper { uri, headers, body })
    }

    /// Return a clone of the body of this message.
    pub fn data(&self) -> Vec<u8> {
        self.body.clone()
    }

    /// Get A Ringing(180) request to answer this invite.
    pub fn ringing(&self, header_cfg: &HeaderWriteConfig) -> IoResult<SipMessage> {
        let mut req = ResponseGenerator::new()
            .code(180)
            .header(self.headers.from().unwrap())
            .header(self.headers.to().unwrap())
            .header(self.headers.call_id().unwrap())
            .header(self.headers.cseq().unwrap())
            .header(self.headers.via().unwrap())
            .header(Header::ContentLength(0));
        header_cfg.write_headers(req.headers_ref_mut());
        req.build()
    }

    /// Generate a response that will accept the invite with the sdp as the body.
    pub fn accept(&self, sdp: Vec<u8>, header_cfg: &HeaderWriteConfig) -> IoResult<SipMessage> {
        let mut req = ResponseGenerator::new()
            .code(200)
            .header(self.headers.cseq().unwrap())
            .header(self.headers.via().unwrap())
            .header(self.headers.to().unwrap())
            .header(self.headers.from().unwrap())
            .header(self.headers.call_id().unwrap())
            .header(Header::ContentLength(sdp.len() as u32))
            .body(sdp);
        header_cfg.write_headers(req.headers_ref_mut());
        req.build()
    }
    
    /// Generate a Bye response for this Invite Request.
    pub fn bye(&self, header_cfg: &HeaderWriteConfig) -> IoResult<SipMessage> {
        let mut req = RequestGenerator::new()
            .method(Method::Bye)
            .uri(self.uri.clone())
            .header(self.headers.cseq().unwrap())
            .header(self.headers.via().unwrap())
            .header(self.headers.to().unwrap())
            .header(self.headers.from().unwrap())
            .header(self.headers.call_id().unwrap());
        header_cfg.write_headers(req.headers_ref_mut());
        req.build()
    }

    /// Verify the CSeq header is equal to `cseq`.
    pub fn check_cseq(&self, cseq: u32) -> IoResult<bool> {
        for header in self.headers.iter() {
            if let Header::CSeq(count, _) = header {
                if count == &cseq {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

        /// Get the messages required to cancel a invitation.
    pub fn cancel(&mut self, header_cfg: &HeaderWriteConfig) -> IoResult<(SipMessage, SipMessage)> {
        let mut out_headers = vec![];
        for header in self.headers.iter() {
            match header {
                Header::CSeq(a, b) => out_headers.push(Header::CSeq(*a, *b)),
                Header::CallId(call) => out_headers.push(Header::CallId(call.clone())),
                Header::From(from) => out_headers.push(Header::From(from.clone())),
                Header::To(to) => out_headers.push(Header::To(to.clone())),
                Header::Via(via) => out_headers.push(Header::Via(via.clone())),
                _ => {},
            }
        }
        let mut final_headers = Headers(out_headers);
        header_cfg.write_headers(&mut final_headers);

        Ok((
            ResponseGenerator::new()
                .code(200)
                .headers(final_headers.clone().0)
                .header(Header::ContentLength(0))
                .build()?,
            ResponseGenerator::new()
                .code(487)
                .headers(final_headers.0)
                .header(Header::ContentLength(0))
                .build()?,
        ))
    }
}


/// The InviteWriter Helps create an Invite Request
/// sent to the `uri` given in the new function.
#[derive(Debug)]
pub struct InviteWriter {
    cseq: u32,
    uri: Uri
}

impl InviteWriter {

    /// Create a new InviteHelper struct. `uri` is the uri to send
    /// the request too.
    pub fn new(uri: Uri) -> InviteWriter {
        InviteWriter {
            cseq: 0,
            uri
        }
    }

    /// Generate a Invite Request.
    pub fn generate_invite(&mut self, uri: Uri, sdp: Vec<u8>) -> IoResult<SipMessage> {
        self.cseq += 1;
        let me_uri = self.uri.clone();
        RequestGenerator::new()
            .method(Method::Invite)
            .uri(uri.clone())
            .header(self.cseq()?)
            .header(Header::From(__named_header!(me_uri)))
            .header(Header::To(__named_header!(uri)))
            .header(Header::CallId(InviteWriter::generate_call_id()))
            .body(sdp)
            .build()
    }

    /// Generate a CSeq header.
    pub fn cseq(&self) -> IoResult<Header> {
        let h = Header::CSeq(self.cseq, Method::Invite);
        Ok(h)
    }

    /// Generate a new CallId value. This is calculated as an
    /// MD5 hash of a randomly generated 16 byte sequence.
    pub fn generate_call_id() -> String {
        format!("{:x}", md5::compute(rand::random::<[u8; 16]>()))
    }
}
