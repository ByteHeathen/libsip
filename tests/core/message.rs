use libsip::*;

use nom::error::VerboseError;

#[test]
fn read_message() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com"));
    let req = RequestGenerator::new()
        .uri(uri)
        .method(Method::Register)
        .build()
        .unwrap();
    assert_eq!(
        Ok((remains.as_ref(), req)),
        parse_message::<VerboseError<&[u8]>>(b"REGISTER sip:example.com SIP/2.0\r\n\r\n")
    );
}

#[test]
fn read_complex() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com"))
               .auth(uri_auth!("user"))
               .parameter(UriParam::RPort(None))
               .parameter(UriParam::Other("new".into(), None))
               .parameter(UriParam::Other("Some".into(), Some("Param".into())))
               .parameter(UriParam::Other("Other".into(), None));
    let req = RequestGenerator::new()
        .uri(uri)
        .method(Method::Register)
        .headers(vec![Header::Expires(10), Header::ContentLength(5)])
        .body(vec![b'6'; 5])
        .build()
        .unwrap();
    assert_eq!(Ok((
        remains.as_ref(), req)),
        parse_message::<VerboseError<&[u8]>>(b"REGISTER sip:user@example.com;rport;new;Some=Param;Other SIP/2.0\r\nExpires: 10\r\nContent-Length: 5\r\n\r\n66666")
    );
}
