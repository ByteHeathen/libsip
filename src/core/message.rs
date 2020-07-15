use nom::{branch::alt, character::*, IResult};

use std::fmt;

use crate::{
    core::{code::error_code_to_str, method::parse_method, version::parse_version},
    headers::parse_header,
    parse::{is_reserved, is_unreserved, parse_byte_vec, parse_u32},
    uri::parse_uri,
    SipMessageExt, *,
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

    pub fn method(&self) -> Option<Method> {
        match self {
            Self::Request { method, .. } => Some(*method),
            Self::Response { .. } => self.cseq().map(|(_, method)| method),
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

/// Write the headers and body of a SIP message.
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

/// Parse SIP headers recursivily
pub fn parse_headers<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], Headers, E> {
    let mut headers = Headers(vec![]);
    let mut input = input;
    while let Ok((data, value)) = parse_header::<E>(input) {
        headers.push(value);
        input = data;
    }
    Ok((input, headers))
}

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::char,
    combinator::{map_res, opt},
    error::ParseError,
};

/// Parses Reason-Phrase ([RFC3261: Page 225](https://tools.ietf.org/html/rfc3261#page-225))
/// # Notes
/// escaped / UTF8-NONASCII / UTF8-CONT isn't supported yet
/// # Examples
/// ```
/// use libsip::core::message::parse_reason_phrase;
/// use nom::error::VerboseError;
/// assert_eq!(
///     parse_reason_phrase::<VerboseError<_>>(b"Not Found"),
///     Ok(("".as_bytes(), "Not Found".as_bytes()))
/// );
/// assert_eq!(
///     parse_reason_phrase::<VerboseError<_>>(b"Call/Transaction Does Not Exist"),
///     Ok(("".as_bytes(), "Call/Transaction Does Not Exist".as_bytes()))
/// );
/// ```
pub fn parse_reason_phrase<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], &'a [u8], E> {
    let pred = |c| is_reserved(c) || is_unreserved(c) || is_space(c);
    take_while(pred)(input)
}

/// Parse a SIP message assuming it is a SIP response.
pub fn parse_response<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], SipMessage, E> {
    let (input, version) = parse_version::<E>(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, code) = map_res(take_while(is_digit), parse_u32)(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, _) = opt(parse_reason_phrase)(input)?;
    let (input, _) = opt(char(' '))(input)?;
    let (input, _) = tag("\r\n")(input)?;
    let (input, headers) = parse_headers::<E>(input)?;
    let (input, _) = tag("\r\n")(input)?;
    let (input, body) = parse_byte_vec::<E>(input)?;
    Ok((
        input,
        SipMessage::Response {
            code,
            version,
            headers,
            body,
        },
    ))
}

/// Parse a SIP message assuming it is a SIP request.
pub fn parse_request<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], SipMessage, E> {
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
    Ok((
        input,
        SipMessage::Request {
            method,
            uri,
            version,
            headers,
            body,
        },
    ))
}

/// This is the main parsing function for libsip.
pub fn parse_message<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], SipMessage, E> {
    alt::<_, _, E, _>((parse_request::<E>, parse_response::<E>))(input)
}
