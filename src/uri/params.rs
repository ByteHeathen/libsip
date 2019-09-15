use nom::character::is_alphabetic;
use nom::Err;
use nom::error::ErrorKind;

use std::fmt;

use crate::core::Transport;
use crate::core::parse_transport;

#[derive(Debug, PartialEq, Clone)]
pub enum Param {
    Transport(Transport)
}

impl Param {

    pub fn from_key<'a>(key: &'a [u8], value: &'a [u8]) -> Result<Param, nom::Err<(&'a [u8], ErrorKind)>> {
        match key.as_ref() {
            b"transport" => Ok(Param::Transport(parse_transport(&value)?.1)),
            _method => Err(Err::Failure((key, ErrorKind::MapRes)))
        }
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Param::Transport(trans) => write!(f, ";transport={}", trans)
        }
    }
}

named!(parse_param<Param>, do_parse!(
    tag!(";") >>
    key: take_while!(is_alphabetic) >>
    tag!("=") >>
    value: take_while!(is_alphabetic) >>
    (Param::from_key(key, value)?)
));

pub fn parse_params(input: &[u8]) -> Result<(&[u8], Vec<Param>), nom::Err<(&[u8], nom::error::ErrorKind)>> {
    let mut results = vec![];
    let mut data = input;
    loop {
        if let Ok((remains, param)) = parse_param(&data) {
            results.push(param);
            data = remains;
        } else {
            break;
        }
    }
    Ok((data, results))
}
