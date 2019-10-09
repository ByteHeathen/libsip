use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

use crate::core::Method;
use crate::core::Transport;
use crate::headers::ContentType;
use crate::headers::Headers;
use crate::headers::Header;
use crate::headers::NamedHeader;
use crate::headers::via::ViaHeader;
use crate::uri::Uri;
use crate::uri::Schema;
use crate::SipMessage;
use crate::ResponseGenerator;
use crate::RequestGenerator;
use super::RegistrationManager;

macro_rules! impl_simple_header_method {
    ($name:ident, $variant:ident, $ty: ident) => {
        pub fn $name(&self) -> IoResult<$ty> {
            if let Some(Header::$variant(header)) = self.headers.$name() {
                Ok(header)
            } else {
                Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, format!("message doesnt contain a {} header", stringify!($variant))))
            }
        }
    }
}

pub struct MessageHelper {
    pub uri: Uri,
    pub headers: Headers,
    pub body: Vec<u8>
}

impl MessageHelper {

    pub fn new(uri: Uri, headers: Headers, body: Vec<u8>) -> IoResult<MessageHelper> {
        Ok(MessageHelper { uri, headers, body })
    }

    impl_simple_header_method!(from, From, NamedHeader);
    impl_simple_header_method!(to, To, NamedHeader);
    impl_simple_header_method!(contact, Contact, NamedHeader);
    impl_simple_header_method!(call_id, CallId, String);
    impl_simple_header_method!(xfs_sending_message, XFsSendingMessage, String);
    impl_simple_header_method!(via, Via, ViaHeader);

    pub fn data(&self) -> Vec<u8> {
        self.body.clone()
    }

    pub fn received(&self) -> IoResult<SipMessage> {
        ResponseGenerator::new()
            .code(200)
            .header(self.headers.from().unwrap())
            .header(self.headers.to().unwrap())
            .header(self.headers.call_id().unwrap())
            .header(self.headers.cseq().unwrap())
            .header(self.headers.via().unwrap())
            .header(Header::ContentLength(0))
            .build()

    }
}

pub struct MessageWriter {
    cseq: u32,
    uri: Uri,
    call_id: String
}

impl MessageWriter {

    pub fn new(uri: Uri) -> MessageWriter {
        let _call_id = md5::compute(rand::random::<[u8 ; 16]>());
        let call_id = format!("{:x}@{}", _call_id, uri.host);
        MessageWriter {
            cseq: 0,
            uri, call_id
        }
    }

    pub fn write_message(&mut self, body: Vec<u8>, to: Uri, reg: Header) -> IoResult<SipMessage> {
        RequestGenerator::new()
            .method(Method::Message)
            .uri(to.clone().schema(Schema::Sip))
            .header(reg)
            .header(Header::MaxForwards(70))
            .header(Header::To(NamedHeader::new(to)))
            .header(Header::From(NamedHeader::new(self.uri.clone())))
            .header(Header::CallId(self.call_id.clone()))
            .header(self.cseq())
            .header(Header::UserAgent(format!("libsip {}", env!("CARGO_PKG_VERSION"))))
            .header(Header::ContentType(ContentType::PlainText))
            .header(Header::ContentLength(body.len() as u32))
            .body(body)
            .build()

    }

    fn cseq(&self) -> Header {
        Header::CSeq(self.cseq, Method::Message)
    }
}
