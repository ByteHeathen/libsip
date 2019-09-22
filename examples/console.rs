/// if the `-v` is present then the registration requests
/// will be shown as well. This example Does not use the sip
/// codec.

extern crate libsip;
extern crate tokio;

use std::io;
use std::time::Duration;
use std::time::Instant;

use libsip::*;
use libsip::uri::Param;
use libsip::core::Transport;
use libsip::parse_message;
use tokio::net::UdpSocket;
use tokio::future::FutureExt;

const USERNAME: &'static str = "20";
const PASSWORD: &'static str = "program";
const SOCKET_ADDRESS: &'static str = "192.168.1.76:5060";
const SERVER_SOCK_ADDRESS: &'static str = "192.168.1.123:5060";

async fn registration_process(reg: &mut RegistrationManager, sock: &mut UdpSocket, verbose: bool) -> Result<(), failure::Error>{
    let mut buf = vec![0; 65535];
    let request = reg.get_request()?;
    if verbose {
        print_sip_message_send(&request);
    }
    sock.send_to(&format!("{}", request).as_ref(), SERVER_SOCK_ADDRESS).await?;
    let (amt, _src) = sock.recv_from(&mut buf).await?;
    let (_, msg) = parse_message(&buf[..amt]).unwrap();
    if verbose {
        print_sip_message_recv(&msg);
    }
    reg.set_challenge(msg)?;
    let auth_request = reg.get_request()?;
    if verbose {
        print_sip_message_send(&auth_request);
    }
    sock.send_to(format!("{}", auth_request).as_ref(), SERVER_SOCK_ADDRESS).await?;
    let (amt, _src) = sock.recv_from(&mut buf).await.unwrap();
    let (_, msg) = parse_message(&buf[..amt]).unwrap();
    if verbose {
        print_sip_message_recv(&msg);
    }
    match msg {
        SipMessage::Response { code, .. } => {
            if code == 200 {
                Ok(())
            } else {
                Err(From::from(io::Error::new(io::ErrorKind::InvalidInput, "Failed to authenticate")))
            }
        },
        _ => {
            Err(From::from(io::Error::new(io::ErrorKind::InvalidInput, "Failed to authenticate")))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let verbose = get_verbose();
    let mut sock = UdpSocket::bind(SOCKET_ADDRESS).await?;
    let mut registrar = RegistrationManager::new(account_uri(), local_uri(), Default::default());
    registrar.username(USERNAME);
    registrar.password(PASSWORD);
    registration_process(&mut registrar, &mut sock, verbose).await?;

    let mut buf = vec![0; 65535];
    let mut expire_time = Instant::now();
    loop {
        let timeout = registrar.expires() as u64;
        if expire_time.elapsed().as_secs() > timeout {
            registration_process(&mut registrar, &mut sock, verbose).await?;
            expire_time = Instant::now();
            continue;
        }
        let result = sock.recv_from(&mut buf)
            .timeout(Duration::from_secs(expire_time.elapsed().as_secs() + timeout)).await;
        match result {
            Ok(Ok(value)) => {
                let (_, msg) = parse_message(&buf[..value.0]).unwrap();
                println!("{}", msg);
                print_sip_message_recv(&msg);
            },
            Ok(Err(err)) => panic!("{}", err),
            Err(_err) => {
                registration_process(&mut registrar, &mut sock, verbose).await?;
                expire_time = Instant::now();
            }
        }
    }
}

fn account_uri() -> Uri {
    Uri::sip(ip_domain!(192, 168, 1, 123)).auth(uri_auth!("20"))
}

fn local_uri() -> Uri {
    Uri::sip(ip_domain!(192, 168, 1, 76)).auth(uri_auth!("20")).parameter(Param::Transport(Transport::Udp))
}

fn print_sip_message_send(msg: &SipMessage) {
    println!("\n>>>>>>>>>>>>>>>>>>\n{}", msg);
}

fn print_sip_message_recv(msg: &SipMessage) {
    println!("<<<<<<<<<<<<<<<<<<\n{}", msg);
}

fn get_verbose() -> bool {
    use std::env::args;
    for arg in args() {
        if arg == "-v" {
            return true;
        }
    }
    false
}
