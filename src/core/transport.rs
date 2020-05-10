use serde::{Deserialize, Serialize};
use nom::{
    IResult,
    branch::alt,
    combinator::map,
    bytes::complete::tag_no_case,
    error::ParseError
};

use std::fmt;

/// SIP protocol transport.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Transport {
    Udp,
    Tcp,
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
            Transport::Tcp => write!(f, "TCP"),
        }
    }
}

/// Parse a SIP message transport protocol.
pub fn parse_transport<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Transport, E> {
    alt::<_, _, E, _>((
        map(tag_no_case::<_, _, E>("TCP"), |_| Transport::Tcp),
        map(tag_no_case::<_, _, E>("UDP"), |_| Transport::Udp)
    ))(input)
}