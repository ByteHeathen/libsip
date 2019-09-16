use crate::core::SipMessage;
use crate::core::Method;
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
    cseq_counter: Vec<(Method, u32)>
}

impl RegistrationManager {

    pub fn new(cfg: Config) -> RegistrationManager {
        let mut cseq_counter = vec![];
        if cfg.add_cseq {
            for method in Method::all() {
                cseq_counter.push((method, 444));
            }
        }
        RegistrationManager { cfg, cseq_counter }
    }

    pub fn process(&mut self, req: SipMessage) -> Result<SipMessage, failure::Error> {
        if let SipMessage::Request { mut method, uri, version, mut headers, body } = req {
            if self.cfg.content_length {
                let length = body.len() as u32;
                headers.push(Header::ContentLength(length));
            }
            if self.cfg.add_cseq {
                for (_method, mut num) in &mut self.cseq_counter {
                    if _method == &mut method {
                        num += 1;
                        headers.push(Header::CSeq(num, method.clone()));
                    }
                }
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
