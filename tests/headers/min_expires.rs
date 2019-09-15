use libsip::headers::Header;
use libsip::headers::parse::parse_min_expires_header;

#[test]
fn write() {
    let header = Header::MinExpires(60);
    assert_eq!("Min-Expires: 60".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![' ' as u8];
    let header = Header::MinExpires(60);
    assert_eq!(Ok((remains.as_ref(), header)), parse_min_expires_header(b"Min-Expires: 60 "));
}
