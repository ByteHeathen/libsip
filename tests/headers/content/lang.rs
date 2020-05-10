use libsip::{
    headers::{parse::parse_content_language_header, Language},
    Header,
};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::ContentLanguage(Language::English);
    assert_eq!("Content-Language: en".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::ContentLanguage(Language::English);
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_content_language_header::<VerboseError<&[u8]>>(b"Content-Language: en")
    );
}
