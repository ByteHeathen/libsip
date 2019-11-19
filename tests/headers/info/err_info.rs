use libsip::{headers::parse::parse_error_info_header, Header};

#[test]
fn write() {
    let header = Header::ErrorInfo("<http://www.example.com/sounds/moo.wav>".into());
    assert_eq!(
        "Error-Info: <http://www.example.com/sounds/moo.wav>".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::ErrorInfo("<http://www.example.com/sounds/moo.wav>".into());
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_error_info_header(b"Error-Info: <http://www.example.com/sounds/moo.wav>\r\n")
    );
}
