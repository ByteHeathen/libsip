use crate::core::Transport;
use crate::core::Version;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct ViaHeader {
    pub version: Version,
    pub transport: Transport,
    pub uri: String
}

impl fmt::Display for ViaHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Via: {}/{} {}", self.version, self.transport, self.uri)
    }
}
