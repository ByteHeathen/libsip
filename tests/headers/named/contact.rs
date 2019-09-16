use libsip::uri::*;
use libsip::uri::UriAuth;
use libsip::headers::parse::parse_contact_header;

#[test]
fn write() {
    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::Contact(Some("Guy".into()), uri);
    assert_eq!("Contact: Guy <sip:guy@example.com>".to_string(), format!("{}", header));

    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::Contact(Some("Guy With Face".into()), uri);
    assert_eq!("Contact: \"Guy With Face\" <sip:guy@example.com>".to_string(), format!("{}", header));

    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::Contact(None, uri);
    assert_eq!("Contact: <sip:guy@example.com>".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::Contact(Some("Guy".into()), uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_contact_header(b"Contact: Guy <sip:guy@example.com>"));

    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::Contact(Some("Guy with face".into()), uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_contact_header(b"Contact: \"Guy with face\" <sip:guy@example.com>"));

    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("guy"));
    let header = Header::Contact(None, uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_contact_header(b"Contact: <sip:guy@example.com>"));
}
