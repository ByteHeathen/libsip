use libsip::*;

#[test]
fn read_message() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com"));
    let req = SipMessage::Request {
        method: Method::Register,
        version: Version::default(),
        uri,
        headers: Headers::new(),
        body: vec![]
    };
    assert_eq!(Ok((remains.as_ref(), req)), parse_message(b"REGISTER sip:example.com SIP/2.0\r\n\r\n"));
}

#[test]
fn read_complex() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("user"));
    let req = SipMessage::Request {
        method: Method::Register,
        version: Version::default(),
        uri,
        headers: Headers(vec![
           Header::Expires(10),
           Header::ContentLength(5)
        ]),
        body: vec![b'6'; 5]
    };
    assert_eq!(Ok((remains.as_ref(), req)), parse_message(b"REGISTER sip:user@example.com SIP/2.0\r\nExpires: 10\r\nContent-Length: 5\r\n\r\n66666"));
}
