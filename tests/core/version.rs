use libsip::{core::parse_version, Version};

use nom::error::VerboseError;

#[test]
fn write_version() {
    assert_eq!("SIP/2.0".to_string(), format!("{}", Version::default()));
    assert_eq!("SIP/1.1".to_string(), format!("{}", Version::new(1, 1)));
}

#[test]
fn read_version() {
    let remains = vec![b' '];
    assert_eq!(
        Ok((remains.as_ref(), Version::default())),
        parse_version::<VerboseError<&[u8]>>(b"SIP/2.0 ")
    );
    assert_eq!(
        Ok((remains.as_ref(), Version::new(1, 1))),
        parse_version::<VerboseError<&[u8]>>(b"SIP/1.1 ")
    );
}
