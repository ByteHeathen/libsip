use libsip::{headers::parse::parse_authentication_info_header, Header};

#[test]
fn write() {
    let header = Header::AuthenticationInfo("<http://www.example.com/sounds/moo.wav>".into());
    assert_eq!(
        "Authentication-Info: <http://www.example.com/sounds/moo.wav>".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::AuthenticationInfo("<http://www.example.com/sounds/moo.wav>".into());
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_authentication_info_header(
            b"Authentication-Info: <http://www.example.com/sounds/moo.wav>\r\n"
        )
    );
}
