use nom::character::*;

use super::*;
use crate::parse::*;
use super::named::*;
use super::content::*;
use super::language::*;
use crate::core::parse_method;


named!(pub parse_header<Header>, alt!(
    parse_accept_encoding_header |
    parse_accept_header |
    parse_accept_language_header |
    parse_alert_info_header |
    parse_allow_header |
    parse_authentication_info_header |
    parse_authorization_header |
    parse_call_info_header |
    parse_callid_header |
    parse_contact_header |
    parse_content_disposition_header |
    parse_content_encoding_header |
    parse_content_language_header |
    parse_content_length_header |
    parse_content_type_header |
    parse_cseq_header |
    parse_date_header |
    parse_error_info_header |
    parse_expires_header |
    parse_from_header |
    parse_in_reply_to_header |
    parse_max_forwards_header |
    parse_mime_version_header |
    parse_min_expires_header |
    parse_organization_header |
    parse_priority_header |
    parse_proxy_authenticate_header |
    parse_proxy_authorization_header |
    parse_proxy_require_header |
    parse_record_route_header |
    parse_reply_to_header |
    parse_require_header |
    parse_retry_after_header |
    parse_route_header |
    parse_server_header |
    parse_subject_header |
    parse_supported_header |
    parse_timestamp_header |
    parse_to_header |
    parse_unsupported_header |
    parse_useragent_header |
    parse_via_header |
    parse_warning_header |
    parse_www_authenticate_header
));

macro_rules! impl_u32_parser {
    ($name:tt, $tag:tt, $variant: ident) => {
        named!(pub $name<Header>, do_parse!(
            tag!($tag) >>
            opt!(take_while!(is_space)) >>
            char!(':') >>
            opt!(take_while!(is_space)) >>
            value: map_res!(take_while!(is_digit), parse_u32) >>
            tag!("\r\n") >>
            (Header::$variant(value))
        ));
    }
}
macro_rules! impl_f32_parser {
    ($name:tt, $tag:tt, $variant: ident) => {
        named!(pub $name<Header>, do_parse!(
            tag!($tag) >>
            opt!(take_while!(is_space)) >>
            char!(':') >>
            opt!(take_while!(is_space)) >>
            value: map_res!(take_while!(|item| is_digit(item) || item == '.' as u8), parse_f32) >>
            (Header::$variant(value))
        ));
    }
}

macro_rules! impl_string_parser {
    ($name:tt, $tag:tt, $variant: ident) => {
        named!(pub $name<Header>, do_parse!(
            tag!($tag) >>
            opt!(take_while!(is_space)) >>
            char!(':') >>
            opt!(take_while!(is_space)) >>
            value: map_res!(take_until!("\r"), slice_to_string) >>
            tag!("\r\n") >>
            (Header::$variant(value))
        ));
    }
}

macro_rules! impl_array_parser {
    ($name:tt, $tag:tt, $variant:ident, $func:ident) => {
        named!(pub $name<Header>, do_parse!(
            tag!($tag) >>
            opt!(take_while!(is_space)) >>
            char!(':') >>
            opt!(take_while!(is_space)) >>
            data: separated_list!(pair!(char!(','), opt!(char!(' '))), $func) >>
            tag!("\r\n") >>
            (Header::$variant(data))
        ));
    }
}

macro_rules! impl_named_parser {
    ($name:tt, $tag:tt, $variant:ident) => {
        named!(pub $name<Header>, do_parse!(
            tag!($tag) >>
            opt!(take_while!(is_space)) >>
            char!(':') >>
            opt!(take_while!(is_space)) >>
            out: parse_named_field_value >>
            params: parse_named_field_params >>
            tag!("\r\n") >>
            (Header::$variant(NamedHeader { display_name: out.0, uri: out.1, params: params }))
        ));
    }
}

macro_rules! impl_type_parser {
    ($name:tt, $tag:tt, $variant:ident) => {
        named!(pub $name<Header>, do_parse!(
            tag!($tag) >>
            opt!(take_while!(is_space)) >>
            char!(':') >>
            opt!(take_while!(is_space)) >>
            ty: parse_content_type >>
            (Header::$variant(ty))
        ));
    }
}

macro_rules! impl_lang_parser {
    ($name:tt, $tag:tt, $variant:ident) => {
        named!(pub $name<Header>, do_parse!(
            tag!($tag) >>
            opt!(take_while!(is_space)) >>
            char!(':') >>
            opt!(take_while!(is_space)) >>
            ty: parse_language >>
            (Header::$variant(ty))
        ));
    }
}

impl_u32_parser!(parse_expires_header, "Expires", Expires);
impl_u32_parser!(parse_min_expires_header, "Min-Expires", MinExpires);
impl_u32_parser!(parse_content_length_header, "Content-Length", ContentLength);
impl_u32_parser!(parse_max_forwards_header, "Max-Forwards", MaxForwards);
impl_f32_parser!(parse_mime_version_header, "MIME-Version", MimeVersion);
impl_string_parser!(parse_useragent_header, "User-Agent", UserAgent);
impl_string_parser!(parse_callid_header, "Call-ID", CallId);
impl_string_parser!(parse_alert_info_header, "Alert-Info", AlertInfo);
impl_string_parser!(parse_error_info_header, "Error-Info", ErrorInfo);
impl_string_parser!(parse_authentication_info_header, "Authentication-Info", AuthenticationInfo);
impl_string_parser!(parse_authorization_header, "Authorization", Authorization);
impl_string_parser!(parse_call_info_header, "Call-Info", CallInfo);
impl_string_parser!(parse_in_reply_to_header, "In-Reply-To", InReplyTo);
impl_string_parser!(parse_content_disposition_header, "Content-Disposition", ContentDisposition);
impl_string_parser!(parse_date_header, "Date", Date);
impl_string_parser!(parse_organization_header, "Organization", Organization);
impl_string_parser!(parse_proxy_authenticate_header, "Proxy-Authenticate", ProxyAuthenticate);
impl_string_parser!(parse_proxy_authorization_header, "Proxy-Authorization", ProxyAuthorization);
impl_string_parser!(parse_proxy_require_header, "Proxy-Require", ProxyRequire);
impl_string_parser!(parse_require_header, "Require", Require);
impl_string_parser!(parse_retry_after_header, "Retry-After", RetryAfter);
impl_string_parser!(parse_route_header, "Route", Route);
impl_string_parser!(parse_subject_header, "Subject", Subject);
impl_string_parser!(parse_record_route_header, "Record-Route", RecordRoute);
impl_string_parser!(parse_server_header, "Server", Server);
impl_string_parser!(parse_unsupported_header, "Unsupported", Unsupported);
impl_string_parser!(parse_warning_header, "Warning", Warning);
impl_string_parser!(parse_via_header, "Via", Via);
impl_string_parser!(parse_priority_header, "Priority", Priority);
impl_string_parser!(parse_www_authenticate_header, "WWW-Authenticate", WwwAuthenticate);
impl_u32_parser!(parse_timestamp_header, "Timestamp", Timestamp);
impl_array_parser!(parse_accept_header, "Accept", Accept, parse_method);
impl_array_parser!(parse_allow_header, "Allow", Allow, parse_method);
impl_array_parser!(parse_supported_header, "Supported", Supported, parse_string);
impl_named_parser!(parse_to_header, "To", To);
impl_named_parser!(parse_from_header, "From", From);
impl_named_parser!(parse_contact_header, "Contact", Contact);
impl_named_parser!(parse_reply_to_header, "Reply-To", ReplyTo);
impl_type_parser!(parse_content_type_header, "Content-Type", ContentType);
impl_type_parser!(parse_content_encoding_header, "Content-Encoding", ContentEncoding);
impl_type_parser!(parse_accept_encoding_header, "Accept-Encoding", AcceptEncoding);
impl_lang_parser!(parse_content_language_header, "Content-Language", ContentLanguage);
impl_lang_parser!(parse_accept_language_header, "Accept-Language", AcceptLanguage);

named!(pub parse_cseq_header<Header>, do_parse!(
    tag!("CSeq") >>
    opt!(take_while!(is_space)) >>
    char!(':') >>
    opt!(take_while!(is_space)) >>
    value: map_res!(take_while!(is_digit), parse_u32) >>
    opt!(take_while!(is_space)) >>
    method: parse_method >>
    tag!("\r\n") >>
    (Header::CSeq(value, method))
));
