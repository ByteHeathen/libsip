use libsip::*;
use libsip::uri::UriAuth;

#[test]
fn read_message() {
    let remains = vec![];
    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain);
    let req = SipMessage::Request {
        method: Method::Register,
        version: Version::default(),
        uri,
        headers: vec![],
        body: vec![]
    };
    assert_eq!(Ok((remains.as_ref(), req)), parse_message(b"REGISTER sip:example.com SIP/2.0\r\n\r\n"));
}

#[test]
fn read_complex() {
    let remains = vec![];
    let domain = Domain::Domain("example.com".into(), None);
    let uri = Uri::sip(domain).auth(UriAuth::new("user"));
    let req = SipMessage::Request {
        method: Method::Register,
        version: Version::default(),
        uri,
        headers: vec![
           Header::Expires(10),
           Header::ContentLength(5)
        ],
        body: vec!['6' as u8 ; 5]
    };
    assert_eq!(Ok((remains.as_ref(), req)), parse_message(b"REGISTER sip:user@example.com SIP/2.0\r\nExpires: 10\r\nContent-Length: 5\r\n\r\n66666"));
}
