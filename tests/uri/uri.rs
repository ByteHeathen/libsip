use libsip::*;

use nom::error::VerboseError;

#[test]
fn read_uri() {
    let expected_remains = vec![b' '];
    assert_eq!(
        Ok((expected_remains.as_ref(), Uri::sip(domain!("hostname")))),
        parse_uri::<VerboseError<&[u8]>>(b"sip:hostname ")
    );

    let expected_remains = vec![b' '];
    assert_eq!(
        Ok((expected_remains.as_ref(), Uri::sips(domain!("hostname")))),
        parse_uri::<VerboseError<&[u8]>>(b"sips:hostname ")
    );

    let expected_remains = vec![b' '];
    assert_eq!(
        Ok((
            expected_remains.as_ref(),
            Uri::new_schemaless(domain!("hostname"))
        )),
        parse_uri::<VerboseError<&[u8]>>(b"hostname ")
    );

    let expected_remains = vec![b' '];
    assert_eq!(
        Ok((
            expected_remains.as_ref(),
            Uri::new_schemaless(domain!("hostname"))
        )),
        parse_uri::<VerboseError<&[u8]>>(b"hostname ")
    );

    let expected_remains = vec![b' '];
    assert_eq!(
        Ok((
            expected_remains.as_ref(),
            Uri::sip(ip_domain!(10, 1, 10, 1))
        )),
        parse_uri::<VerboseError<&[u8]>>(b"sip:10.1.10.1 ")
    );

    let expected_remains = vec![b' '];
    let expected = Uri::sip(domain!("hostname.com")).auth(uri_auth!("username"));
    assert_eq!(
        Ok((expected_remains.as_ref(), expected)),
        parse_uri::<VerboseError<&[u8]>>(b"sip:username@hostname.com ")
    );

    let expected_remains = vec![b' '];
    let expected = Uri::sip(domain!("hostname.com")).auth(uri_auth!("username", "password"));
    assert_eq!(
        Ok((expected_remains.as_ref(), expected)),
        parse_uri::<VerboseError<&[u8]>>(b"sip:username:password@hostname.com ")
    );

    let expected_remains = vec![b' '];
    let expected = Uri::sip(domain!("hostname.com", 8080)).auth(uri_auth!("username", "password"));
    assert_eq!(
        Ok((expected_remains.as_ref(), expected)),
        parse_uri::<VerboseError<&[u8]>>(b"sip:username:password@hostname.com:8080 ")
    );

    let expected_remains = vec![b' '];
    let expected = Uri::sip(domain!("hostname.com", 8080))
        .parameter(UriParam::Transport(Transport::Udp))
        .auth(uri_auth!("username", "password"));
    assert_eq!(
        Ok((expected_remains.as_ref(), expected)),
        parse_uri::<VerboseError<&[u8]>>(b"sip:username:password@hostname.com:8080;transport=UDP ")
    );

    let expected_remains = vec![b' '];
    let expected = Uri::sip(domain!("hostname.com", 8080))
        .parameter(UriParam::Transport(Transport::Tcp))
        .parameter(UriParam::Other("Some".into(), Some("Other".into())))
        .auth(uri_auth!("username", "password"));
    assert_eq!(
        Ok((expected_remains.as_ref(), expected)),
        parse_uri::<VerboseError<&[u8]>>(
            b"sip:username:password@hostname.com:8080;transport=TCP;Some=Other "
        )
    );
}

#[test]
fn write_uri() {
    let uri = Uri::sip(domain!("hostname"));
    assert_eq!("sip:hostname".to_string(), format!("{}", uri));

    let uri = Uri::new_schemaless(domain!("hostname"));
    assert_eq!("hostname".to_string(), format!("{}", uri));

    let uri = Uri::sip(ip_domain!(10, 1, 10, 1));
    assert_eq!("sip:10.1.10.1".to_string(), format!("{}", uri));

    let uri = Uri::new_schemaless(ip_domain!(10, 1, 10, 1));
    assert_eq!("10.1.10.1".to_string(), format!("{}", uri));

    let uri = Uri::sip(domain!("hostname.com")).auth(UriAuth::new("username"));
    assert_eq!("sip:username@hostname.com".to_string(), format!("{}", uri));

    let uri = Uri::sip(domain!("hostname.com")).auth(uri_auth!("username", "password"));
    assert_eq!(
        "sip:username:password@hostname.com".to_string(),
        format!("{}", uri)
    );

    let uri = Uri::sip(domain!("hostname.com", 8080)).auth(uri_auth!("username", "password"));
    assert_eq!(
        "sip:username:password@hostname.com:8080".to_string(),
        format!("{}", uri)
    );

    let uri = Uri::sip(domain!("hostname.com", 8080))
        .parameter(UriParam::Transport(Transport::Udp))
        .auth(uri_auth!("username", "password"));
    assert_eq!(
        "sip:username:password@hostname.com:8080;transport=UDP",
        format!("{}", uri)
    );

    let uri = Uri::new_schemaless(domain!("hostname.com", 8080))
        .parameter(UriParam::Transport(Transport::Udp))
        .auth(uri_auth!("username", "password"));
    assert_eq!(
        "username:password@hostname.com:8080;transport=UDP",
        format!("{}", uri)
    );
}
