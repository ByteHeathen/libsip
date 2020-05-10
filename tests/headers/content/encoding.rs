use libsip::{
    headers::{parse::parse_content_encoding_header, ContentType},
    Header,
};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::ContentEncoding(ContentType::Sdp);
    assert_eq!(
        "Content-Encoding: application/sdp".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::ContentEncoding(ContentType::Sdp);
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_content_encoding_header::<VerboseError<&[u8]>>(b"Content-Encoding: application/sdp")
    );
}
