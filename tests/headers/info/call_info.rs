use libsip::Header;
use libsip::headers::parse::parse_call_info_header;

#[test]
fn write() {
    let header = Header::CallInfo("<http://www.example.com/sounds/moo.wav>".into());
    assert_eq!("Call-Info: <http://www.example.com/sounds/moo.wav>".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::CallInfo("<http://www.example.com/sounds/moo.wav>".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_call_info_header(b"Call-Info: <http://www.example.com/sounds/moo.wav>\r\n"));
}
