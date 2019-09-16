extern crate libsip;

use libsip::*;
use libsip::uri::Param;
use libsip::core::Transport;
use libsip::core::parse_message;
use libsip::registration::RegistrationManager;

use std::net::UdpSocket;

fn get_register_request() -> SipMessage {
    let to_uri = get_our_uri();
    let from_uri = to_uri.clone();
    let contact_uri = to_uri.clone();
    SipMessage::Request {
        method: Method::Register,
        uri: Uri::sip(ip_domain!(192,168,1,123)),
        version: Version::default(),
        headers: vec![
            Header::Via(format!("SIP/2.0/UDP 192.168.1.123;transport=UDP;branch=Some-Branch")),
            Header::MaxForwards(70),
            Header::From(named_header!(from_uri)),
            Header::To(named_header!(to_uri)),
            Header::Contact(named_header!(contact_uri)),
            Header::CallId("kjh34asdfasdfasdfasdf@192.168.1.123".into()),
            Header::Allow(vec![Method::Invite, Method::Ack, Method::Cancel]),
        ],
        body: vec![]
    }
}

fn get_our_uri() -> Uri {
    Uri::sip(ip_domain!(192, 168, 1, 76, 5060))
        .auth(uri_auth!("program"))
        .parameter(Param::Transport(Transport::Udp))
}

fn send_request_print_response(req: SipMessage) -> Result<(), failure::Error> {
    let addr = "0.0.0.0:5060";
    let sock = UdpSocket::bind(addr)?;
    sock.send_to(&format!("{}", req).as_ref(), "192.168.1.123:5060")?;
    let mut buf = vec![0; 65535];
    let (amt, src) = sock.recv_from(&mut buf)?;
    let (_, msg) = parse_message(&buf[..amt]).unwrap();
    println!("{:?} - {:?} - {:?}", src, amt, &msg);
    Ok(())
}


fn main() -> Result<(), failure::Error>{
    let mut builder = RegistrationManager::default();

    let req = get_register_request();

    let final_req = builder.process(req)?;
    println!("{}", final_req);

    send_request_print_response(final_req)?;
    Ok(())
}
