extern crate libsip;

use libsip::*;
use libsip::uri::Param;
use libsip::core::Transport;
use libsip::uri::parse_uri;
use libsip::core::message::parse_response;
use libsip::registration::RegistrationManager;

use std::net::UdpSocket;

fn get_our_uri() -> Uri {
    Uri::sip(ip_domain!(192, 168, 1, 76, 5060))
        .auth(uri_auth!("phone"))
        .parameter(Param::Transport(Transport::Udp))
}

fn send_request_get_response(req: SipMessage) -> Result<SipMessage, failure::Error> {
    let addr = "0.0.0.0:5060";
    let sock = UdpSocket::bind(addr)?;
    sock.send_to(&format!("{}", req).as_ref(), "192.168.1.123:5060")?;
    let mut buf = vec![0; 65535];
    let (amt, _src) = sock.recv_from(&mut buf)?;
    if let Err(nom::Err::Error((data, _))) = parse_response(&buf[..amt]) {
        panic!("{}", String::from_utf8_lossy(data));
    }
    let (_, msg) = parse_response(&buf[..amt]).unwrap();
    Ok(msg)
}


fn main() -> Result<(), failure::Error>{
    let acc_url = parse_uri(b"sip:20@192.168.1.123 ").unwrap().1
            .parameter(Param::Transport(Transport::Udp));
    let mut builder = RegistrationManager::new(acc_url, get_our_uri(), Default::default());
    builder.username("20");
    builder.password("program");

    let req = builder.get_request()?;
    println!("{}", req);

    let res = send_request_get_response(req)?;
    println!("{}\n", &res);
    builder.set_challenge(res)?;

    let authed = builder.get_request()?;
    println!("\n{}\n", authed);

    let res = send_request_get_response(authed)?;
    println!("{}\n", res);
    Ok(())
}
