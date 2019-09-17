use libsip::Header;
use libsip::headers::parse::parse_mime_version_header;

#[test]
fn write() {
    let header = Header::MimeVersion(1.0);
    assert_eq!("MIME-Version: 1".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![b' '];
    let header = Header::MimeVersion(1.0);
    assert_eq!(Ok((remains.as_ref(), header)), parse_mime_version_header(b"MIME-Version: 1.0 "));
}
