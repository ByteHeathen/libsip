use serde::{Deserialize, Serialize};
use nom::IResult;
use nom::branch::alt;
use nom::combinator::map;
use nom::bytes::complete::tag_no_case;

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

pub fn parse_transport(input: &[u8]) -> IResult<&[u8], Transport> {
    alt((
        map(tag_no_case("TCP"), |_| Transport::Tcp),
        map(tag_no_case("UDP"), |_| Transport::Udp)
    ))(input)
}