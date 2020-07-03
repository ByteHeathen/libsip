use crate::parse::parse_u8;
use nom::{
    bytes::complete::{tag, take_while1},
    character::{complete::char as parse_char, is_digit},
    combinator::map_res,
    error::ParseError,
    IResult,
};

use std::fmt;

/// SIP Protocol version struct.
/// default: 2.0
#[derive(Debug, PartialEq, Clone)]
pub struct Version(u8, u8);

impl Default for Version {
    fn default() -> Version {
        Version(2, 0)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SIP/{}.{}", self.0, self.1)
    }
}

impl Version {
    pub fn new(maj: u8, min: u8) -> Version {
        Version(maj, min)
    }
}

/// Parse the SIP protocol version.
pub fn parse_version<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], Version, E> {
    let (input, _) = tag("SIP/")(input)?;
    let (input, major) = map_res(take_while1(is_digit), parse_u8)(input)?;
    let (input, _) = parse_char('.')(input)?;
    let (input, minor) = map_res(take_while1(is_digit), parse_u8)(input)?;
    Ok((input, Version(major, minor)))
}
