use libsip::*;

use nom::error::VerboseError;

#[test]
fn write_simple() {
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("user"));
    let req = RequestGenerator::new()
        .uri(uri)
        .method(Method::Register)
        .build()
        .unwrap();
    assert_eq!(
        "REGISTER sip:user@example.com SIP/2.0\r\n\r\n".to_string(),
        format!("{}", req)
    );
}

#[test]
fn write_complex() {
    let uri = Uri::sip(domain!("example.com"));
    let req = RequestGenerator::new()
        .uri(uri)
        .method(Method::Register)
        .headers(vec![Header::Expires(10), Header::ContentLength(5)])
        .body(vec![b'5'; 5])
        .build()
        .unwrap();
    assert_eq!(
        "REGISTER sip:example.com SIP/2.0\r\nExpires: 10\r\nContent-Length: 5\r\n\r\n55555"
            .to_string(),
        format!("{}", req)
    );
}

#[test]
fn read_simple() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("user"));
    let req = RequestGenerator::new()
        .uri(uri)
        .method(Method::Register)
        .build()
        .unwrap();
    assert_eq!(
        Ok((remains.as_ref(), req)),
        parse_request::<VerboseError<&[u8]>>(b"REGISTER sip:user@example.com SIP/2.0\r\n\r\n")
    );
}

#[test]
fn read_complex() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("user"));
    let req = RequestGenerator::new()
        .uri(uri)
        .method(Method::Register)
        .headers(vec![Header::Expires(10), Header::ContentLength(5)])
        .body(vec![b'6'; 5])
        .build()
        .unwrap();
    assert_eq!(Ok((remains.as_ref(), req)), parse_request::<VerboseError<&[u8]>>(b"REGISTER sip:user@example.com SIP/2.0\r\nExpires: 10\r\nContent-Length: 5\r\n\r\n66666"));
}
