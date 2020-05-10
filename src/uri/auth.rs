use crate::parse::slice_to_string;
use serde::{Deserialize, Serialize};

use nom::{
    IResult,
    error::ParseError,
    character::complete::char,
    character::is_alphanumeric,
    bytes::complete::take_while,
    combinator::{ map_res, opt}
};

use std::fmt;

/// URI Credentials
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UriAuth {
    username: String,
    password: Option<String>,
}

impl UriAuth {
    /// Create new UriAuth from `username`.
    pub fn new<S: Into<String>>(username: S) -> UriAuth {
        UriAuth {
            username: username.into(),
            password: None,
        }
    }

    /// Set the uri password.
    pub fn password<S: Into<String>>(mut self, p: S) -> UriAuth {
        self.password = Some(p.into());
        self
    }
}

impl fmt::Display for UriAuth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(pass) = &self.password {
            write!(f, "{}:{}", &self.username, pass)?;
        } else {
            write!(f, "{}", &self.username)?;
        }
        Ok(())
    }
}

/// Parse the username/password of a uri.
pub fn parse_uriauth<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], UriAuth, E> {
    let (input, username) = map_res(take_while(is_alphanumeric), slice_to_string::<E>)(input)?;
    let (input, password) = opt(parse_password::<E>)(input)?;
    let (input, _) = char('@')(input)?;
    Ok((input, UriAuth { username, password }))
 }

/// Currently this will only accept alphanumeric characters.
pub fn parse_password<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], String, E> {
    let (input, _) = char(':')(input)?;
    Ok(map_res(take_while(is_alphanumeric), slice_to_string::<E>)(input)?)
}
