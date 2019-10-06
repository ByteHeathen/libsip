use crate::*;

use std::io::Result as IoResult;

/// Sip Response Generator. When build is called the struct
/// is consumed and produces a SipMessage::Response variant.
pub struct ResponseGenerator {
    code: u32,
    version: Version,
    headers: Headers,
    body: Vec<u8>
}

impl ResponseGenerator {

    pub fn new() -> ResponseGenerator {
        ResponseGenerator {
            code: 200,
            version: Version::default(),
            headers: Headers::new(),
            body: vec![]
        }
    }

    /// Set the response status code.
    pub fn code(mut self, code: u32) -> ResponseGenerator {
        self.code = code;
        self
    }

    /// Add multiple headers to the response header list.
    pub fn headers(mut self, h: Vec<Header>) -> ResponseGenerator {
        self.headers.extend(h);
        self
    }

    /// Add a single header to the response header list.
    pub fn header(mut self, h: Header) -> ResponseGenerator {
        self.headers.push(h);
        self
    }

    /// Create the Sip response.
    pub fn build(self) -> IoResult<SipMessage> {
        let res = SipMessage::Response {
            code: self.code,
            version: self.version,
            headers: self.headers,
            body: self.body
        };
        Ok(res)
    }
}
