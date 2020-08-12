use libsip::*;

use nom::error::{VerboseError, VerboseErrorKind, ErrorKind};


#[test]
fn read_simple() {
    let input = "INVITE sip:bob@biloxi.example.com SIP/2.0\r
Via: SIP/2.0/TCP client.atlanta.example.com:5060;branch=z9hG4bK74bf9\r
Max-For\r\n";
    let remains: &'static [u8] = &[77, 97, 120, 45, 70, 111, 114, 13, 10];
    assert_eq!(
        Err(nom::Err::Error(VerboseError { errors: vec![(remains.as_ref(), VerboseErrorKind::Nom(ErrorKind::Tag))] })),
        parse_request::<VerboseError<&[u8]>>(input.as_bytes())
    );
}
