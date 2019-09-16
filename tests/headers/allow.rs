use libsip::*;
use libsip::headers::parse::parse_allow_header;

#[test]
fn write() {
    let header = Header::Allow(vec![Method::Invite, Method::Options]);
    assert_eq!("Allow: INVITE,OPTIONS".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Allow(vec![Method::Register, Method::Invite]);
    assert_eq!(Ok((remains.as_ref(), header)), parse_allow_header(b"Allow: REGISTER,INVITE\r\n"));
}
