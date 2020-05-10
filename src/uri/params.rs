use nom::{
    character::{is_alphabetic, is_alphanumeric},
    error::ParseError
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{
    core::{parse_transport, Transport},
    uri::{parse_domain, Domain},
};

/// Uri Parameters.
///
/// TODO: Expand this enum. Similar to `libsip::Header`
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Param {
    Transport(Transport),
    Branch(String),
    Received(Domain),
    RPort,
    Other(String, String)
}

impl Param {
    /// Create `Param` from a key value pair.
    pub fn from_key<'a, E: ParseError<&'a [u8]>>(
        key: &'a [u8],
        value: &'a [u8],
    ) -> Result<Param, nom::Err<E>> {
        match key {
            b"transport" => Ok(Param::Transport(parse_transport::<E>(&value)?.1)),
            b"branch" => Ok(Param::Branch(
                String::from_utf8(value.to_vec()).expect("Utf-8 Error"),
            )),
            b"received" => {
                //let mut data = value.to_vec();
                //data.push(b' ');
                Ok(Param::Received(parse_domain::<E>(&value)?.1))
            },
            _method => Ok(Param::Other(
                String::from_utf8_lossy(key).to_string(),
                String::from_utf8_lossy(value).to_string()
            )),
        }
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Param::Transport(trans) => write!(f, ";transport={}", trans),
            Param::Branch(branch) => write!(f, ";branch={}", branch),
            Param::Received(branch) => write!(f, ";received={}", branch),
            Param::RPort => write!(f, ";rport"),
            Param::Other(key, value) => write!(f, ";{}={}", key, value)
        }
    }
}

use nom::{
    IResult,
    bytes::complete::{ take_while, tag},
    combinator::map,
    branch::alt,
};

pub fn parse_param<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Param, E> {
    alt::<_, _, E, _>(
        (map(tag::<_, _, E>(";rport"), |_| Param::RPort), parse_named_param)
    )(input)
}

/// Parse a single named field param.
pub fn parse_named_param<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Param, E> {
    let (input, _) = tag(";")(input)?;
    let (input, key) = take_while(is_alphabetic)(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, value) = take_while(|item| is_alphanumeric(item) || b'.' == item)(input)?;
    Param::from_key::<E>(key, value)
        .and_then(|item| Ok((input, item)))
}

/// Parse multiple uri parameters.
pub fn parse_params<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Vec<Param>, E> {
    let mut results = vec![];
    let mut data = input;

    while let Ok((remains, param)) = parse_param::<E>(&data) {
        results.push(param);
        data = remains;
    }
    Ok((data, results))
}
