use bytes::BytesMut;
use bytes::buf::BufMut;
use tokio_codec::{ Encoder, Decoder };

use std::io;

use crate::SipMessage;
use crate::parse_message;

pub struct SipCodec {

}

impl Default for SipCodec {
    fn default() -> SipCodec {
        SipCodec {}
    }
}

impl Encoder for SipCodec {
    type Item = SipMessage;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put(&format!("{}", item));
        Ok(())
    }
}

impl Decoder for SipCodec {
    type Item = SipMessage;
    type Error = io::Error;

    fn decode(&mut self, dst: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match parse_message(&dst) {
            Ok((remains, message)) => {
                if remains.is_empty() {
                    Ok(Some(message))
                } else {
                    println!("Unexpected remaining: {}", String::from_utf8_lossy(remains));
                    Ok(None)
                }
            },
            value => {
                panic!("{:?}", value);
            }
        }
    }
}
