use libsip::headers::Header;
use libsip::headers::parse::parse_proxy_authenticate_header;

#[test]
fn write() {
    let header = Header::ProxyAuthenticate("call@id.com".into());
    assert_eq!("Proxy-Authenticate: call@id.com".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::ProxyAuthenticate("call@id.com".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_proxy_authenticate_header(b"Proxy-Authenticate: call@id.com\r\n"));
}
