use serde::{Deserialize, Serialize};

use std::fmt;
use nom::{
    IResult,
    branch::alt,
    combinator::map,
    bytes::complete::tag_no_case
};

/// Sip URI Schema.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Schema {
    Sip,
    Sips,
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Schema::Sip => write!(f, "sip"),
            Schema::Sips => write!(f, "sips"),
        }
    }
}

/// Parse SIP URI schema. Only Accepts 'sip' and 'sips'.
pub fn parse_schema(input: &[u8]) -> IResult<&[u8], Schema> {
    alt((
        map(tag_no_case("sip"), |_| Schema::Sip),
        map(tag_no_case("sips"), |_| Schema::Sips)
    ))(input)
}
