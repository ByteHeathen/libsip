mod write;
pub mod parse;
pub use self::parse::parse_header;

mod content;
pub use self::content::ContentType;

mod language;
pub use self::language::Language;

use crate::uri::Uri;
use crate::core::Method;

#[derive(Debug, PartialEq, Clone)]
pub enum Header {
    To(Option<String>, Uri),
    From(Option<String>, Uri),
    Contact(Option<String>, Uri),
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
    ReplyTo(Option<String>, Uri),
    Require(String),
    RetryAfter(String),
    Route(String),
    Subject(String),
    RecordRoute(String),
    Server(String),
    Supported(String),
    Timestamp(u32),
    Unsupported(String),
    Warning(String),
    Via(String),
    Priority(String),
    WwwAuthenticate(String)
}
