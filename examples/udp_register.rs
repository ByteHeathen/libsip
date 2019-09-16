extern crate libsip;

use libsip::builder::RequestBuilder;
use libsip::core::Version;
use libsip::core::Method;
use libsip::core::Transport;
use libsip::headers::Header;
use libsip::core::SipMessage;
use libsip::uri::Uri;
use libsip::uri::UriAuth;
use libsip::uri::Domain;
use libsip::uri::Param;

use std::net::UdpSocket;
use std::net::Ipv4Addr;

fn get_register_request() -> SipMessage {
    SipMessage::Request {
        method: Method::Register,
        uri: Uri::sip(Domain::Ipv4(Ipv4Addr::new(192,168,1,123), None)),
        version: Version::default(),
        headers: vec![
            Header::Via(format!("SIP/2.0/UDP 192.168.1.123;transport=UDP;branch=Some-Branch")),
            Header::MaxForwards(70),
            Header::From(None, get_our_uri()),
            Header::To(None, get_our_uri()),
            Header::Contact(None, get_our_uri()),
            Header::CallId("kjh34asdfasdfasdfasdf@192.168.1.123".into()),
            Header::Allow(vec![Method::Invite, Method::Ack, Method::Cancel]),
        ],
        body: vec![]
    }
}

fn get_our_uri() -> Uri {
    Uri::sip(Domain::Ipv4(Ipv4Addr::new(192, 168, 1, 76), Some(5060)))
        .auth(UriAuth::new("program"))
        .parameter(Param::Transport(Transport::Udp))
}

fn send_request(req: SipMessage) -> Result<(), failure::Error> {
    let addr = "0.0.0.0:5060";
    let sock = UdpSocket::bind(addr)?;
    sock.send_to(&format!("{}", req).as_ref(), "192.168.1.123:5060")?;
    Ok(())
}


fn main() -> Result<(), failure::Error>{
    let mut builder = RequestBuilder::new(Default::default());

    let req = get_register_request();

    let final_req = builder.process(req)?;
    println!("{}", final_req);

    send_request(final_req)?;
    Ok(())
}
