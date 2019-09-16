use libsip::Header;
use libsip::headers::parse::parse_timestamp_header;

#[test]
fn write() {
    let header = Header::Timestamp(60);
    assert_eq!("Timestamp: 60".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Timestamp(60);
    assert_eq!(Ok((remains.as_ref(), header)), parse_timestamp_header(b"Timestamp: 60\r\n"));
}
