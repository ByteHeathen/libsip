use std::io::Result as IoResult;

use crate::headers::Headers;
use crate::headers::Header;
use crate::headers::NamedHeader;
use crate::headers::via::ViaHeader;
use crate::uri::Uri;
use crate::SipMessage;
use crate::ResponseGenerator;

macro_rules! impl_simple_header_method {
    ($name:ident, $variant:ident, $ty: ident) => {
        /// Retrieve value of the $variant header.
        pub fn $name(&self) -> IoResult<$ty> {
            if let Some(Header::$variant(header)) = self.headers.$name() {
                Ok(header)
            } else {
                Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, format!("invitiation doesnt contain a {} header", stringify!($variant))))
            }
        }
    }
}

/// Structure to ease getting data from a Sip INVITE request.
#[derive(Debug)]
pub struct InviteHelper {
    pub uri: Uri,
    pub headers: Headers,
    pub body: Vec<u8>
}

impl InviteHelper {

    pub fn new(uri: Uri, headers: Headers, body: Vec<u8>) -> IoResult<InviteHelper> {
        Ok(InviteHelper { uri, headers, body })
    }

    impl_simple_header_method!(from, From, NamedHeader);
    impl_simple_header_method!(to, To, NamedHeader);
    impl_simple_header_method!(call_id, CallId, String);
    impl_simple_header_method!(via, Via, ViaHeader);

    /// Return a clone of the body of this message.
    pub fn data(&self) -> Vec<u8> {
        self.body.clone()
    }

    /// Get A Ringing(180) request to answer this invite.
    pub fn ringing(&self) -> IoResult<SipMessage> {
        ResponseGenerator::new()
            .code(180)
            .header(self.headers.from().unwrap())
            .header(self.headers.to().unwrap())
            .header(self.headers.call_id().unwrap())
            .header(self.headers.cseq().unwrap())
            .header(self.headers.via().unwrap())
            .header(Header::ContentLength(0))
            .build()

    }

    /// Generate a response that will accept the invite with the sdp as the body.
    pub fn accept(&self, sdp: Vec<u8>) -> IoResult<SipMessage> {
        ResponseGenerator::new()
            .code(200)
            .header(self.headers.cseq().unwrap())
            .header(self.headers.via().unwrap())
            .header(self.headers.to().unwrap())
            .header(self.headers.from().unwrap())
            .body(sdp)
            .build()
    }

    pub fn check_cseq(&self, id: u32) -> IoResult<bool> {
        for header in self.headers.iter() {
            if let Header::CSeq(count, _) = header {
                if count == &id {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
