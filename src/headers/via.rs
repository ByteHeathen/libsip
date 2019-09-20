use crate::uri::Uri;
use crate::core::Version;
use crate::core::Transport;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct ViaHeader {
    pub version: Version,
    pub transport: Transport,
    pub uri: Uri
}

impl fmt::Display for ViaHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Via: {}/{} {}", self.version, self.transport, self.uri)
    }
}
