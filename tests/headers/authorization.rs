use libsip::Header;
use libsip::headers::auth::*;

use libsip::headers::parse::parse_authorization_header;

use std::collections::HashMap;

#[test]
fn write() {
    let mut map = HashMap::new();
    map.insert("key".into(), "value".into());
    let header = Header::Authorization(AuthHeader(Schema::Digest, map));
    assert_eq!("Authorization: Digest key=\"value\"".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let mut map = HashMap::new();
    map.insert("key".into(), "value".into());
    let header = Header::Authorization(AuthHeader(Schema::Digest, map));
    assert_eq!(Ok((remains.as_ref(), header)), parse_authorization_header(b"Authorization: Digest key=value \r\n"));
}
