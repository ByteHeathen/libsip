/// This example will register with a Registrar
/// and print all requests received from the server.
/// If the `-v` is present then the registration requests
/// will be shown as well. This example expects there to
/// be a SIP server running on IP address '192.168.1.129:5060'
/// and will attempt to connect to a sip account with the
/// username '20' and the password 'program'.
extern crate libsip;
extern crate tokio;

use std::{
    io::{Error as IoError, ErrorKind as IoErrorKind, Result as IoResult},
    time::{Duration, Instant},
};

use nom::error::VerboseError;

use libsip::*;
use tokio::net::UdpSocket;

const USERNAME: &'static str = "20";
const PASSWORD: &'static str = "program";
const SOCKET_ADDRESS: &'static str = "192.168.1.129:5060";
const SERVER_SOCK_ADDRESS: &'static str = "192.168.1.133:5060";

async fn registration_process(
    reg: &mut RegistrationManager,
    sock: &mut UdpSocket,
    verbose: bool,
) -> IoResult<()> {
    let mut buf = vec![0; 65535];
    let request = reg.get_request(&Default::default())?;
    if verbose {
        print_sip_message_send(&request);
    }
    sock.send_to(&format!("{}", request).as_ref(), SERVER_SOCK_ADDRESS)
        .await?;
    let (amt, _src) = sock.recv_from(&mut buf).await?;
    let (_, msg) = parse_message::<VerboseError<&[u8]>>(&buf[..amt]).unwrap();
    if verbose {
        print_sip_message_recv(&msg);
    }
    if msg.is_response() && msg.status_code() == Ok(200) {
        return Ok(());
    }
    reg.set_challenge(msg)?;
    let auth_request = reg.get_request(&Default::default())?;
    if verbose {
        print_sip_message_send(&auth_request);
    }
    sock.send_to(format!("{}", auth_request).as_ref(), SERVER_SOCK_ADDRESS)
        .await?;
    let (amt, _src) = sock.recv_from(&mut buf).await.unwrap();
    let (_, msg) = parse_message::<VerboseError<&[u8]>>(&buf[..amt]).unwrap();
    if verbose {
        print_sip_message_recv(&msg);
    }
    match msg {
        SipMessage::Response { code, .. } => {
            if code == 200 {
                Ok(())
            } else {
                Err(From::from(IoError::new(
                    IoErrorKind::InvalidInput,
                    "Failed to authenticate",
                )))
            }
        },
        _ => Err(From::from(IoError::new(
            IoErrorKind::InvalidInput,
            "Failed to authenticate",
        ))),
    }
}

#[tokio::main]
async fn main() -> IoResult<()> {
    let verbose = get_verbose();
    let mut sock = UdpSocket::bind(SOCKET_ADDRESS).await?;
    let mut registrar = RegistrationManager::new(account_uri(), local_uri());
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
        let timeout_duration = Duration::from_secs(expire_time.elapsed().as_secs() + timeout);
        let result = tokio::time::timeout(timeout_duration, sock.recv_from(&mut buf)).await;
        match result {
            Ok(Ok(value)) => {
                let (_, msg) = parse_message::<VerboseError<&[u8]>>(&buf[..value.0]).unwrap();
                println!("{}", msg);
                print_sip_message_recv(&msg);
            },
            Ok(Err(err)) => panic!("{}", err),
            Err(_err) => {
                registration_process(&mut registrar, &mut sock, verbose).await?;
                expire_time = Instant::now();
            },
        }
    }
}

fn account_uri() -> Uri {
    Uri::sip(ip_domain!(192, 168, 1, 133)).auth(uri_auth!("20"))
}

fn local_uri() -> Uri {
    Uri::sip(ip_domain!(192, 168, 1, 129))
        .auth(uri_auth!("20"))
        .parameter(UriParam::Transport(Transport::Udp))
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
