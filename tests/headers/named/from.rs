use libsip::headers::Header;
use libsip::headers::parse::parse_from_header;
use libsip::uri::Uri;
use libsip::uri::Domain;
use libsip::uri::UriAuth;

#[test]
fn write() {
    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::From(Some("Guy".into()), uri);
    assert_eq!("From: Guy <sip:guy@example.com>".to_string(), format!("{}", header));

    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::From(Some("Guy With Face".into()), uri);
    assert_eq!("From: \"Guy With Face\" <sip:guy@example.com>".to_string(), format!("{}", header));

    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::From(None, uri);
    assert_eq!("From: <sip:guy@example.com>".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::From(Some("Guy".into()), uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_from_header(b"From: Guy <sip:guy@example.com>"));

    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::From(Some("Guy with face".into()), uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_from_header(b"From: \"Guy with face\" <sip:guy@example.com>"));

    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::From(None, uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_from_header(b"From: <sip:guy@example.com>"));
}
