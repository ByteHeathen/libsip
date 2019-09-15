use libsip::headers::Header;
use libsip::headers::parse::parse_authorization_header;

#[test]
fn write() {
    let header = Header::Authorization("<http://www.example.com/sounds/moo.wav>".into());
    assert_eq!("Authorization: <http://www.example.com/sounds/moo.wav>".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Authorization("<http://www.example.com/sounds/moo.wav>".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_authorization_header(b"Authorization: <http://www.example.com/sounds/moo.wav>\n"));
}
