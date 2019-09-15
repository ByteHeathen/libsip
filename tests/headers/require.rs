use libsip::headers::Header;
use libsip::headers::parse::parse_require_header;

#[test]
fn write() {
    let header = Header::Require("Softphone 1.0".into());
    assert_eq!("Require: Softphone 1.0".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Require("Softphone 1.0".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_require_header(b"Require: Softphone 1.0\n"));
}
