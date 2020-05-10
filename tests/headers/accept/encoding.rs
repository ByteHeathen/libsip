use libsip::{
    headers::{parse::parse_accept_encoding_header, ContentType},
    Header,
};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::AcceptEncoding(ContentType::Sdp);
    assert_eq!(
        "Accept-Encoding: application/sdp".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::AcceptEncoding(ContentType::Sdp);
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_accept_encoding_header::<VerboseError<&[u8]>>(b"Accept-Encoding: application/sdp")
    );
}
