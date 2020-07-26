use crate::{headers::parse::parse_contact_header, *};

use nom::error::VerboseError;

#[test]
fn write() {
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(contact_header!(uri, "Guy"));
    assert_eq!(
        "Contact: Guy <sip:guy@example.com>".to_string(),
        format!("{}", header)
    );

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(contact_header!(uri, "Guy With Face"));
    assert_eq!(
        "Contact: \"Guy With Face\" <sip:guy@example.com>".to_string(),
        format!("{}", header)
    );

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(contact_header!(uri));
    assert_eq!(
        "Contact: sip:guy@example.com".to_string(),
        format!("{}", header)
    );

    //set a non quoted param (token)
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let mut contact_header = contact_header!(uri);
    contact_header.set_param("+sip.foo", Some("bar"));
    let header = Header::Contact(contact_header);
    assert_eq!(
        "Contact: sip:guy@example.com;+sip.foo=bar".to_string(),
        format!("{}", header)
    );

    //set a quoted param
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let mut contact_header = contact_header!(uri);
    contact_header.set_param(
        "+sip.instance",
        Some("\"<urn:uuid:1e020c2b-46f6-4867-9d11-65547b8967fa>\""),
    );
    let header = Header::Contact(contact_header);
    assert_eq!(
        "Contact: sip:guy@example.com;+sip.instance=\"<urn:uuid:1e020c2b-46f6-4867-9d11-65547b8967fa>\"".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(contact_header!(uri, "Guy"));
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_contact_header::<VerboseError<&[u8]>>(b"Contact: Guy <sip:guy@example.com>\r\n")
    );

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(contact_header!(uri, "Guy with face"));
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_contact_header::<VerboseError<&[u8]>>(
            b"Contact: \"Guy with face\" <sip:guy@example.com>\r\n"
        )
    );

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::Contact(contact_header!(uri));
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_contact_header::<VerboseError<&[u8]>>(b"Contact: <sip:guy@example.com>\r\n")
    );

    //set a non quoted param (token)
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let mut contact_header = contact_header!(uri);
    contact_header.set_param("+sip.foo", Some("bar"));
    let header = Header::Contact(contact_header);
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_contact_header::<VerboseError<&[u8]>>(
            b"Contact: <sip:guy@example.com>;+sip.foo=bar\r\n"
        )
    );

    //set a quoted param (token)
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let mut contact_header = contact_header!(uri);
    contact_header.set_param(
        "+sip.instance",
        Some("\"<urn:uuid:1e020c2b-46f6-4867-9d11-65547b8967fa>\""),
    );
    let header = Header::Contact(contact_header);
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_contact_header::<VerboseError<&[u8]>>(b"Contact: <sip:guy@example.com>;+sip.instance=\"<urn:uuid:1e020c2b-46f6-4867-9d11-65547b8967fa>\"\r\n")
    );
}
