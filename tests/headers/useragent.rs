use libsip::headers::Header;
use libsip::headers::parse::parse_useragent_header;

#[test]
fn write() {
    let header = Header::UserAgent("Softphone 1.0".into());
    assert_eq!("User-Agent: Softphone 1.0".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::UserAgent("Softphone 1.0".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_useragent_header(b"User-Agent: Softphone 1.0\n"));
}
