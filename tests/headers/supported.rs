use libsip::headers::Header;
use libsip::headers::parse::parse_supported_header;

#[test]
fn write() {
    let header = Header::Supported("Softphone 1.0".into());
    assert_eq!("Supported: Softphone 1.0".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Supported("Softphone 1.0".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_supported_header(b"Supported: Softphone 1.0\n"));
}
