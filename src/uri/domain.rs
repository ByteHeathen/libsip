use nom::character::{
    is_alphanumeric,
    is_digit
};
use serde::{Deserialize, Serialize};

use std::{fmt, net::Ipv4Addr};

use crate::parse::{parse_ip_address, parse_u16, slice_to_string};

/// Domain address for a URI. Currently Only Ipv4 and
/// domains are implemented.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Domain {
    Ipv4(Ipv4Addr, Option<u16>),
    Domain(String, Option<u16>),
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Domain::Ipv4(addr, port) => {
                if let Some(port) = port {
                    write!(f, "{}:{}", addr, port)
                } else {
                    write!(f, "{}", addr)
                }
            },
            Domain::Domain(domain, port) => {
                if let Some(port) = port {
                    write!(f, "{}:{}", domain, port)
                } else {
                    write!(f, "{}", domain)
                }
            },
        }
    }
}

use nom::{
    IResult,
    branch::alt,
    character::complete::char,
    bytes::complete::take_while,
    combinator::{opt, map_res},
    error::ParseError
};

pub fn parse_port<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Option<u16>, E> {
    let (input, port) = opt(map_res::<_, _, _, _, E, _, _>(take_while::<_, _, E>(is_digit), parse_u16::<E>))(input)?;
    Ok((input, port))
}

pub fn parse_domain<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Domain, E> {
  alt((parse_ip_domain::<E>, parse_domain_domain::<E>))(input)
}

pub fn parse_ip_domain<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Domain, E> {
    let (input, addr) = parse_ip_address::<E>(input)?;
    let (input, _) = opt::<_, _, E, _>(char::<_, E>(':'))(input)?;
    let (input, port) = parse_port::<E>(input)?;
    Ok((input, Domain::Ipv4(addr, port)))
}

pub fn parse_domain_domain<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Domain, E> {
    let (input, domain) = map_res::<_, _, _, _, E, _, _>(
        take_while::<_, _, E>(|item| is_alphanumeric(item) || item == b'.'), slice_to_string::<E>)(input)?;
    let (input, _) = opt::<_, _, E, _>(char::<_, E>(':'))(input)?;
    let (input, port) = opt::<_, _, E, _>(map_res::<_, _, _, _, E, _, _>(take_while::<_, _, E>(is_digit), parse_u16::<E>))(input)?;
    Ok((input, Domain::Domain(domain, port)))
}