use libsip::headers::Header;
use libsip::headers::parse::parse_retry_after_header;

#[test]
fn write() {
    let header = Header::RetryAfter("Softphone 1.0".into());
    assert_eq!("Retry-After: Softphone 1.0".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::RetryAfter("Softphone 1.0".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_retry_after_header(b"Retry-After: Softphone 1.0\n"));
}
