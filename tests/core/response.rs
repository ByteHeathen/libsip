use libsip::{core::message::parse_response, *};

#[test]
fn write_simple() {
    let req = ResponseGenerator::new().code(200).build().unwrap();
    assert_eq!("SIP/2.0 200 OK\r\n\r\n".to_string(), format!("{}", req));
}

#[test]
fn write_complex() {
    let req = ResponseGenerator::new()
        .code(180)
        .header(Header::Expires(10))
        .header(Header::ContentLength(5))
        .body(vec![b'5'; 5])
        .build()
        .unwrap();
    assert_eq!(
        "SIP/2.0 180 Ringing\r\nExpires: 10\r\nContent-Length: 5\r\n\r\n55555".to_string(),
        format!("{}", req)
    );
}

#[test]
fn read_simple() {
    let remains = vec![];
    let req = ResponseGenerator::new().code(200).build().unwrap();
    assert_eq!(
        Ok((remains.as_ref(), req)),
        parse_response(b"SIP/2.0 200 OK\r\n\r\n")
    );
}

#[test]
fn read_complex() {
    let remains = vec![];
    let req = ResponseGenerator::new()
        .code(180)
        .header(Header::Expires(10))
        .header(Header::ContentLength(5))
        .body(vec![b'5'; 5])
        .build()
        .unwrap();
    assert_eq!(
        Ok((remains.as_ref(), req)),
        parse_response(b"SIP/2.0 180 Ringing\r\nExpires: 10\r\nContent-Length: 5\r\n\r\n55555")
    );
}
