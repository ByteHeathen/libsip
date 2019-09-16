use libsip::headers::Header;
use libsip::headers::parse::parse_server_header;

#[test]
fn write() {
    let header = Header::Server("Softphone 1.0".into());
    assert_eq!("Server: Softphone 1.0".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Server("Softphone 1.0".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_server_header(b"Server: Softphone 1.0\r\n"));
}
