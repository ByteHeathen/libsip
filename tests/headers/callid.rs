use libsip::*;
use libsip::headers::parse::parse_callid_header;

#[test]
fn write() {
    let header = Header::CallId("Sofngfwertwowert.0".into());
    assert_eq!("Call-ID: Sofngfwertwowert.0".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::CallId("Sofngfwertwowert.0".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_callid_header(b"Call-ID: Sofngfwertwowert.0\r\n"));
}
