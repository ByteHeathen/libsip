use libsip::{headers::parse::parse_proxy_authenticate_header, Header};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::ProxyAuthenticate("call@id.com".into());
    assert_eq!(
        "Proxy-Authenticate: call@id.com".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::ProxyAuthenticate("call@id.com".into());
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_proxy_authenticate_header::<VerboseError<&[u8]>>(
            b"Proxy-Authenticate: call@id.com\r\n"
        )
    );
}
