use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Transport {
    Udp,
    Tcp
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
