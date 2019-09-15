use libsip::headers::Header;
use libsip::headers::parse::parse_to_header;
use libsip::uri::Uri;
use libsip::uri::Domain;
use libsip::uri::UriAuth;

#[test]
fn write() {
    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::To(Some("Guy".into()), uri);
    assert_eq!("To: Guy <sip:guy@example.com>".to_string(), format!("{}", header));

    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::To(Some("Guy With Face".into()), uri);
    assert_eq!("To: \"Guy With Face\" <sip:guy@example.com>".to_string(), format!("{}", header));

    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::To(None, uri);
    assert_eq!("To: <sip:guy@example.com>".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::To(Some("Guy".into()), uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_to_header(b"To: Guy <sip:guy@example.com>"));

    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::To(Some("Guy with face".into()), uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_to_header(b"To: \"Guy with face\" <sip:guy@example.com>"));

    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::To(None, uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_to_header(b"To: <sip:guy@example.com>"));
}
