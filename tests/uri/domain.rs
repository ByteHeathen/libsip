use libsip::*;
use libsip::Domain;
use libsip::uri::parse_domain;

use std::net::Ipv4Addr;

#[test]
fn read_domain() {
    let remains = vec![' ' as u8];
    let domain = domain!("example.com");
    assert_eq!(Ok((remains.as_ref(), domain)), parse_domain(b"example.com "));

    let remains = vec![' ' as u8];
    let domain = domain!("example.com", 8080);
    assert_eq!(Ok((remains.as_ref(), domain)), parse_domain(b"example.com:8080 "));
}

#[test]
fn read_ip_address() {
    let remains = vec![' ' as u8];
    let domain = ip_domain!(10, 1, 10, 1);
    assert_eq!(Ok((remains.as_ref(), domain)), parse_domain(b"10.1.10.1 "));

    let remains = vec![' ' as u8];
    let domain = ip_domain!(10, 1, 10, 1, 8080);
    assert_eq!(Ok((remains.as_ref(), domain)), parse_domain(b"10.1.10.1:8080 "));
}

#[test]
fn write_domain() {
    let domain = domain!("example.com");
    assert_eq!("example.com".to_string(), format!("{}", domain));

    let domain = domain!("example.com", 8080);
    assert_eq!("example.com:8080".to_string(), format!("{}", domain));
}

#[test]
fn write_ip_address() {
    let domain = Domain::Ipv4(Ipv4Addr::new(10, 1, 10, 1), None);
    assert_eq!("10.1.10.1".to_string(), format!("{}", domain));

    let domain = Domain::Ipv4(Ipv4Addr::new(10, 1, 10, 1), Some(8080));
    assert_eq!("10.1.10.1:8080".to_string(), format!("{}", domain));
}
