use libsip::*;
use libsip::headers::parse::parse_reply_to_header;

#[test]
fn write() {
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::ReplyTo(Some("Guy".into()), uri);
    assert_eq!("Reply-To: Guy <sip:guy@example.com>".to_string(), format!("{}", header));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::ReplyTo(Some("Guy With Face".into()), uri);
    assert_eq!("Reply-To: \"Guy With Face\" <sip:guy@example.com>".to_string(), format!("{}", header));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::ReplyTo(None, uri);
    assert_eq!("Reply-To: <sip:guy@example.com>".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::ReplyTo(Some("Guy".into()), uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_reply_to_header(b"Reply-To: Guy <sip:guy@example.com>"));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::ReplyTo(Some("Guy with face".into()), uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_reply_to_header(b"Reply-To: \"Guy with face\" <sip:guy@example.com>"));
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::ReplyTo(None, uri);
    assert_eq!(Ok((remains.as_ref(), header)), parse_reply_to_header(b"Reply-To: <sip:guy@example.com>"));
}
