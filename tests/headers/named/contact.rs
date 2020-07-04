use libsip::{headers::parse::parse_contact_header, *};

use nom::error::VerboseError;

#[test]
fn write() {
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(named_header!(uri, "Guy"));
    assert_eq!(
        "Contact: Guy <sip:guy@example.com>".to_string(),
        format!("{}", header)
    );

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(named_header!(uri, "Guy With Face"));
    assert_eq!(
        "Contact: \"Guy With Face\" <sip:guy@example.com>".to_string(),
        format!("{}", header)
    );

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(named_header!(uri));
    assert_eq!(
        "Contact: sip:guy@example.com".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(named_header!(uri, "Guy"));
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_contact_header::<VerboseError<&[u8]>>(b"Contact: Guy <sip:guy@example.com>\r\n")
    );

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(named_header!(uri, "Guy with face"));
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_contact_header::<VerboseError<&[u8]>>(
            b"Contact: \"Guy with face\" <sip:guy@example.com>\r\n"
        )
    );

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(named_header!(uri));
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_contact_header::<VerboseError<&[u8]>>(b"Contact: <sip:guy@example.com>\r\n")
    );
}
