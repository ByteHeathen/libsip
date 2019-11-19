use libsip::{
    headers::{parse::parse_accept_language_header, Language},
    Header,
};

#[test]
fn write() {
    let header = Header::AcceptLanguage(Language::English);
    assert_eq!("Accept-Language: en".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::AcceptLanguage(Language::English);
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_accept_language_header(b"Accept-Language: en")
    );
}
