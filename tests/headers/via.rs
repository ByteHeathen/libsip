use libsip::{
    core::*,
    headers::{parse::parse_via_header, via::ViaHeader},
    uri::Param,
    *,
};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = ViaHeader {
        version: Version::default(),
        transport: Transport::Udp,
        uri: Uri::new_schemaless(domain!("example.com")),
    };
    assert_eq!(
        "Via: SIP/2.0/UDP example.com".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec!['\r' as u8, '\n' as u8];
    let header = ViaHeader {
        version: Version::default(),
        transport: Transport::Udp,
        uri: Uri::new_schemaless(domain!("example.com")),
    };
    assert_eq!(
        Ok((remains.as_ref(), Header::Via(header))),
        parse_via_header::<VerboseError<&[u8]>>(b"Via: SIP/2.0/UDP example.com\r\n")
    );

    let input = b"Via: SIP/2.0/UDP 192.168.1.120;rport;branch=z9hG4bK7Q6y313Qrt6Uc\r\n";
    let remains = vec!['\r' as u8, '\n' as u8];
    let header = ViaHeader {
        version: Version::default(),
        transport: Transport::Udp,
        uri: Uri::new_schemaless(ip_domain!(192, 168, 1, 120))
            .parameter(Param::RPort)
            .parameter(Param::Branch("z9hG4bK7Q6y313Qrt6Uc".into())),
    };
    assert_eq!(
        Ok((remains.as_ref(), Header::Via(header))),
        parse_via_header::<VerboseError<&[u8]>>(input)
    );

    let input = b"Via: SIP/2.0/UDP 192.168.1.120;branch=03395ed83a7b9502c671c769bbe369cb;received=192.168.1.76\r\n";
    let remains = vec!['\r' as u8, '\n' as u8];
    let header = ViaHeader {
        version: Version::default(),
        transport: Transport::Udp,
        uri: Uri::new_schemaless(ip_domain!(192, 168, 1, 120))
            .parameter(Param::Branch("03395ed83a7b9502c671c769bbe369cb".into()))
            .parameter(Param::Received(ip_domain!(192, 168, 1, 76))),
    };
    assert_eq!(
        Ok((remains.as_ref(), Header::Via(header))),
        parse_via_header::<VerboseError<&[u8]>>(input)
    );
}
