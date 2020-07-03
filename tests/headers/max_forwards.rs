use libsip::{headers::parse::parse_max_forwards_header, *};

use nom::error::VerboseError;

#[test]
fn write() {
    let header = Header::MaxForwards(70);
    assert_eq!("Max-Forwards: 70".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::MaxForwards(60);
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_max_forwards_header::<VerboseError<&[u8]>>(b"Max-Forwards: 60\r\n")
    );
}
