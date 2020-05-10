use libsip::{headers::parse::parse_priority_header, Header};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::Priority("Softphone 1.0".into());
    assert_eq!("Priority: Softphone 1.0".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Priority("Softphone 1.0".into());
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_priority_header::<VerboseError<&[u8]>>(b"Priority: Softphone 1.0\r\n")
    );
}
