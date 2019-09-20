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
    Other(String, String)
}
