use libsip::*;
use libsip::headers::parse::parse_from_header;

#[test]
fn write() {
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(named_header!(uri, "Guy"));
    assert_eq!("From: Guy <sip:guy@example.com>".to_string(), format!("{}", header));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(named_header!(uri, "Guy With Face"));
    assert_eq!("From: \"Guy With Face\" <sip:guy@example.com>".to_string(), format!("{}", header));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(named_header!(uri));
    assert_eq!("From: sip:guy@example.com".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(named_header!(uri, "Guy"));
    assert_eq!(Ok((remains.as_ref(), header)), parse_from_header(b"From: Guy <sip:guy@example.com>\r\n"));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(named_header!(uri, "Guy with face"));
    assert_eq!(Ok((remains.as_ref(), header)), parse_from_header(b"From: \"Guy with face\" <sip:guy@example.com>\r\n"));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(named_header!(uri));
    assert_eq!(Ok((remains.as_ref(), header)), parse_from_header(b"From: <sip:guy@example.com>\r\n"));

    let uri = Uri::sip(ip_domain!(127, 0, 0, 1)).auth(uri_auth!("unknown"));
    let header = Header::From(named_header!(uri));
    assert_eq!(Ok((remains.as_ref(), header)), parse_from_header(b"From: sip:unknown@127.0.0.1\r\n"));

}
