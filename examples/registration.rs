//! This example was created using a virtualbox vm
//! running ubuntu 19.04 and a SIP server running at
//! 192.168.1.123:5060
//!
//! This will attempt to register a sip account and print
//! the returned OK response.
//!
//! This expects and extension with the numeric id `20`
//! and a password of `program`

extern crate libsip;

use libsip::*;

use std::{io::Result as IoResult, net::UdpSocket};

use nom::error::VerboseError;

fn get_our_uri() -> Uri {
    Uri::sip(ip_domain!(192, 168, 1, 129, 5060))
        .auth(uri_auth!("20"))
        .parameter(UriParam::Transport(Transport::Udp))
}

fn send_request_get_response(req: SipMessage) -> IoResult<SipMessage> {
    let addr = "0.0.0.0:5060";
    let sock = UdpSocket::bind(addr)?;
    sock.send_to(&format!("{}", req).as_ref(), "192.168.1.133:5060")?;
    let mut buf = vec![0; 65535];
    let (amt, _src) = sock.recv_from(&mut buf)?;
    if let Err(nom::Err::Error((data, _))) = parse_response(&buf[..amt]) {
        panic!("{}", String::from_utf8_lossy(data));
    }
    let (_, msg) = parse_response::<VerboseError<&[u8]>>(&buf[..amt]).unwrap();
    Ok(msg)
}

fn main() -> IoResult<()> {
    let acc_url = parse_uri::<VerboseError<&[u8]>>(b"sip:20@192.168.1.133 ")
        .unwrap()
        .1
        .parameter(UriParam::Transport(Transport::Udp));
    let mut builder = RegistrationManager::new(acc_url, get_our_uri());
    builder.username("20");
    builder.password("program");

    let req = builder.get_request(&Default::default())?;
    println!("{}", req);

    let res = send_request_get_response(req)?;
    println!("{}\n", &res);
    builder.set_challenge(res)?;

    let authed = builder.get_request(&Default::default())?;
    println!("\n{}\n", authed);

    let res = send_request_get_response(authed)?;
    println!("{}\n", res);
    Ok(())
}
