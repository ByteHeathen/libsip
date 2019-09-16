use libsip::headers::parse::parse_content_length_header;
use libsip::headers::Header;

#[test]
fn write() {
    let header = Header::ContentLength(70);
    assert_eq!("Content-Length: 70".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::ContentLength(60);
    assert_eq!(Ok((remains.as_ref(), header)), parse_content_length_header(b"Content-Length: 60\r\n"));
}
