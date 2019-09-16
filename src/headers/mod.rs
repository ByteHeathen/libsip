mod write;
pub mod parse;
pub use self::parse::parse_header;

mod content;
pub use self::content::ContentType;

mod language;
pub use self::language::Language;

use crate::uri::Uri;
use crate::core::Method;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Header {
    To(Option<String>, Uri, HashMap<String, String>),
    From(Option<String>, Uri, HashMap<String, String>),
    Contact(Option<String>, Uri, HashMap<String, String>),
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
    Authorization(String),
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
    ReplyTo(Option<String>, Uri, HashMap<String, String>),
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
    Via(String),
    Priority(String),
    WwwAuthenticate(String)
}
