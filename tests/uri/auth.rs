use libsip::{uri::parse_uriauth, uri_auth};

use nom::error::VerboseError;

#[test]
fn read_auth() {
    let remains = vec![];
    assert_eq!(
        Ok((remains.as_ref(), uri_auth!("username"))),
        parse_uriauth::<VerboseError<&[u8]>>(b"username@")
    );

    let remains = vec![];
    assert_eq!(
        Ok((remains.as_ref(), uri_auth!("username", "password"))),
        parse_uriauth::<VerboseError<&[u8]>>(b"username:password@")
    );
}
