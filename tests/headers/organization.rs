use libsip::{headers::parse::parse_organization_header, Header};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::Organization("Softphone 1.0".into());
    assert_eq!(
        "Organization: Softphone 1.0".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Organization("Softphone 1.0".into());
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_organization_header::<VerboseError<&[u8]>>(b"Organization: Softphone 1.0\r\n")
    );
}
