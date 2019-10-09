use libsip::*;
use libsip::headers::parse::parse_to_header;

#[test]
fn write() {
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::To(Some("Guy".into()), uri);
    assert_eq!("To: Guy <sip:guy@example.com>".to_string(), format!("{}", header));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::To(Some("".into()), uri);
    assert_eq!("To: "" <sip:guy@example.com>".to_string(), format!("{}", header));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::To(Some("Guy With Face".into()), uri);
    assert_eq!("To: \"Guy With Face\" <sip:guy@example.com>".to_string(), format!("{}", header));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::To(None, uri);
    assert_eq!("To: <sip:guy@example.com>".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::To(Some("Guy".into()), uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_to_header(b"To: Guy <sip:guy@example.com>"));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::To(Some("Guy with face".into()), uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_to_header(b"To: \"Guy with face\" <sip:guy@example.com>"));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::To(None, uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_to_header(b"To: <sip:guy@example.com>"));
}
