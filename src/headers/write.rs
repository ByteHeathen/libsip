use std::fmt;

use crate::core::Method;
use super::Header;

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Header::To(value) => write_simple_field("To", value, f),
            Header::From(value) => write_simple_field("From", value, f),
            Header::Contact(value) => write_simple_field("Contact", value, f),
            Header::ReplyTo(value) => write_simple_field("Reply-To", value, f),
            Header::CSeq(num, method) => write!(f, "CSeq: {} {}", num, method),
            Header::MaxForwards(num) => write!(f, "Max-Forwards: {}", num),
            Header::Expires(num) => write!(f, "Expires: {}", num),
            Header::Accept(methods) => write_method_array_header("Accept", f, methods),
            Header::Allow(methods) => write_method_array_header("Allow", f, methods),
            Header::ContentEncoding(ty) => write_simple_field("Content-Encoding", ty, f),
            Header::ContentLength(len) => write_simple_field("Content-Length", len, f),
            Header::ContentType(ty) => write_simple_field("Content-Type", ty, f),
            Header::UserAgent(agent) => write_simple_field("User-Agent", agent, f),
            Header::CallId(call_id) => write_simple_field("Call-ID", call_id, f),
            Header::ContentLanguage(lang) => write_simple_field("Content-Language", lang, f),
            Header::AcceptLanguage(lang) => write_simple_field("Accept-Language", lang, f),
            Header::AcceptEncoding(ty) => write_simple_field("Accept-Encoding", ty, f),
            Header::AlertInfo(data) => write_simple_field("Alert-Info", data, f),
            Header::ErrorInfo(data) => write_simple_field("Error-Info", data, f),
            Header::AuthenticationInfo(data) => write_simple_field("Authentication-Info", data, f),
            Header::Authorization(data) => write_auth_header("Authorization", data, f),
            Header::CallInfo(data) => write_simple_field("Call-Info", data, f),
            Header::InReplyTo(data) => write_simple_field("In-Reply-To", data, f),
            Header::ContentDisposition(data) => write_simple_field("Content-Disposition", data, f),
            Header::Date(string) => write_simple_field("Date", string, f),
            Header::MinExpires(exp) => write_simple_field("Min-Expires", exp, f),
            Header::MimeVersion(exp) => write_simple_field("MIME-Version", exp, f),
            Header::Organization(org) => write_simple_field("Organization", org, f),
            Header::ProxyAuthenticate(data) => write_simple_field("Proxy-Authenticate", data, f),
            Header::ProxyAuthorization(data) => write_simple_field("Proxy-Authorization", data, f),
            Header::ProxyRequire(data) => write_simple_field("Proxy-Require", data, f),
            Header::Require(data) => write_simple_field("Require", data, f),
            Header::RetryAfter(data) => write_simple_field("Retry-After", data, f),
            Header::Route(data) => write_simple_field("Route", data, f),
            Header::Subject(data) => write_simple_field("Subject", data, f),
            Header::RecordRoute(data) => write_simple_field("Record-Route", data, f),
            Header::Server(data) => write_simple_field("Server", data, f),
            Header::Supported(data) => write_string_array_header("Supported", f, data),
            Header::Timestamp(data) => write_simple_field("Timestamp", data, f),
            Header::Unsupported(data) => write_simple_field("Unsupported", data, f),
            Header::Warning(data) => write_simple_field("Warning", data, f),
            Header::Via(data) => write!(f, "{}", data),
            Header::Priority(data) => write_simple_field("Priority", data, f),
            Header::WwwAuthenticate(data) => write_auth_header("WWW-Authenticate", data, f),
            Header::XFsSendingMessage(data) => write_simple_field("X-FS-Sending-Message", data, f),
            Header::Other(key, value) => write!(f, "{}: {}", key, value)
        }
    }
}


macro_rules! write_array_header {
    ($name:ident, $item: ident) => {
        fn $name(name: &str, f: &mut fmt::Formatter, v: &[$item]) -> fmt::Result {
            write!(f, "{}: ", name)?;
            for (index, item) in v.iter().enumerate() {
                if index == 0 {
                    write!(f, "{}", item)?;
                } else {
                    write!(f, ",{}", item)?;
                }
            }
            Ok(())
        }
    }
}

write_array_header!(write_method_array_header, Method);
write_array_header!(write_string_array_header, String);
fn write_simple_field<D: fmt::Display>(header: &str, data: D, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", header, data)
}
fn write_auth_header<D: fmt::Display>(header: &str, data: D, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", header, data)
}
