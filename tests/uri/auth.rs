use libsip::{uri::parse_uriauth, uri_auth};

#[test]
fn read_auth() {
    let remains = vec![];
    assert_eq!(
        Ok((remains.as_ref(), uri_auth!("username"))),
        parse_uriauth(b"username@")
    );

    let remains = vec![];
    assert_eq!(
        Ok((remains.as_ref(), uri_auth!("username", "password"))),
        parse_uriauth(b"username:password@")
    );
}
