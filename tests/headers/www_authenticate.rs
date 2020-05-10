use libsip::*;
use nom::error::VerboseError;
use libsip::headers::parse::parse_www_authenticate_header;

use std::collections::HashMap;

#[test]
fn write() {
    let mut map = HashMap::new();
    map.insert("key".into(), "value".into());
    let header = Header::WwwAuthenticate(AuthHeader(AuthSchema::Digest, map));
    assert_eq!(
        "WWW-Authenticate: Digest key=\"value\"".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let mut map = HashMap::new();
    map.insert("key".into(), "value".into());
    let header = Header::WwwAuthenticate(AuthHeader(AuthSchema::Digest, map));
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_www_authenticate_header::<VerboseError<&[u8]>>(b"WWW-Authenticate: Digest key=value \r\n")
    );
}
