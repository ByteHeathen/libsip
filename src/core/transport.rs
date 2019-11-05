use serde::{ Serialize, Deserialize };

use std::fmt;

/// SIP protocol transport.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Transport {
    Udp,
    Tcp
}

impl Default for Transport {
    fn default() -> Transport {
        Transport::Udp
    }
}

impl fmt::Display for Transport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Transport::Udp => write!(f, "UDP"),
            Transport::Tcp => write!(f, "TCP")
        }
    }
}

named!(pub parse_transport<Transport>, alt!(
    map!(tag!("TCP"), |_| Transport::Tcp) |
    map!(tag!("UDP"), |_| Transport::Udp)
));
