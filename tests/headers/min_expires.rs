use libsip::{headers::parse::parse_min_expires_header, Header};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::MinExpires(60);
    assert_eq!("Min-Expires: 60".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::MinExpires(60);
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_min_expires_header::<VerboseError<&[u8]>>(b"Min-Expires: 60\r\n")
    );
}
