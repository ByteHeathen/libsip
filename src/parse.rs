use nom::{character::*, error::ErrorKind};

use std::{
    io::{Error as IoError, ErrorKind as IoErrorKind, ErrorKind::InvalidInput},
    net::Ipv4Addr,
};

pub type ParserResult<'a, T> = Result<(&'a [u8], T), nom::Err<(&'a [u8], ErrorKind)>>;

/// Parse input as a string using `String::from_utf8`.
pub fn slice_to_string(slice: &[u8]) -> Result<String, IoError> {
    if slice.is_empty() {
        Err(IoError::new(InvalidInput, "slice has length 0"))
    } else {
        Ok(String::from_utf8(Vec::from(slice))
            .map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Failed to parse utf8 string"))?)
    }
}

pub fn slice_to_string_nullable(slice: &[u8]) -> Result<String, IoError> {
    Ok(String::from_utf8(Vec::from(slice))
        .map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Failed to parse utf8 string"))?)
}

/// Parse unsigned 16 bit integer using `Parse::parse`.
pub fn parse_u16(slice: &[u8]) -> Result<u16, IoError> {
    Ok(::std::str::from_utf8(slice)
        .map_err(|_| {
            IoError::new(
                IoErrorKind::InvalidInput,
                "Failed to parse utf8 u16 integer",
            )
        })?
        .parse()
        .map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Failed to parse u16 integer"))?)
}

/// Parse unsigned 8 bit integer using `Parse::parse`.
pub fn parse_u8(slice: &[u8]) -> Result<u8, IoError> {
    Ok(::std::str::from_utf8(slice)
        .map_err(|_| {
            IoError::new(
                IoErrorKind::InvalidInput,
                "Failed to parse utf8 u16 integer",
            )
        })?
        .parse()
        .map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Failed to parse u8 integer"))?)
}

/// Parse unsigned 32 bit integer using `Parse::parse`.
pub fn parse_u32(slice: &[u8]) -> Result<u32, IoError> {
    Ok(::std::str::from_utf8(slice)
        .map_err(|_| {
            IoError::new(
                IoErrorKind::InvalidInput,
                "Failed to parse utf8 u32 integer",
            )
        })?
        .parse()
        .map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Failed to parse u32 integer"))?)
}

/// Parse input as an f32 using `Parse::parse`.
pub fn parse_f32(slice: &[u8]) -> Result<f32, IoError> {
    Ok(::std::str::from_utf8(slice)
        .map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Failed to parse utf8 f32"))?
        .parse()
        .map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Failed to parse f32"))?)
}

/// Parse Input as a vector of bytes.
pub fn parse_byte_vec(input: &[u8]) -> ParserResult<Vec<u8>> {
    Ok((&input[input.len()..], input.to_vec()))
}

named!(pub parse_ip_address<Ipv4Addr>, do_parse!(
    byte1: map_res!(take_while!(is_digit), parse_u8) >>
    char!('.') >>
    byte2: map_res!(take_while!(is_digit), parse_u8) >>
    char!('.') >>
    byte3: map_res!(take_while!(is_digit), parse_u8) >>
    char!('.') >>
    byte4: map_res!(take_while!(is_digit), parse_u8) >>
    (Ipv4Addr::new(byte1, byte2,  byte3, byte4))
));

named!(pub parse_string<String>, map_res!(
    take_while!(is_alphanumeric), slice_to_string
));

named!(pub parse_possibly_quoted_string<String>, alt!(
	parse_string | parse_quoted_string
));

named!(pub parse_quoted_string<String>, do_parse!(
    char!('\"') >>
    out: map_res!(take_until!("\""), slice_to_string_nullable) >>
    char!('\"') >>
    (out)
));
