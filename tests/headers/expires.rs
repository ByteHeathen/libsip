use libsip::headers::Header;
use libsip::headers::parse::parse_expires_header;

#[test]
fn write() {
    let header = Header::Expires(60);
    assert_eq!("Expires: 60".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Expires(60);
    assert_eq!(Ok((remains.as_ref(), header)), parse_expires_header(b"Expires: 60\r\n"));
}
