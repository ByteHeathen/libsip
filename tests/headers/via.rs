use libsip::headers::Header;
use libsip::headers::parse::parse_via_header;

#[test]
fn write() {
    let header = Header::Via("Softphone 1.0".into());
    assert_eq!("Via: Softphone 1.0".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Via("Softphone 1.0".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_via_header(b"Via: Softphone 1.0\n"));
}
