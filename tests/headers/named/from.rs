use libsip::*;
use libsip::headers::parse::parse_from_header;

#[test]
fn write() {
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(Some("Guy".into()), uri);
    assert_eq!("From: Guy <sip:guy@example.com>".to_string(), format!("{}", header));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(Some("Guy With Face".into()), uri);
    assert_eq!("From: \"Guy With Face\" <sip:guy@example.com>".to_string(), format!("{}", header));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(None, uri);
    assert_eq!("From: <sip:guy@example.com>".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(Some("Guy".into()), uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_from_header(b"From: Guy <sip:guy@example.com>"));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(Some("Guy with face".into()), uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_from_header(b"From: \"Guy with face\" <sip:guy@example.com>"));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(None, uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_from_header(b"From: <sip:guy@example.com>"));
}
