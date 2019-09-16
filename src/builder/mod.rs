mod config;
pub use self::config::BuilderConfig;

use crate::core::SipMessage;
use crate::core::Method;
use crate::headers::Header;

#[derive(Debug, PartialEq, Clone)]
pub struct RequestBuilder {
    cfg: BuilderConfig,
    cseq_counter: Vec<(Method, u32)>
}

impl RequestBuilder {

    pub fn new(cfg: BuilderConfig) -> RequestBuilder {
        let mut cseq_counter = vec![];
        if cfg.add_cseq {        
            for method in Method::all() {
                cseq_counter.push((method, 444));
            }
        }
        RequestBuilder { cfg, cseq_counter }
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
