use nom::character::{is_alphanumeric, is_digit};
use serde::{Deserialize, Serialize};

use std::{fmt, net::Ipv4Addr};

use crate::parse::{parse_ip_address, parse_u16, slice_to_string};

/// Domain address for a URI.
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
    combinator::{opt, map_res}
};

pub fn parse_domain(input: &[u8]) -> IResult<&[u8], Domain> {
  alt((parse_ip_domain, parse_domain_domain))(input)
}

pub fn parse_ip_domain(input: &[u8]) -> IResult<&[u8], Domain> {
    let (input, addr) = parse_ip_address(input)?;
    let (input, _) = opt(char(':'))(input)?;
    let (input, port) = opt(map_res(take_while(is_digit), parse_u16))(input)?;
    Ok((input, Domain::Ipv4(addr, port)))
}

pub fn parse_domain_domain(input: &[u8]) -> IResult<&[u8], Domain> {
    let (input, domain) = map_res(take_while(|item| is_alphanumeric(item) || item == b'.'), slice_to_string)(input)?;
    let (input, _) = opt(char(':'))(input)?;
    let (input, port) = opt(map_res(take_while(is_digit), parse_u16))(input)?;
    Ok((input, Domain::Domain(domain, port)))
}