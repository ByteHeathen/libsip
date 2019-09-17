use crate::core::SipMessage;
use crate::headers::Header;

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    pub content_length: bool,
    pub add_cseq: bool,
    pub expires_header: Option<u32>,
    pub user_agent: Option<String>
}

impl Default for Config {
    fn default() -> Config {
        Config {
            content_length: true,
            add_cseq: true,
            expires_header: Some(60),
            user_agent: Some(format!("libsip {}", env!("CARGO_PKG_VERSION")))
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct RegistrationManager {
    cfg: Config,
    cseq_counter: u32
}

impl RegistrationManager {

    pub fn new(cfg: Config) -> RegistrationManager {
        RegistrationManager { cfg, cseq_counter: 444 }
    }

    pub fn process(&mut self, req: SipMessage) -> Result<SipMessage, failure::Error> {
        if let SipMessage::Request { method, uri, version, mut headers, body } = req {
            if self.cfg.content_length {
                let length = body.len() as u32;
                headers.push(Header::ContentLength(length));
            }
            if self.cfg.add_cseq {
                self.cseq_counter += 1;
                headers.push(Header::CSeq(self.cseq_counter, method));
            }
            if let Some(exp) = self.cfg.expires_header {
                headers.push(Header::Expires(exp));
            }
            if let Some(agent) = &self.cfg.user_agent {
                headers.push(Header::UserAgent(agent.clone()));
            }
            Ok(SipMessage::Request { method, uri, version, headers, body })
        } else { unreachable!() }
    }
}

impl Default for RegistrationManager {
    fn default() -> RegistrationManager {
        RegistrationManager::new(Default::default())
    }
}
