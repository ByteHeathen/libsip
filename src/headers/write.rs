use std::fmt;

use crate::core::Method;
use crate::uri::Uri;
use super::Header;

use std::collections::HashMap;

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Header::To(name, uri, params) => write_named_header("To", name, uri, params, f),
            Header::From(name, uri, params) => write_named_header("From", name, uri, params, f),
            Header::Contact(name, uri, params) => write_named_header("Contact", name, uri, params, f),
            Header::ReplyTo(name, uri, params) => write_named_header("Reply-To", name, uri, params, f),
            Header::CSeq(num, method) => write!(f, "CSeq: {} {}", num, method),
            Header::MaxForwards(num) => write!(f, "Max-Forwards: {}", num),
            Header::Expires(num) => write!(f, "Expires: {}", num),
            Header::Accept(methods) => write_method_array_header("Accept", f, methods),
            Header::Allow(methods) => write_method_array_header("Allow", f, methods),
            Header::ContentEncoding(ty) => write!(f, "Content-Encoding: {}", ty),
            Header::ContentLength(len) => write!(f, "Content-Length: {}", len),
            Header::ContentType(ty) => write!(f, "Content-Type: {}", ty),
            Header::UserAgent(agent) => write!(f, "User-Agent: {}", agent),
            Header::CallId(call_id) => write!(f, "Call-ID: {}", call_id),
            Header::ContentLanguage(lang) => write!(f, "Content-Language: {}", lang),
            Header::AcceptLanguage(lang) => write!(f, "Accept-Language: {}", lang),
            Header::AcceptEncoding(ty) => write!(f, "Accept-Encoding: {}", ty),
            Header::AlertInfo(data) => write!(f, "Alert-Info: {}", data),
            Header::ErrorInfo(data) => write!(f, "Error-Info: {}", data),
            Header::AuthenticationInfo(data) => write!(f, "Authentication-Info: {}", data),
            Header::Authorization(data) => write!(f, "Authorization: {}", data),
            Header::CallInfo(data) => write!(f, "Call-Info: {}", data),
            Header::InReplyTo(data) => write!(f, "In-Reply-To: {}", data),
            Header::ContentDisposition(data) => write!(f, "Content-Disposition: {}", data),
            Header::Date(string) => write!(f, "Date: {}", string),
            Header::MinExpires(exp) => write!(f, "Min-Expires: {}", exp),
            Header::MimeVersion(exp) => write!(f, "MIME-Version: {}", exp),
            Header::Organization(org) => write!(f, "Organization: {}", org),
            Header::ProxyAuthenticate(data) => write!(f, "Proxy-Authenticate: {}", data),
            Header::ProxyAuthorization(data) => write!(f, "Proxy-Authorization: {}", data),
            Header::ProxyRequire(data) => write!(f, "Proxy-Require: {}", data),
            Header::Require(data) => write!(f, "Require: {}", data),
            Header::RetryAfter(data) => write!(f, "Retry-After: {}", data),
            Header::Route(data) => write!(f, "Route: {}", data),
            Header::Subject(data) => write!(f, "Subject: {}", data),
            Header::RecordRoute(data) => write!(f, "Record-Route: {}", data),
            Header::Server(data) => write!(f, "Server: {}", data),
            Header::Supported(data) => write_string_array_header("Supported", f, data),
            Header::Timestamp(data) => write!(f, "Timestamp: {}", data),
            Header::Unsupported(data) => write!(f, "Unsupported: {}", data),
            Header::Warning(data) => write!(f, "Warning: {}", data),
            Header::Via(data) => write!(f, "Via: {}", data),
            Header::Priority(data) => write!(f, "Priority: {}", data),
            Header::WwwAuthenticate(data) => write!(f, "WWW-Authenticate: {}", data)
        }
    }
}

fn write_named_header(header: &str, name: &Option<String>, uri: &Uri, params: &HashMap<String, String>, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: ", header)?;
    if let Some(name) = name {
        if name.contains(' ') {
            write!(f, "\"{}\" <{}>", name, uri)?;
        } else {
            write!(f, "{} <{}>", name, uri)?;
        }
    } else {
        write!(f, "<{}>", uri)?;
    }
    for (key, value) in params.iter() {
        write!(f, ";{}={}", key, value)?;
    }
    Ok(())
}

fn write_method_array_header(name: &str, f: &mut fmt::Formatter, v: &Vec<Method>) -> fmt::Result {
    write!(f, "{}: ", name)?;
    for (index, method) in v.iter().enumerate() {
        if index == 0 {
            write!(f, "{}", method)?;
        } else {
            write!(f, ",{}", method)?;
        }
    }
    Ok(())
}

fn write_string_array_header(name: &str, f: &mut fmt::Formatter, v: &Vec<String>) -> fmt::Result {
    write!(f, "{}: ", name)?;
    for (index, method) in v.iter().enumerate() {
        if index == 0 {
            write!(f, "{}", method)?;
        } else {
            write!(f, ",{}", method)?;
        }
    }
    Ok(())
}
