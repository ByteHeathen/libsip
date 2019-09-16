use libsip::Header;
use libsip::headers::parse::parse_www_authenticate_header;

#[test]
fn write() {
    let header = Header::WwwAuthenticate("Softphone 1.0".into());
    assert_eq!("WWW-Authenticate: Softphone 1.0".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::WwwAuthenticate("Softphone 1.0".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_www_authenticate_header(b"WWW-Authenticate: Softphone 1.0\r\n"));
}
