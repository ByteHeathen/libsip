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
    error::ParseError,
    error::ErrorKind
};

use std::{
    io::{
        Error as IoError,
        ErrorKind as IoErrorKind
    },
    net::Ipv4Addr,
};

/// Parse input as a string using `String::from_utf8`.
pub fn slice_to_string<'a, E: ParseError<&'a [u8]>>(slice: &'a [u8]) -> Result<String, E> {
    if slice.is_empty() {
        Err(E::from_error_kind(slice, ErrorKind::Eof))
    } else {
        Ok(String::from_utf8(Vec::from(slice))
            .map_err(|_| E::from_error_kind(slice, ErrorKind::IsNot))?)
    }
}

pub fn slice_to_string_nullable(slice: &[u8]) -> Result<String, IoError> {
    Ok(String::from_utf8(Vec::from(slice))
        .map_err(|_| IoError::new(IoErrorKind::InvalidInput, "Failed to parse utf8 string"))?)
}

/// Parse unsigned 16 bit integer using `Parse::parse`.
pub fn parse_u16<'a, E: ParseError<&'a [u8]>>(slice: &'a [u8]) -> Result<u16, E> {
    Ok(::std::str::from_utf8(slice)
        .map_err(|_| {
            E::from_error_kind(slice, ErrorKind::IsNot)
        })?
        .parse()
        .map_err(|_| E::from_error_kind(slice, ErrorKind::IsNot))?)
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
pub fn parse_byte_vec<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Vec<u8>, E> {
    Ok((&input[input.len()..], input.to_vec()))
}

pub fn parse_ip_address<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], Ipv4Addr, E> {
  let (input, byte1) = map_res(take_while(is_digit), parse_u8)(input)?;
  let (input, _) = parse_char('.')(input)?;
  let (input, byte2) = map_res(take_while(is_digit), parse_u8)(input)?;
  let (input, _) = parse_char('.')(input)?;
  let (input, byte3) = map_res(take_while(is_digit), parse_u8)(input)?;
  let (input, _) = parse_char('.')(input)?;
  let (input, byte4) = map_res(take_while(is_digit), parse_u8)(input)?;
  Ok((input, Ipv4Addr::new(byte1, byte2, byte3, byte4)))
}

pub fn parse_string<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], String, E> {
    map_res(take_while(is_alphanumeric), slice_to_string::<E>)(input)
}

pub fn parse_possibly_quoted_string<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], String, E> {
    alt::<_, _, E, _>((
        parse_string::<E>,
        parse_quoted_string::<E>
    ))(input)
}

pub fn parse_quoted_string<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], String, E> {
    let (input, _) = parse_char('\"')(input)?;
    let (input, out) = map_res(take_until("\""), slice_to_string_nullable)(input)?;
    let (input, _) = parse_char('\"')(input)?;
    Ok((input, out))
}