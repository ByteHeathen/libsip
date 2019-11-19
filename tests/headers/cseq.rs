use libsip::{headers::parse::parse_cseq_header, Header, Method};

#[test]
fn write() {
    let header = Header::CSeq(444, Method::Register);
    assert_eq!("CSeq: 444 REGISTER".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::CSeq(60, Method::Register);
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_cseq_header(b"CSeq: 60 REGISTER\r\n")
    );
}
