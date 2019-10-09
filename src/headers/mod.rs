mod named;
mod write;
mod content;
mod language;
pub mod auth;
pub mod parse;
pub mod via;
pub use self::content::ContentType;
pub use self::language::Language;
pub use self::named::NamedHeader;
pub use self::parse::parse_header;


use crate::core::Method;

/// Wrapper around a Vec<Header> to simplify creating
/// and a list of headers
#[derive(Debug, PartialEq, Clone)]
pub struct Headers(pub Vec<Header>);

impl Headers {

    pub fn new() -> Headers {
        Headers(vec![])
    }

    /// Push a new Header onto the stack.
    pub fn push(&mut self, h: Header) {
        self.0.push(h)
    }

    /// Access the underlying vec iterator.
    pub fn iter(&self) -> impl Iterator<Item=&Header> {
        self.0.iter()
    }

    /// Add the Headers onto the interior Vec<Header>.
    pub fn extend(&mut self, i: Vec<Header>) {
        self.0.extend(i)
    }

    /// Return An Expires header if one is present.
    pub fn expires(&self) -> Option<Header> {
        for h in &self.0 {
            if let Header::Expires(i) = h {
                return Some(Header::Expires(i.clone()));
            }
        }
        None
    }

    /// Return the CSeq header if one is present.
    pub fn cseq(&self) -> Option<Header> {
        for h in &self.0 {
            if let Header::CSeq(a, b) = h {
                return Some(Header::CSeq(a.clone(), b.clone()));
            }
        }
        None
    }

    /// Return the From header if one is present.
    pub fn from(&self) -> Option<Header> {
        for h in &self.0 {
            if let Header::From(a) = h {
                return Some(Header::From(a.clone()));
            }
        }
        None
    }

    /// Return the To header if one is present.
    pub fn to(&self) -> Option<Header> {
        for h in &self.0 {
            if let Header::To(a) = h {
                return Some(Header::To(a.clone()));
            }
        }
        None
    }

    /// Return the Contact header if one is present.
    pub fn contact(&self) -> Option<Header> {
        for h in &self.0 {
            if let Header::Contact(a) = h {
                return Some(Header::Contact(a.clone()));
            }
        }
        None
    }

    /// Return the Contact header if one is present.
    pub fn call_id(&self) -> Option<Header> {
        for h in &self.0 {
            if let Header::CallId(a) = h {
                return Some(Header::CallId(a.clone()));
            }
        }
        None
    }

    /// Return the Contact header if one is present.
    pub fn via(&self) -> Option<Header> {
        for h in &self.0 {
            if let Header::Via(a) = h {
                return Some(Header::Via(a.clone()));
            }
        }
        None
    }

    /// Return the Contact header if one is present.
    pub fn xfs_sending_message(&self) -> Option<Header> {
        for h in &self.0 {
            if let Header::XFsSendingMessage(a) = h {
                return Some(Header::XFsSendingMessage(a.clone()));
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Header {
    To(NamedHeader),
    Contact(NamedHeader),
    From(NamedHeader),
    ReplyTo(NamedHeader),
    CSeq(u32, Method),
    MaxForwards(u32),
    Expires(u32),
    Accept(Vec<Method>),
    ContentLength(u32),
    Allow(Vec<Method>),
    UserAgent(String),
    CallId(String),
    ContentType(ContentType),
    ContentLanguage(Language),
    ContentEncoding(ContentType),
    AcceptLanguage(Language),
    AcceptEncoding(ContentType),
    AlertInfo(String),
    ErrorInfo(String),
    AuthenticationInfo(String),
    Authorization(auth::AuthHeader),
    CallInfo(String),
    InReplyTo(String),
    ContentDisposition(String),
    Date(String),
    MinExpires(u32),
    MimeVersion(f32),
    Organization(String),
    ProxyAuthenticate(String),
    ProxyAuthorization(String),
    ProxyRequire(String),
    Require(String),
    RetryAfter(String),
    Route(String),
    Subject(String),
    RecordRoute(String),
    Server(String),
    Supported(Vec<String>),
    Timestamp(u32),
    Unsupported(String),
    Warning(String),
    Via(via::ViaHeader),
    Priority(String),
    WwwAuthenticate(auth::AuthHeader),
    XFsSendingMessage(String),
    Other(String, String)
}
