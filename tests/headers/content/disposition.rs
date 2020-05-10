use libsip::{headers::parse::parse_content_disposition_header, Header};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::ContentDisposition("<http://www.example.com/sounds/moo.wav>".into());
    assert_eq!(
        "Content-Disposition: <http://www.example.com/sounds/moo.wav>".to_string(),
        format!("{}", header)
    );
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::ContentDisposition("<http://www.example.com/sounds/moo.wav>".into());
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_content_disposition_header::<VerboseError<&[u8]>>(
            b"Content-Disposition: <http://www.example.com/sounds/moo.wav>\r\n"
        )
    );
}
