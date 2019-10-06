use crate::*;
use crate::uri::Param;
use crate::uri::UriAuth;
use crate::core::Transport;
use crate::headers::NamedHeader;
use crate::headers::auth::Schema;
use crate::headers::auth::AuthHeader;
use crate::headers::via::ViaHeader;
use crate::RequestGenerator;

use std::collections::HashMap;
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
    /// The username to use for login.
    pub user: Option<String>,
    /// The password to use for login.
    pub pass: Option<String>,
    /// Authentication realm
    realm: Option<String>,
    /// Authentication nonce
    nonce: Option<String>
}

impl Default for Config {
    fn default() -> Config {
        Config {
            content_length: true,
            expires_header: Some(60),
            user_agent: Some(format!("libsip {}", env!("CARGO_PKG_VERSION"))),
            user: None,
            pass: None,
            realm: None,
            nonce: None
        }
    }
}

/// Handle's the SIP registration process.
/// This structure is designed to handle the authentication
/// process from a SoftPhone's point of view.
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
    call_id: String
}

impl RegistrationManager {

    /// Create a new Registration Manager typically this will happen once in a program's
    /// lifecycle. `account_uri` is the sip uri used to authenticate with and `local_uri`
    /// is the sip uri of the listening socket.
    pub fn new(account_uri: Uri, local_uri: Uri, cfg: Config) -> RegistrationManager {
        RegistrationManager {
            account_uri, local_uri, cfg,
            cseq_counter: 444,
            auth_header: None,
            nonce_c: 1,
            c_nonce: None,
            branch: format!("{:x}", md5::compute(rand::random::<[u8 ; 16]>())),
            call_id: format!("{:x}", md5::compute(rand::random::<[u8 ; 16]>()))
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
        let to_header = self.account_uri.clone();
        let from_header = self.account_uri.clone();
        let mut contact_header = self.local_uri.clone();
        if let Some(name) = &self.cfg.user {
            contact_header = contact_header.auth(UriAuth::new(name));
        }

        let mut headers = vec![
            Header::ContentLength(0),
            Header::To(NamedHeader::new(to_header)),
            Header::From(NamedHeader::new(from_header)),
            Header::Contact(NamedHeader::new(contact_header)),
            Header::CSeq(self.cseq_counter, Method::Register),
            Header::CallId(format!("{}@{}", self.call_id, self.account_uri.host())),
            self.via_header()
        ];

        if let Some(exp) = self.cfg.expires_header {
            headers.push(Header::Expires(exp));
        }
        if let Some(agent) = &self.cfg.user_agent {
            headers.push(Header::UserAgent(agent.clone()));
        }
        self.add_auth_header(&mut headers);

        Ok(
            RequestGenerator::new()
                .method(Method::Register)
                .uri(self.account_uri.clone())
                .headers(headers)
                .build()?
        )
    }

    /// After the first register request is sent. pass the received sip response
    /// to this function to perform compute the hashed password.
    pub fn set_challenge(&mut self, msg: SipMessage) -> Result<(), io::Error> {
        if let SipMessage::Response { headers, .. } = msg {
            for item in headers.iter() {
                match item {
                    Header::WwwAuthenticate(auth) => self.set_auth_vars(auth)?,
                    Header::Expires(expire) => { self.cfg.expires_header = Some(expire.clone()); },
                    _ => {}
                }
            }
            Ok(())
        } else {
            unreachable!()
        }
    }

    fn set_auth_vars(&mut self, d: &AuthHeader) -> Result<(), io::Error> {
        if let Some(realm) = d.1.get("realm") {
            self.cfg.realm = Some(realm.into());
        }
        if let Some(nonce) = d.1.get("nonce") {
            self.cfg.nonce = Some(nonce.into());
        }
        match d.0 {
            Schema::Digest => self.handle_md5_auth()
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
        let via_uri = self.account_uri.clone()
                    .parameter(Param::Branch(self.branch.clone()))
                    .authless()
                    .schemaless();
        Header::Via(ViaHeader { uri: via_uri, version: Default::default(), transport: Transport::Udp})
    }

    fn handle_md5_auth(&mut self) -> Result<(), io::Error> {
        if let Some(realm) = &self.cfg.realm {
            if let Some(nonce) = &self.cfg.nonce {
                if let Some(user) = &self.cfg.user {
                    if let Some(pass) = &self.cfg.pass {
                        let mut map = HashMap::new();
                        let cnonce = self.generate_cnonce();
                        map.insert("username".into(), user.clone());
                        map.insert("nonce".into(), format!("{}", nonce));
                        map.insert("realm".into(), realm.clone());
                        map.insert("uri".into(), format!("{}", self.account_uri));
                        map.insert("qop".into(), "auth".into());
                        map.insert("algorithm".into(), "MD5".into());
                        map.insert("cnonce".into(), format!("{:x}", cnonce));
                        map.insert("nc".into(), format!("{}", self.nonce_c));
                        let ha1 = md5::compute(&format!("{}:{}:{}", user, realm, pass));
                        let ha2 = md5::compute(format!("REGISTER:{}", self.account_uri.clone()));
                        let digest = format!("{:x}:{}:{:x}:{:x}:auth:{:x}", ha1, nonce, self.nonce_c, cnonce, ha2);
                        let pass = md5::compute(digest);
                        map.insert("response".into(), format!("{:x}", pass));
                        self.auth_header = Some(AuthHeader(Schema::Digest, map));
                    }
                }
            }
        }
        Ok(())
    }

    fn generate_cnonce(&self) -> md5::Digest {
        md5::compute(rand::random::<[u8 ; 16]>())
    }

    fn add_auth_header(&self, headers: &mut Vec<Header>) {
        if let Some(header) = self.auth_header.clone() {
            headers.push(Header::Authorization(header));
        }
    }
}
