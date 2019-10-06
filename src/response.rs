use crate::*;

use std::io::Result as IoResult;

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

    pub fn code(mut self, code: u32) -> ResponseGenerator {
        self.code = code;
        self
    }

    pub fn headers(mut self, h: Vec<Header>) -> ResponseGenerator {
        self.headers.extend(h);
        self
    }

    pub fn header(mut self, h: Header) -> ResponseGenerator {
        self.headers.push(h);
        self
    }

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
