use nom::character::{ is_alphabetic, is_alphanumeric };
use nom::Err;
use nom::error::ErrorKind;
use serde::{ Serialize, Deserialize };
use std::fmt;

use crate::parse::ParserResult;
use crate::core::Transport;
use crate::core::parse_transport;
use crate::uri::Domain;
use crate::uri::parse_domain;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Param {
    Transport(Transport),
    Branch(String),
    Received(Domain),
    RPort
}

impl Param {

    pub fn from_key<'a>(key: &'a [u8], value: &'a [u8]) -> Result<Param, nom::Err<(&'a [u8], ErrorKind)>> {
        match key {
            b"transport" => Ok(Param::Transport(parse_transport(&value)?.1)),
            b"branch" => Ok(Param::Branch(String::from_utf8(value.to_vec()).expect("Utf-8 Error"))),
            b"received" => Ok(Param::Received(parse_domain(&value)?.1)),
            _method => Err(Err::Failure((key, ErrorKind::MapRes)))
        }
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Param::Transport(trans) => write!(f, ";transport={}", trans),
            Param::Branch(branch) => write!(f, ";branch={}", branch),
            Param::Received(branch) => write!(f, ";received={}", branch),
            Param::RPort => write!(f, "rport")
        }
    }
}

named!(parse_param<Param>, alt!(
    map!(pair!(char!(';'), tag!("rport")), |_| Param::RPort) |
    parse_named_param
));

named!(parse_named_param<Param>, do_parse!(
    tag!(";") >>
    key: take_while!(is_alphabetic) >>
    tag!("=") >>
    value: take_while!(is_alphanumeric) >>
    (Param::from_key(key, value)?)
));

pub fn parse_params(input: &[u8]) -> ParserResult<Vec<Param>> {
    let mut results = vec![];
    let mut data = input;

    while let Ok((remains, param)) = parse_param(&data) {
        results.push(param);
        data = remains;
    }
    Ok((data, results))
}
