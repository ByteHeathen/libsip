use libsip::Header;
use libsip::headers::parse::parse_in_reply_to_header;

#[test]
fn write() {
    let header = Header::InReplyTo("call@id.com".into());
    assert_eq!("In-Reply-To: call@id.com".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::InReplyTo("call@id.com".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_in_reply_to_header(b"In-Reply-To: call@id.com\n"));
}
