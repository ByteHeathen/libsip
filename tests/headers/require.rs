use libsip::{headers::parse::parse_require_header, Header};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::Require("Softphone 1.0".into());
    assert_eq!("Require: Softphone 1.0".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Require("Softphone 1.0".into());
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_require_header::<VerboseError<&[u8]>>(b"Require: Softphone 1.0\r\n")
    );
}
