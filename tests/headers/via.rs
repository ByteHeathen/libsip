use libsip::*;
use libsip::core::*;
use libsip::headers::via::ViaHeader;
use libsip::headers::parse::parse_via_header;

#[test]
fn write() {
    let header = ViaHeader {
        version: Version::default(),
        transport: Transport::Udp,
        uri: Uri::new_schemaless(domain!("example.com"))
    };
    assert_eq!("Via: SIP/2.0/UDP example.com".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = ViaHeader {
        version: Version::default(),
        transport: Transport::Udp,
        uri: Uri::new_schemaless(domain!("example.com"))
    };
    assert_eq!(Ok((remains.as_ref(), Header::Via(header))), parse_via_header(b"Via: SIP/2.0/UDP example.com\r\n"));
}
