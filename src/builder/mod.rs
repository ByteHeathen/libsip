mod config;
pub use self::config::BuilderConfig;

use crate::core::SipMessage;
use crate::headers::Header;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageBuilder {
    cfg: BuilderConfig,
}

impl MessageBuilder {

    pub fn new(cfg: BuilderConfig) -> MessageBuilder {
        MessageBuilder { cfg }
    }

    pub fn process(&mut self, req: SipMessage) -> Result<SipMessage, failure::Error> {
        if req.is_request() {
            self.process_request(req)
        } else {
            self.process_response(req)
        }
    }

    fn process_request(&mut self, mut req: SipMessage) -> Result<SipMessage, failure::Error> {
        if self.cfg.req_content_length_header {
            let length = req.body().len() as u32;
            req.headers_mut().push(Header::ContentLength(length));
        }
        Ok(req)
    }

    fn process_response(&mut self, mut req: SipMessage) -> Result<SipMessage, failure::Error> {
        if self.cfg.res_content_length_header {
            let length = req.body().len() as u32;
            req.headers_mut().push(Header::ContentLength(length));
        }
        Ok(req)
    }
}
