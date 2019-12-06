use crate::{
    core::Transport,
    headers::{
        auth::{AuthContext, AuthHeader},
        via::ViaHeader,
        NamedHeader,
    },
    uri::{Param, UriAuth},
    RequestGenerator, *,
};

use std::io;

/// Configuration used to build the register request.
#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    /// Whether to append the `Content-Length` header
    /// default: true
    pub content_length: bool,
    /// The value to use for the `Expiration` header
    /// default: 60
    pub expires_header: Option<u32>,
    /// The value to set for the user_agent header,
    /// default: `libsip env!('CARGO_PKG_VERSION')`
    pub user_agent: Option<String>,
    /// Methods sent in the allowed methods header.
    /// default: Invite, Cancel, Bye, Message
    pub allowed_methods: Vec<Method>,
    /// The username to use for login.
    pub user: Option<String>,
    /// The password to use for login.
    pub pass: Option<String>,
    /// Authentication realm
    realm: Option<String>,
    /// Authentication nonce
    nonce: Option<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            content_length: true,
            expires_header: Some(60),
            user_agent: Some(format!("libsip {}", env!("CARGO_PKG_VERSION"))),
            allowed_methods: vec![
              Method::Invite, Method::Cancel,
              Method::Bye, Method::Message
            ],
            user: None,
            pass: None,
            realm: None,
            nonce: None,
        }
    }
}

/// Handle's the SIP registration process.
/// This structure is designed to handle the authentication
/// process from a SoftPhone's point of view.
///
/// Currently only Digest auth authentication is implemented.
#[derive(Debug, PartialEq, Clone)]
pub struct RegistrationManager {
    /// Uri representing the account to attempt to register.
    account_uri: Uri,
    /// Uri representing the local machine used to register.
    local_uri: Uri,
    cfg: Config,
    /// Current REGISTER cseq count number.
    cseq_counter: u32,
    /// The computed hash nonce count.
    nonce_c: u32,
    /// The CNonce value of the computer hash.
    c_nonce: Option<String>,
    /// The Finished computed auth header.
    auth_header: Option<AuthHeader>,
    /// The branch to use for registration.
    branch: String,
    /// The Call Id to use for register requests.
    call_id: String,
}

impl RegistrationManager {
    /// Create a new Registration Manager typically this will happen once in a program's
    /// lifecycle. `account_uri` is the sip uri used to authenticate with and `local_uri`
    /// is the sip uri of the listening socket.
    pub fn new(account_uri: Uri, local_uri: Uri, cfg: Config) -> RegistrationManager {
        RegistrationManager {
            account_uri,
            local_uri,
            cfg,
            cseq_counter: 444,
            auth_header: None,
            nonce_c: 1,
            c_nonce: None,
            branch: format!("{:x}", md5::compute(rand::random::<[u8; 16]>())),
            call_id: format!("{:x}", md5::compute(rand::random::<[u8; 16]>())),
        }
    }

    /// Set the username used in the authentication process.
    pub fn username<S: Into<String>>(&mut self, s: S) {
        self.cfg.user = Some(s.into());
    }

    /// Set the password used in the authentication process.
    pub fn password<S: Into<String>>(&mut self, p: S) {
        self.cfg.pass = Some(p.into());
    }

    /// Get the register request. if this method is called before `set_challenge`
    /// then no authentication header will be set, if called after `set_challenge`
    /// then the Authorization header will be set.
    pub fn get_request(&mut self) -> Result<SipMessage, io::Error> {
        self.cseq_counter += 1;
        self.nonce_c += 1;
        let to_header = self.account_uri.clone();
        let from_header = self.account_uri.clone();
        let mut contact_header = self.local_uri.clone();
        let mut headers = vec![];

        if let Some(name) = &self.cfg.user {
            contact_header = contact_header.auth(UriAuth::new(name));
            if let Some(auth_header) = &self.auth_header {
                if let Some(pass) = &self.cfg.pass {
                    let ctx = AuthContext {
                        user: &name,
                        pass,
                        nc: self.nonce_c,
                        uri: &self.account_uri,
                    };
                    headers.push(Header::Authorization(auth_header.authenticate(ctx)?));
                }
            }
        }
        headers.push(Header::ContentLength(0));
        headers.push(Header::To(NamedHeader::new(to_header)));
        headers.push(Header::From(NamedHeader::new(from_header)));
        headers.push(Header::Contact(NamedHeader::new(contact_header)));
        headers.push(Header::CSeq(self.cseq_counter, Method::Register));
        headers.push(Header::CallId(format!(
            "{}@{}",
            self.call_id,
            self.account_uri.host()
        )));
        headers.push(self.via_header());
        headers.push(self.allowed_methods());

        if let Some(exp) = self.cfg.expires_header {
            headers.push(Header::Expires(exp));
        }
        if let Some(agent) = &self.cfg.user_agent {
            headers.push(Header::UserAgent(agent.clone()));
        }
        Ok(RequestGenerator::new()
            .method(Method::Register)
            .uri(self.account_uri.clone().authless())
            .headers(headers)
            .build()?)
    }

    /// After the first register request is sent. pass the received sip response
    /// to this function to perform compute the hashed password.
    pub fn set_challenge(&mut self, msg: SipMessage) -> Result<(), io::Error> {
        if let SipMessage::Response { headers, .. } = msg {
            for item in headers.into_iter() {
                match item {
                    Header::WwwAuthenticate(auth) => {
                        self.auth_header = Some(auth);
                    },
                    Header::Expires(expire) => {
                        self.cfg.expires_header = Some(expire);
                    },
                    _ => {},
                }
            }
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Challenge Response was not a SIP response",
            ))
        }
    }

    /// Retreive the expires header value.
    pub fn expires(&self) -> u32 {
        self.cfg.expires_header.unwrap_or(60)
    }

    /// Retreive the current cseq counter.
    pub fn cseq(&self) -> u32 {
        self.cseq_counter
    }

    /// Retreive the via header being used to represent the local
    /// listening socket.
    pub fn via_header(&self) -> Header {
        let via_uri = self
            .local_uri
            .clone()
            .parameter(Param::Branch(self.branch.clone()))
            .authless()
            .schemaless();
        Header::Via(ViaHeader {
            uri: via_uri,
            version: Default::default(),
            transport: Transport::Udp,
        })
    }

    pub fn allowed_methods(&self) -> Header {
        Header::Allow(self.cfg.allowed_methods.clone())
    }
}
