use nom::character::is_digit;
use nom::error::ErrorKind;
use failure::Error;

use std::net::Ipv4Addr;
use std::io::Error as IoError;
use std::io::ErrorKind::InvalidInput;

pub fn slice_to_string(slice: &[u8]) -> Result<String, Error> {
	if slice.len() == 0 {
		Err(Error::from_boxed_compat(Box::new(IoError::new(InvalidInput, "slice has length 0"))))
	} else {
		Ok(String::from_utf8(Vec::from(slice))?)
	}
}

pub fn parse_u16(slice: &[u8]) -> Result<u16, Error> {
	Ok(::std::str::from_utf8(slice)?.parse()?)
}

pub fn parse_u8(slice: &[u8]) -> Result<u8, Error> {
	Ok(::std::str::from_utf8(slice)?.parse()?)
}

pub fn parse_u32(slice: &[u8]) -> Result<u32, Error> {
	Ok(::std::str::from_utf8(slice)?.parse()?)
}

pub fn parse_f32(slice: &[u8]) -> Result<f32, Error> {
	Ok(::std::str::from_utf8(slice)?.parse()?)
}

pub fn parse_byte_vec(input: &[u8]) -> Result<(&[u8], Vec<u8>), nom::Err<(&[u8], ErrorKind)>> {
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
