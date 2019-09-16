use libsip::headers::Header;
use libsip::headers::parse::parse_proxy_require_header;

#[test]
fn write() {
    let header = Header::ProxyRequire("call@id.com".into());
    assert_eq!("Proxy-Require: call@id.com".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::ProxyRequire("call@id.com".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_proxy_require_header(b"Proxy-Require: call@id.com\r\n"));
}
