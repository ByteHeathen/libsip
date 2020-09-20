use libsip::{headers::parse::parse_authorization_header, *};

use nom::error::VerboseError;

use std::collections::HashMap;

#[test]
fn write() {
    let mut map = HashMap::new();
    map.insert("key".into(), "value".into());
    let header = Header::Authorization(AuthHeader(AuthSchema::Digest, map));
    assert_eq!(
        "Authorization: Digest key=\"value\"".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let mut map = HashMap::new();
    map.insert("key".into(), "value".into());
    let header = Header::Authorization(AuthHeader(AuthSchema::Digest, map));
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_authorization_header::<VerboseError<&[u8]>>(b"Authorization: Digest key=value \r\n")
    );
}

#[test]
fn qop_write() {
    let mut map = HashMap::new();
    map.insert("qop".into(), "auth".into());
    let header = AuthHeader(AuthSchema::Digest, map);
    assert_eq!(
        format!("{}", header),
        String::from("Digest qop=auth")
    );
}

#[test]
fn auth_write() {
    let mut map = HashMap::new();
    map.insert("nc".into(), "1".into());
    let header = AuthHeader(AuthSchema::Digest, map);
    assert_eq!(
        format!("{}", header),
        String::from("Digest nc=1")
    );
}
