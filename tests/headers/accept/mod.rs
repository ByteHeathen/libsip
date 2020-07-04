mod encoding;
mod lang;

use libsip::{headers::parse::parse_accept_header, *};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::Accept(vec![Method::Invite, Method::Options]);
    assert_eq!("Accept: INVITE,OPTIONS".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Accept(vec![Method::Register, Method::Invite]);
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_accept_header::<VerboseError<&[u8]>>(b"Accept: REGISTER, INVITE\r\n")
    );
}
