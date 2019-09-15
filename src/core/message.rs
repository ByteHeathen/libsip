use nom::character::is_digit;
use nom::character::is_alphabetic;
use nom::error::ErrorKind;

use std::fmt;

use crate::core::code::error_code_to_str;
use crate::core::Version;
use crate::core::version::parse_version;
use crate::core::Method;
use crate::core::method::parse_method;
use crate::uri::Uri;
use crate::uri::parse_uri;
use crate::headers::Header;
use crate::headers::parse_header;
use crate::parse::parse_u32;
use crate::parse::parse_byte_vec;
use crate::parse::slice_to_string;

#[derive(Debug, PartialEq, Clone)]
pub enum SipMessage {
    Request {
        method: Method,
        uri: Uri,
        version: Version,
        headers: Vec<Header>,
        body: Vec<u8>
    },
    Response {
        code: u32,
        version: Version,
        headers: Vec<Header>,
        body: Vec<u8>
    }
}

impl SipMessage {

    pub fn is_request(&self) -> bool {
        if let SipMessage::Request { .. } = self {
            true
        } else {
            false
        }
    }

    pub fn body(&self) -> &Vec<u8> {
        match self {
            SipMessage::Request { body, .. } => body,
            SipMessage::Response { body, .. } => body
        }
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        match self {
            SipMessage::Request { body, .. } => body,
            SipMessage::Response { body, .. } => body
        }
    }

    pub fn headers(&self) -> &Vec<Header> {
        match self {
            SipMessage::Request { headers, .. } => headers,
            SipMessage::Response { headers, .. } => headers
        }
    }

    pub fn headers_mut(&mut self) -> &mut Vec<Header> {
        match self {
            SipMessage::Request { headers, .. } => headers,
            SipMessage::Response { headers, .. } => headers
        }
    }
}

impl fmt::Display for SipMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SipMessage::Request { method, uri, version, headers, body } => {
                write!(f, "{} {} {}\r\n", method, uri, version)?;
                display_headers_and_body(f, headers, body)
            },
            SipMessage::Response { code, version, headers, body } => {
                if let Some(desc) = error_code_to_str(code.clone() as u16) {
                    write!(f, "{} {} {}\r\n", version, code, desc)?;
                } else {
                    write!(f, "{} {}\r\n", version, code)?;
                }
                display_headers_and_body(f, headers, body)
            }
        }
    }
}

pub fn display_headers_and_body(f: &mut fmt::Formatter, headers: &Vec<Header>, body: &Vec<u8>) -> Result<(), fmt::Error> {
    for header in headers.iter() {
        write!(f, "{}\r\n", header)?;
    }
    f.write_str(&String::from_utf8_lossy(&body))?;
    Ok(())
}

pub fn parse_headers(input: &[u8]) -> Result<(&[u8], Vec<Header>), nom::Err<(&[u8], ErrorKind)>> {
    let mut headers = vec![];
    let mut input = input;
    loop {
        match parse_header(input) {
            Ok((data, value)) => {
                headers.push(value);
                if let Ok((remains, _)) = parse_header_ending(data) {
                    input = remains;
                } else {
                    break;
                }
            },
            _ => break
        }
    }
    Ok((input, headers))
}

named!(parse_header_ending<()>, do_parse!(
    tag!("\r\n") >>
    (())
));

named!(pub parse_response<SipMessage>, do_parse!(
    version: parse_version >>
    char!(' ') >>
    code: map_res!(take_while!(is_digit), parse_u32) >>
    char!(' ') >>
    opt!(map_res!(take_while!(is_alphabetic), slice_to_string)) >>
    opt!(char!(' ')) >>
    tag!("\r\n") >>
    headers: parse_headers >>
    body: parse_byte_vec >>
    (SipMessage::Response { code, version, headers, body })
));

named!(pub parse_request<SipMessage>, do_parse!(
    method: parse_method >>
    char!(' ') >>
    uri: parse_uri >>
    char!(' ') >>
    version: parse_version >>
    opt!(char!(' ')) >>
    tag!("\r\n") >>
    headers: parse_headers >>
    body: parse_byte_vec >>
    (SipMessage::Request { method, uri, version, headers, body })
));

named!(pub parse_message<SipMessage>, alt!(
    parse_request | parse_response
));
