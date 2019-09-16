use libsip::core::Version;
use libsip::core::SipMessage;
use libsip::headers::Header;
use libsip::core::message::parse_response;

#[test]
fn write_simple() {
    let req = SipMessage::Response {
        code: 200,
        version: Version::default(),
        headers: vec![],
        body: vec![]
    };
    assert_eq!("SIP/2.0 200 OK\r\n\r\n".to_string(), format!("{}", req));
}

#[test]
fn write_complex() {
    let req = SipMessage::Response {
        code: 180,
        version: Version::default(),
        headers: vec![
            Header::Expires(10),
            Header::ContentLength(5)
        ],
        body: vec!['5' as u8; 5]
    };
    assert_eq!("SIP/2.0 180 Ringing\r\nExpires: 10\r\nContent-Length: 5\r\n\r\n55555".to_string(), format!("{}", req));
}

#[test]
fn read_simple() {
    let remains = vec![];
    let req = SipMessage::Response {
        code: 200,
        version: Version::default(),
        headers: vec![],
        body: vec![]
    };
    assert_eq!(Ok((remains.as_ref(), req)), parse_response(b"SIP/2.0 200 OK\r\n"));
}

#[test]
fn read_complex() {
    let remains = vec![];
    let req = SipMessage::Response {
        code: 180,
        version: Version::default(),
        headers: vec![
            Header::Expires(10),
            Header::ContentLength(5)
        ],
        body: vec!['5' as u8; 5]
    };
    assert_eq!(Ok((remains.as_ref(), req)), parse_response(b"SIP/2.0 180 Ringing\r\nExpires: 10\r\nContent-Length: 5\r\n55555"));
}
