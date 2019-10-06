use crate::*;

use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

/// Sip Response Generator. When build is called the struct
/// is consumed and produces a SipMessage::Response variant.
/// Calling the `code` method before the `build` method is
/// required.
pub struct ResponseGenerator {
    code: Option<u32>,
    version: Version,
    headers: Headers,
    body: Vec<u8>
}

impl ResponseGenerator {

    pub fn new() -> ResponseGenerator {
        ResponseGenerator {
            code: None,
            version: Version::default(),
            headers: Headers::new(),
            body: vec![]
        }
    }

    /// Set the response status code.
    pub fn code(mut self, code: u32) -> ResponseGenerator {
        self.code = Some(code);
        self
    }

    /// Add multiple headers to the response header list.
    /// This use's Vec::extend so that the current items
    /// in the header list are kept.
    pub fn headers(mut self, headers: Vec<Header>) -> ResponseGenerator {
        self.headers.extend(headers);
        self
    }

    /// Add a single header to the response header list.
    pub fn header(mut self, header: Header) -> ResponseGenerator {
        self.headers.push(header);
        self
    }

    /// Set the sip response body. This completely replaces
    /// the current response body.
    pub fn body(mut self, body: Vec<u8>) -> ResponseGenerator {
        self.body = body;
        self
    }

    /// Create the Sip response.
    pub fn build(self) -> IoResult<SipMessage> {
        if let Some(code) = self.code {

            let res = SipMessage::Response {
                code: code,
                version: self.version,
                headers: self.headers,
                body: self.body
            };
            Ok(res)
        } else {
            Err(IoError::new(IoErrorKind::InvalidInput, "ResponseGenerator requires `code` method be called."))
        }
    }
}
