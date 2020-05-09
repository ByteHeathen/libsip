use nom::character::*;

use nom::{
    IResult,
    branch::alt
};

use std::fmt;

use crate::{
    core::{code::error_code_to_str, method::parse_method, version::parse_version},
    headers::parse_header,
    parse::{parse_byte_vec, parse_u32, slice_to_string, *},
    uri::parse_uri,
    *,
};

/// Sip Protocol Message.
#[derive(Debug, PartialEq, Clone)]
pub enum SipMessage {
    Request {
        method: Method,
        uri: Uri,
        version: Version,
        headers: Headers,
        body: Vec<u8>,
    },
    Response {
        code: u32,
        version: Version,
        headers: Headers,
        body: Vec<u8>,
    },
}

impl SipMessage {
    /// Determine if this is a SIP request.
    pub fn is_request(&self) -> bool {
        if let SipMessage::Request { .. } = self {
            true
        } else {
            false
        }
    }

    /// Determine if this is a SIP response.
    pub fn is_response(&self) -> bool {
        if let SipMessage::Response { .. } = self {
            true
        } else {
            false
        }
    }

    /// Retreive the SIP response's status code.
    /// Returns None for requests.
    pub fn status_code(&self) -> Option<u32> {
        if let SipMessage::Response { code, .. } = self {
            Some(*code)
        } else {
            None
        }
    }

    /// Retreive the body of this SIP Message.
    pub fn body(&self) -> &Vec<u8> {
        match self {
            SipMessage::Request { body, .. } => body,
            SipMessage::Response { body, .. } => body,
        }
    }

    /// Retreive a mutable reference to the SIP Messages body.
    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        match self {
            SipMessage::Request { body, .. } => body,
            SipMessage::Response { body, .. } => body,
        }
    }

    /// Retreive headers from the SIP message.
    pub fn headers(&self) -> &Headers {
        match self {
            SipMessage::Request { headers, .. } => headers,
            SipMessage::Response { headers, .. } => headers,
        }
    }

    /// Retreive a mutable reference to the SIP Message's header list.
    pub fn headers_mut(&mut self) -> &mut Headers {
        match self {
            SipMessage::Request { headers, .. } => headers,
            SipMessage::Response { headers, .. } => headers,
        }
    }
}

impl fmt::Display for SipMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SipMessage::Request {
                method,
                uri,
                version,
                headers,
                body,
            } => {
                writeln!(f, "{} {} {}\r", method, uri, version)?;
                display_headers_and_body(f, headers, body)
            },
            SipMessage::Response {
                code,
                version,
                headers,
                body,
            } => {
                if let Some(desc) = error_code_to_str(*code) {
                    writeln!(f, "{} {} {}\r", version, code, desc)?;
                } else {
                    writeln!(f, "{} {}\r", version, code)?;
                }
                display_headers_and_body(f, headers, body)
            },
        }
    }
}

pub fn display_headers_and_body(
    f: &mut fmt::Formatter,
    headers: &Headers,
    body: &[u8],
) -> Result<(), fmt::Error> {
    for header in headers.iter() {
        writeln!(f, "{}\r", header)?;
    }
    writeln!(f, "\r")?;
    f.write_str(&String::from_utf8_lossy(&body))?;
    Ok(())
}

pub fn parse_headers(input: &[u8]) -> ParserResult<Headers> {
    let mut headers = Headers(vec![]);
    let mut input = input;
    while let Ok((data, value)) = parse_header(input) {
        headers.push(value);
        input = data;
    }
    Ok((input, headers))
}

use nom::{
    combinator::{map_res, opt},
    bytes::complete::{take_while, tag},
    character::complete::char
};

pub fn parse_response(input: &[u8]) -> IResult<&[u8], SipMessage> {
    let (input, version) = parse_version(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, code) = map_res(take_while(is_digit), parse_u32)(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, _) = opt(map_res(take_while(|item| is_alphabetic(item) || is_space(item)), slice_to_string))(input)?;
    let (input, _) = opt(char(' '))(input)?;
    let (input, _) = tag("\r\n")(input)?;
    let (input, headers) = parse_headers(input)?;
    let (input, _) = tag("\r\n")(input)?;
    let (input, body) = parse_byte_vec(input)?;
    Ok((input, SipMessage::Response { code, version, headers, body }))
}

pub fn parse_request(input: &[u8]) -> IResult<&[u8], SipMessage> {
    let (input, method) = parse_method(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, uri) = parse_uri(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, version) = parse_version(input)?;
    let (input, _) = opt(char(' '))(input)?;
    let (input, _) = tag("\r\n")(input)?;
    let (input, headers) = parse_headers(input)?;
    let (input, _) = tag("\r\n")(input)?;
    let (input, body) = parse_byte_vec(input)?;
    Ok((input, SipMessage::Request { method, uri, version, headers, body }))
}

pub fn parse_message(input: &[u8]) -> IResult<&[u8], SipMessage> {
    alt((
        parse_request,
        parse_response
    ))(input)
}