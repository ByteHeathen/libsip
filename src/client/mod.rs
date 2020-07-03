//! The client module contains all of the code for
//! processing and generating SIP requests. Encapsulating
//! all this functionality is the SoftPhone struct.

mod registration;
pub use self::registration::RegistrationManager;

mod messaging;
pub use self::messaging::{MessageHelper, MessageWriter};

mod invite;
pub use self::invite::{InviteHelper, InviteWriter};

use crate::{Header, Headers, Method, SipMessage, Uri};

use std::{
    collections::HashMap,
    io::{Error as IoError, ErrorKind as IoErrorKind, Result as IoResult},
};

/// This struct is used in the client module when creating sip messages
/// it is used to specify some common values for the generated sip
/// headers.
pub struct HeaderWriteConfig {
    /// The Value to set for the User Agent header.
    /// By default this is set to libsip {version},
    /// Set to None to disable adding a User Agent header.
    pub user_agent: Option<String>,
    /// The value for the Allowed Methods Header.
    /// By default set to Invite, Cancel, Bye, Message.
    /// Set to None to disable adding header.
    pub allowed_methods: Option<Vec<Method>>,
}

impl HeaderWriteConfig {
    /// Write configured headers into the provided Vec.
    pub fn write_headers_vec(&self, m: &mut Vec<Header>) {
        if let Some(agent) = &self.user_agent {
            m.push(Header::UserAgent(agent.into()));
        }
        if let Some(allowed) = &self.allowed_methods {
            m.push(Header::Allow(allowed.clone()));
        }
    }

    /// Write configured headers into the provided Headers Map.
    pub fn write_headers(&self, m: &mut Headers) {
        if let Some(agent) = &self.user_agent {
            m.push(Header::UserAgent(agent.into()));
        }
        if let Some(allowed) = &self.allowed_methods {
            m.push(Header::Allow(allowed.clone()));
        }
    }
}

impl Default for HeaderWriteConfig {
    fn default() -> HeaderWriteConfig {
        HeaderWriteConfig {
            user_agent: Some(format!("libsip {}", env!("CARGO_PKG_VERSION"))),
            allowed_methods: Some(vec![
                Method::Invite,
                Method::Cancel,
                Method::Bye,
                Method::Message,
            ]),
        }
    }
}

/// Simple SIP client for implementing softphones.
/// Currently the only thing implemented is registration
/// and sending text messages. The only other feature planned
/// is an interface for sending & receiving calls.
pub struct SoftPhone {
    /// Header Configuration Used when adding default
    /// headers on generated SIP messages.
    header_cfg: HeaderWriteConfig,
    /// Message writer instance used when generating
    /// a SIP message.
    msg: MessageWriter,
    /// Invitation writer instance used when generating,
    /// a SIP call.
    invite: InviteWriter,
    /// Registration manage instance.
    reg: RegistrationManager,
    /// List of ongoing calls.
    calls: HashMap<String, InviteHelper>,
}

impl SoftPhone {
    /// Create a new SoftPhone client. `local_uri` is the SipUri that you listen on
    /// and `account_uri` is the uri of your SIP user account.
    pub fn new(local_uri: Uri, account_uri: Uri) -> SoftPhone {
        SoftPhone {
            header_cfg: HeaderWriteConfig::default(),
            msg: MessageWriter::new(account_uri.clone()),
            invite: InviteWriter::new(account_uri.clone()),
            reg: RegistrationManager::new(account_uri, local_uri),
            calls: HashMap::new(),
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

    /// Return a reference to the invite writer.
    pub fn invite(&self) -> &InviteWriter {
        &self.invite
    }

    /// Return a mutable reference to the InviteWriter.
    pub fn invite_mut(&mut self) -> &mut InviteWriter {
        &mut self.invite
    }

    /// Return a reference to the used HeaderWriteConfig.
    pub fn header_cfg(&self) -> &HeaderWriteConfig {
        &self.header_cfg
    }

    /// Return a mutable reference to the used HeaderWriteConfig.
    pub fn header_cfg_mut(&mut self) -> &mut HeaderWriteConfig {
        &mut self.header_cfg
    }

    /// Simple pass through method to get a registration request.
    pub fn get_register_request(&mut self) -> IoResult<SipMessage> {
        Ok(self.reg.get_request(&self.header_cfg)?)
    }

    /// Set the received auth challenge request.
    pub fn set_register_challenge(&mut self, c: SipMessage) -> IoResult<()> {
        self.reg.set_challenge(c)?;
        Ok(())
    }

    /// Send a new Message to `uri`.
    pub fn write_message(&mut self, b: Vec<u8>, uri: Uri) -> IoResult<SipMessage> {
        Ok(self
            .msg
            .write_message(b, uri, self.reg.via_header(), &self.header_cfg)?)
    }

    /// Send a new Invite Request to `uri`.
    pub fn send_invite(&mut self, body: Vec<u8>, uri: Uri) -> IoResult<SipMessage> {
        self.invite.generate_invite(uri, body)
    }

    /// Give the softphone a received call, returns the
    /// ringing response to be sent.
    pub fn get_received_request(&mut self, msg: SipMessage) -> IoResult<SipMessage> {
        let invite = InviteHelper::new(msg)?;
        let call_id = invite.call_id()?;
        let received = invite.ringing(&self.header_cfg)?;
        self.calls.insert(call_id, invite);
        Ok(received)
    }

    /// Get a SIP Message that will accept a previously
    /// recieved invitation.
    pub fn get_accept_request(&mut self, body: Vec<u8>, call: &str) -> IoResult<SipMessage> {
        if let Some(invite) = self.calls.get_mut(call) {
            Ok(invite.accept(body, &self.header_cfg)?)
        } else {
            Err(IoError::new(IoErrorKind::NotFound, "Call not found"))
        }
    }

    /// Get a SIP message that will close a previously
    /// received invitation.
    pub fn get_bye_request(&mut self, call: &str) -> IoResult<SipMessage> {
        if let Some(invite) = self.calls.get_mut(call) {
            Ok(invite.bye(&self.header_cfg)?)
        } else {
            Err(IoError::new(IoErrorKind::NotFound, "Call not found"))
        }
    }

    /// Get the messages required to cancel a invitation.
    pub fn get_cancel_request(&mut self, call: &str) -> IoResult<(SipMessage, SipMessage)> {
        if let Some(invite) = self.calls.get_mut(call) {
            invite.cancel(&self.header_cfg)
        } else {
            Err(IoError::new(IoErrorKind::NotFound, "Call not found"))
        }
    }
}
