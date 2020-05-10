use libsip::{headers::parse::parse_alert_info_header, Header};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::AlertInfo("<http://www.example.com/sounds/moo.wav>".into());
    assert_eq!(
        "Alert-Info: <http://www.example.com/sounds/moo.wav>".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::AlertInfo("<http://www.example.com/sounds/moo.wav>".into());
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_alert_info_header::<VerboseError<&[u8]>>(b"Alert-Info: <http://www.example.com/sounds/moo.wav>\r\n")
    );
}
