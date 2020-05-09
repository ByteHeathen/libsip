use nom::character::is_digit;

use crate::parse::parse_u8;

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

use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::char as parse_char;
use nom::combinator::map_res;
use nom::bytes::complete::take_while1;

pub fn parse_version(input: &[u8]) -> IResult<&[u8], Version> {
    let (input, _) = tag("SIP/")(input)?;
    let (input, major) = map_res(take_while1(is_digit), parse_u8)(input)?;
    let (input, _) = parse_char('.')(input)?;
    let (input, minor) = map_res(take_while1(is_digit), parse_u8)(input)?;
    Ok((input, Version(major, minor)))
}