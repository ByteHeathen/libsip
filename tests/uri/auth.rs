use libsip::uri::UriAuth;
use libsip::uri::parse_uriauth;

#[test]
fn read_auth() {
    let remains = vec![];
    let auth = UriAuth::new("username");
    assert_eq!(Ok((remains.as_ref(), auth)), parse_uriauth(b"username@"));

    let remains = vec![];
    let auth = UriAuth::new("username").password("password");
    assert_eq!(Ok((remains.as_ref(), auth)), parse_uriauth(b"username:password@"));
}
