mod registration;
pub use self::registration::RegistrationManager;

mod messaging;
pub use self::messaging::MessageHelper;
pub use self::messaging::MessageWriter;

use crate::Uri;
use crate::SipMessage;

use std::io::Result as IoResult;

pub struct SoftPhone {
    msg: MessageWriter,
    reg: RegistrationManager
}

impl SoftPhone {

    pub fn new(local_uri: Uri, account_uri: Uri) -> SoftPhone {
        SoftPhone {
            msg: MessageWriter::new(account_uri.clone()),
            reg: RegistrationManager::new(account_uri, local_uri, Default::default())
        }
    }

    pub fn registry(&self) -> &RegistrationManager {
        &self.reg
    }

    pub fn registry_mut(&mut self) -> &mut RegistrationManager {
        &mut self.reg
    }

    pub fn messaging(&self) -> &MessageWriter {
        &self.msg
    }

    pub fn messaging_mut(&mut self) -> &mut MessageWriter {
        &mut self.msg
    }

    pub fn get_register_request(&mut self) -> IoResult<SipMessage> {
        Ok(self.reg.get_request()?)
    }

    pub fn set_register_challenge(&mut self, c: SipMessage) -> IoResult<()> {
        self.reg.set_challenge(c)?;
        Ok(())
    }

    pub fn write_message(&mut self, b: Vec<u8>, uri: Uri) -> IoResult<()> {
        self.msg.write_message(b, uri, self.reg.via_header())?;
        Ok(())
    }
}
