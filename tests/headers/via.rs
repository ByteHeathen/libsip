use libsip::*;
use libsip::core::*;
use libsip::uri::Param;
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

     let input = b"Via: SIP/2.0/UDP 192.168.1.120;rport;branch=z9hG4bK7Q6y313Qrt6Uc\r\n";
     let remains = vec![];
     let header = ViaHeader {
         version: Version::default(),
         transport: Transport::Udp,
         uri: Uri::new_schemaless(ip_domain!(192, 168, 1, 120)).parameter(Param::RPort).parameter(Param::Branch("z9hG4bK7Q6y313Qrt6Uc".into()))
     };
     assert_eq!(Ok((remains.as_ref(), Header::Via(header))), parse_via_header(input));
}
