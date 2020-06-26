use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{
    core::{is_token, parse_transport, Transport},
    uri::{parse_domain, parse_port, Domain},
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::is_alphabetic,
    combinator::map,
    error::ParseError,
    IResult,
};

/// Uri Parameters.
///
/// TODO: Expand this enum. Similar to `libsip::Header`
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum UriParam {
    Transport(Transport),
    Branch(String),
    Received(Domain),
    RPort(Option<u16>),
    Other(String, Option<String>),
}

impl UriParam {
    /// Create `UriParam` from a key value pair.
    pub fn from_key<'a, E: ParseError<&'a [u8]>>(
        key: &'a [u8],
        value: &'a [u8],
    ) -> Result<UriParam, nom::Err<E>> {
        match key {
            b"rport" => Ok(UriParam::RPort(parse_port::<E>(&value)?.1)),
            b"transport" => Ok(UriParam::Transport(parse_transport::<E>(&value)?.1)),
            b"branch" => Ok(UriParam::Branch(
                String::from_utf8(value.to_vec()).expect("Utf-8 Error"),
            )),
            b"received" => {
                //let mut data = value.to_vec();
                //data.push(b' ');
                Ok(UriParam::Received(parse_domain::<E>(&value)?.1))
            },
            _method => Ok(UriParam::Other(
                String::from_utf8_lossy(key).to_string(),
                Some(String::from_utf8_lossy(value).to_string()),
            )),
        }
    }
}

impl fmt::Display for UriParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UriParam::Transport(trans) => write!(f, ";transport={}", trans),
            UriParam::Branch(branch) => write!(f, ";branch={}", branch),
            UriParam::Received(branch) => write!(f, ";received={}", branch),
            UriParam::RPort(Some(value)) => write!(f, ";rport={}", value),
            UriParam::RPort(None) => write!(f, ";rport"),
            UriParam::Other(key, Some(value)) => write!(f, ";{}={}", key, value),
            UriParam::Other(key, None) => write!(f, ";{}", key),
        }
    }
}

pub fn parse_param<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], UriParam, E> {
    alt::<_, _, E, _>((
        parse_named_param,
        map(tag::<_, _, E>(";rport"), |_| UriParam::RPort(None)),
        parse_single_param,
    ))(input)
}

/// Parse a single named field param.
pub fn parse_named_param<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], UriParam, E> {
    let (input, _) = tag(";")(input)?;
    let (input, key) = take_while(is_alphabetic)(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, value) = take_while(is_token)(input)?;
    UriParam::from_key::<E>(key, value).and_then(|item| Ok((input, item)))
}

pub fn parse_single_param<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], UriParam, E> {
    let (input, _) = tag(";")(input)?;
    let (input, key) = take_while(is_alphabetic)(input)?;
    Ok((
        input,
        UriParam::Other(String::from_utf8_lossy(key).into(), None),
    ))
}

/// Parse multiple uri parameters.
pub fn parse_params<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], Vec<UriParam>, E> {
    let mut results = vec![];
    let mut data = input;

    while let Ok((remains, param)) = parse_param::<E>(&data) {
        results.push(param);
        data = remains;
    }
    Ok((data, results))
}
