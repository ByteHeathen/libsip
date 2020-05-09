use nom::{
    IResult,
    branch::alt,
    character::{
        *,
        complete::char as parse_char
    },
    bytes::{
        complete::{
            take_while,
            take_until
        }
    },
    combinator::map_res,
    error::ErrorKind
};

use std::{
    io::{
        Error as IoError,
        ErrorKind as IoErrorKind,
        ErrorKind::InvalidInput
    },
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

pub fn parse_ip_address(input: &[u8]) -> IResult<&[u8], Ipv4Addr> {
  let (input, byte1) = map_res(take_while(is_digit), parse_u8)(input)?;
  let (input, _) = parse_char('.')(input)?;
  let (input, byte2) = map_res(take_while(is_digit), parse_u8)(input)?;
  let (input, _) = parse_char('.')(input)?;
  let (input, byte3) = map_res(take_while(is_digit), parse_u8)(input)?;
  let (input, _) = parse_char('.')(input)?;
  let (input, byte4) = map_res(take_while(is_digit), parse_u8)(input)?;
  Ok((input, Ipv4Addr::new(byte1, byte2, byte3, byte4)))
}

pub fn parse_string(input: &[u8]) -> IResult<&[u8], String> {
    map_res(take_while(is_alphanumeric), slice_to_string)(input)
}

pub fn parse_possibly_quoted_string(input: &[u8]) -> IResult<&[u8], String> {
    alt((
        parse_string,
        parse_quoted_string
    ))(input)
}

pub fn parse_quoted_string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_char('\"')(input)?;
    let (input, out) = map_res(take_until("\""), slice_to_string_nullable)(input)?;
    let (input, _) = parse_char('\"')(input)?;
    Ok((input, out))
}