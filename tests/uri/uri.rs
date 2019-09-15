use std::net::Ipv4Addr;

use libsip::uri::*;
use libsip::core::Transport;

#[test]
fn read_uri() {
    let expected_remains = vec![' ' as u8];
    let domain = Domain::Domain("hostname".into(), None);
    assert_eq!(Ok((expected_remains.as_ref(), Uri::sip(domain))), parse_uri(b"sip:hostname "));

    let expected_remains = vec![' ' as u8];
    let domain = Domain::Ipv4(Ipv4Addr::new(10,1,10,1), None);
    assert_eq!(Ok((expected_remains.as_ref(), Uri::sip(domain))), parse_uri(b"sip:10.1.10.1 "));

    let expected_remains = vec![' ' as u8];
    let domain = Domain::Domain("hostname.com".into(), None);
    let expected = Uri::sip(domain).auth(UriAuth::new("username"));
    assert_eq!(Ok((expected_remains.as_ref(), expected)), parse_uri(b"sip:username@hostname.com "));

    let expected_remains = vec![' ' as u8];
    let domain = Domain::Domain("hostname.com".into(), None);
    let expected = Uri::sip(domain).auth(UriAuth::new("username").password("password"));
    assert_eq!(Ok((expected_remains.as_ref(), expected)), parse_uri(b"sip:username:password@hostname.com "));

    let expected_remains = vec![' ' as u8];
    let domain = Domain::Domain("hostname.com".into(), Some(8080));
    let expected = Uri::sip(domain).auth(UriAuth::new("username").password("password"));
    assert_eq!(Ok((expected_remains.as_ref(), expected)), parse_uri(b"sip:username:password@hostname.com:8080 "));

    let expected_remains = vec![' ' as u8];
    let domain = Domain::Domain("hostname.com".into(), Some(8080));
    let expected = Uri::sip(domain)
        .parameter(Param::Transport(Transport::Udp))
        .auth(UriAuth::new("username").password("password"));
    assert_eq!(Ok((expected_remains.as_ref(), expected)), parse_uri(b"sip:username:password@hostname.com:8080;transport=UDP "));
}

#[test]
fn write_uri() {
    let uri = Uri::sip(Domain::Domain("hostname".into(), None));
    assert_eq!("sip:hostname".to_string(), format!("{}", uri));

    let uri = Uri::sip(Domain::Ipv4(Ipv4Addr::new(10,1,10,1), None));
    assert_eq!("sip:10.1.10.1".to_string(), format!("{}", uri));

    let uri = Uri::sip(Domain::Domain("hostname.com".into(), None))
                        .auth(UriAuth::new("username"));
    assert_eq!("sip:username@hostname.com".to_string(), format!("{}", uri));

    let uri = Uri::sip(Domain::Domain("hostname.com".into(), None))
                .auth(UriAuth::new("username").password("password"));
    assert_eq!("sip:username:password@hostname.com".to_string(), format!("{}", uri));

    let domain = Domain::Domain("hostname.com".into(), Some(8080));
    let uri = Uri::sip(domain).auth(UriAuth::new("username").password("password"));
    assert_eq!("sip:username:password@hostname.com:8080".to_string(), format!("{}", uri));

    let uri = Uri::sip(Domain::Domain("hostname.com".into(), Some(8080)))
        .parameter(Param::Transport(Transport::Udp))
        .auth(UriAuth::new("username").password("password"));
    assert_eq!("sip:username:password@hostname.com:8080;transport=UDP", format!("{}", uri));
}
