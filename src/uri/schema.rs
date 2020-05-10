use serde::{Deserialize, Serialize};

use std::fmt;
use nom::{
    IResult,
    branch::alt,
    combinator::map,
    error::ParseError,
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
pub fn parse_schema<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Schema, E> {
    alt::<_, _, E, _>((
        map(tag_no_case::<_, _, E>("sip"), |_| Schema::Sip),
        map(tag_no_case::<_, _, E>("sips"), |_| Schema::Sips)
    ))(input)
}
