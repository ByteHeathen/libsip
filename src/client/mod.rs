mod registration;
pub use self::registration::RegistrationManager;

mod messaging;
pub use self::messaging::MessageHelper;
pub use self::messaging::MessageWriter;

use crate::Uri;
use crate::SipMessage;

use std::io::Result as IoResult;

/// Simple SIP client for implementing softphones.
/// Currently the only thing implemented is registration
/// and sending text messages. The only other feature planned
/// is an interface for sending & receiving calls.
pub struct SoftPhone {
    pub msg: MessageWriter,
    pub reg: RegistrationManager
}

impl SoftPhone {

    /// Create a new SoftPhone client. `local_uri` is the SipUri that you listen on
    /// and `account_uri` is the uri of your SIP user account.
    pub fn new(local_uri: Uri, account_uri: Uri) -> SoftPhone {
        SoftPhone {
            msg: MessageWriter::new(account_uri.clone()),
            reg: RegistrationManager::new(account_uri, local_uri, Default::default())
        }
    }

    /// Return a reference to the sip registration manager.
    pub fn registry(&self) -> &RegistrationManager {
        &self.reg
    }

    /// Return a mutable reference tp the sip registration manager.
    pub fn registry_mut(&mut self) -> &mut RegistrationManager {
        &mut self.reg
    }

    /// Return a reference to the message writer.
    pub fn messaging(&self) -> &MessageWriter {
        &self.msg
    }

    /// Return a mutable reference to the MessageWriter.
    pub fn messaging_mut(&mut self) -> &mut MessageWriter {
        &mut self.msg
    }

    /// Simple pass through method to get a registration request.
    pub fn get_register_request(&mut self) -> IoResult<SipMessage> {
        Ok(self.reg.get_request()?)
    }

    /// Set the received auth challenge request.
    pub fn set_register_challenge(&mut self, c: SipMessage) -> IoResult<()> {
        self.reg.set_challenge(c)?;
        Ok(())
    }

    /// Send a new Message to `uri`.
    pub fn write_message(&mut self, b: Vec<u8>, uri: Uri) -> IoResult<SipMessage> {
        Ok(self.msg.write_message(b, uri, self.reg.via_header())?)
    }
}
