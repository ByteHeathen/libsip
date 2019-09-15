extern crate libsip;

use libsip::builder::MessageBuilder;
use libsip::core::Version;
use libsip::core::Method;
use libsip::core::SipMessage;
use libsip::uri::Uri;
use libsip::uri::Domain;

fn get_register_request() -> SipMessage {
    SipMessage::Request {
        method: Method::Register,
        uri: Uri::sip(Domain::Domain("example.com".into(), None)),
        version: Version::default(),
        headers: vec![],
        body: vec![]
    }
}

fn send_request(_req: SipMessage) -> Result<(), failure::Error> {
    Ok(())
}


fn main() -> Result<(), failure::Error>{
    let mut builder = MessageBuilder::default();

    let req = get_register_request();

    let final_req = builder.process(req)?;
    println!("{}", final_req);

    send_request(final_req)?;
    Ok(())
}
