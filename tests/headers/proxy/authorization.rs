use libsip::{headers::parse::parse_proxy_authorization_header, Header};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::ProxyAuthorization("call@id.com".into());
    assert_eq!(
        "Proxy-Authorization: call@id.com".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::ProxyAuthorization("call@id.com".into());
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_proxy_authorization_header::<VerboseError<&[u8]>>(
            b"Proxy-Authorization: call@id.com\r\n"
        )
    );
}
