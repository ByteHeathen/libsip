use libsip::{headers::parse::parse_unsupported_header, Header};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::Unsupported("Softphone 1.0".into());
    assert_eq!(
        "Unsupported: Softphone 1.0".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Unsupported("Softphone 1.0".into());
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_unsupported_header::<VerboseError<&[u8]>>(b"Unsupported: Softphone 1.0\r\n")
    );
}
