use crate::*;

use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

/// Sip Request Generator. When build is called the struct
/// is consumed and produces a SipMessage::Request variant.
pub struct RequestGenerator {
    method: Option<Method>,
    uri: Option<Uri>,
    version: Version,
    headers: Headers,
    body: Vec<u8>
}

impl RequestGenerator {

    pub fn new() -> RequestGenerator {
        RequestGenerator {
            method: None,
            uri: None,
            version: Version::default(),
            headers: Headers::new(),
            body: vec![]
        }
    }

    /// Set the sip request method.
    pub fn method(mut self, m: Method) -> RequestGenerator {
        self.method = Some(m);
        self
    }

    /// Set the sip request uri.
    pub fn uri(mut self, u: Uri) -> RequestGenerator {
        self.uri = Some(u);
        self
    }

    /// Add multiple headers to the request header list.
    pub fn headers(mut self, h: Vec<Header>) -> RequestGenerator {
        self.headers.extend(h);
        self
    }

    /// Add a single header to the request header list.
    pub fn header(mut self, h: Header) -> RequestGenerator {
        self.headers.push(h);
        self
    }

    /// Set the sip request body.
    pub fn body(mut self, b: Vec<u8>) -> RequestGenerator {
        self.body = b;
        self
    }

    /// Build the sip request.
    pub fn build(self) -> IoResult<SipMessage> {
        if let Some(method) = self.method {
            if let Some(uri) = self.uri {
                Ok(SipMessage::Request {
                    method, uri,
                    version: self.version,
                    headers: self.headers,
                    body: self.body
                })
            } else {
                Err(IoError::new(IoErrorKind::InvalidInput, "`uri` method call required"))
            }
        } else {
            Err(IoError::new(IoErrorKind::InvalidInput, "`method` method call required"))
        }
    }
}
