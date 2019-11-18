use crate::*;

use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

/// Sip Request Generator. When build is called the struct
/// is consumed and produces a SipMessage::Request variant.
/// Calling the `method` & `uri` methods before the `build`
/// method is required.
#[derive(Default)]
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
    pub fn method(mut self, method: Method) -> RequestGenerator {
        self.method = Some(method);
        self
    }

    /// Set the sip request uri.
    pub fn uri(mut self, uri: Uri) -> RequestGenerator {
        self.uri = Some(uri);
        self
    }

    /// Add multiple headers to the request header list.
    /// This use's Vec::extend so that the current items
    /// in the header list are kept.
    pub fn headers(mut self, headers: Vec<Header>) -> RequestGenerator {
        self.headers.extend(headers);
        self
    }

    /// Add a single header to the request header list.
    pub fn header(mut self, header: Header) -> RequestGenerator {
        self.headers.push(header);
        self
    }

    /// Set the sip request body. This completely replaces
    /// the current request body.
    pub fn body(mut self, body: Vec<u8>) -> RequestGenerator {
        self.body = body;
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
