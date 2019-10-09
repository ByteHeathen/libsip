use nom::character::is_digit;
use nom::character::is_alphanumeric;
use serde::{ Deserialize, Serialize };

use std::fmt;
use std::net::Ipv4Addr;

use crate::parse::parse_u16;
use crate::parse::slice_to_string;
use crate::parse::parse_ip_address;

/// Domain address for a URI.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Domain {
    Ipv4(Ipv4Addr, Option<u16>),
    Domain(String, Option<u16>)
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Domain::Ipv4(addr, port) => {
                if let Some(port) = port {
                    write!(f, "{}:{}", addr, port)
                } else {
                    write!(f, "{}", addr)
                }
            }
            Domain::Domain(domain, port) => {
                if let Some(port) = port {
                    write!(f, "{}:{}", domain, port)
                } else {
                    write!(f, "{}", domain)
                }
            }
        }
    }
}

named!(pub parse_domain<Domain>, do_parse!(
    out: alt!(parse_ip_domain | parse_domain_domain) >>
    (out)
));

named!(pub parse_ip_domain<Domain>, do_parse!(
    addr: parse_ip_address >>
    opt!(char!(':')) >>
    port: opt!(map_res!(take_while!(is_digit), parse_u16)) >>
    (Domain::Ipv4(addr, port))
));

named!(pub parse_domain_domain<Domain>, do_parse!(
    domain: map_res!(take_while!(|item| is_alphanumeric(item) || item == b'.'), slice_to_string) >>
    opt!(char!(':')) >>
    port: opt!(map_res!(take_while!(is_digit), parse_u16)) >>
    (Domain::Domain(domain, port))
));
