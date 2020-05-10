use nom::character::*;
use nom::{
    IResult,
    error::ParseError,
    combinator::{
        map, opt,
        map_res,
    },
    sequence::{
        pair
    },
    multi::{
        separated_list0
    },
    character::complete::{
        char
    },
    bytes::complete::{
        tag_no_case, take_while,
        tag, take_until
    }
};
use super::{content::*, language::*, named::*, *};
use crate::{
    core::{parse_method, parse_transport, parse_version},
    parse::*,
    uri::parse_uri,
};

use std::collections::HashMap;

pub fn parse_header<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Header> {
    let (input, _) = opt(tag("\r\n"))(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, header) = _parse_header(input)?;
    Ok((input, header))
}

named!(pub _parse_header<Header>, alt!(
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
    parse_xfs_sending_message_header |
    parse_www_authenticate_header |
    parse_other_header
));

macro_rules! impl_u32_parser {
    ($name:tt, $tag:tt, $variant: ident) => {
        pub fn $name<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Header, E> {
            let (input, _) = tag($tag)(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, _) = char(':')(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, value) = map_res(take_while(is_digit), parse_u32)(input)?;
            let (input, _) = tag("\r\n")(input)?;
            Ok((input, Header::$variant(value)))
        }
    }
}
macro_rules! impl_f32_parser {
    ($name:tt, $tag:tt, $variant: ident) => {
        pub fn $name<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Header, E> {
            let (input, _) = tag_no_case($tag)(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, _) = char(':')(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, value) = map_res(take_while(|item| is_digit(item) || item == b'.'), parse_f32)(input)?;
            Ok((input, Header::$variant(value)))
        }
    }
}

macro_rules! impl_string_parser {
    ($name:tt, $tag:tt, $variant: ident) => {
        pub fn $name<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Header, E> {
            let (input, _) = tag_no_case::<_, _, E>($tag)(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, _) = char(':')(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, value) = map_res(take_until("\r"), slice_to_string::<E>)(input)?;
            let (input, _) = tag("\r\n")(input)?;
            Ok((input, Header::$variant(value)))
        }
    }
}

macro_rules! impl_array_parser {
    ($name:tt, $tag:tt, $variant:ident, $func:ident) => {
        pub fn $name<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Header, E> {
            let (input, _) = tag_no_case($tag)(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, _) = char(':')(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, data) = separated_list0(pair(char(','), opt(char(' '))), $func)(input)?;
            let (input, _) = tag("\r\n")(input)?;
            Ok((input, Header::$variant(data)))
        }
    }
}

macro_rules! impl_named_parser {
    ($name:tt, $tag:tt, $variant:ident) => {
        pub fn $name<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Header, E> {
            let (input, _) = tag_no_case($tag)(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, _) = char(':')(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, out) = parse_named_field_value(input)?;
            let (input, params) = parse_named_field_params(input)?;
            let (input, _) = tag("\r\n")(input)?;
            Ok((input, Header::$variant(NamedHeader { display_name: out.0, uri: out.1, params: params })))
        }
    }
}

macro_rules! impl_type_parser {
    ($name:tt, $tag:tt, $variant:ident) => {
        pub fn $name<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Header, E> {
            let (input, _) = tag_no_case($tag)(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, _) = char(':')(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, ty) = parse_content_type::<E>(input)?;
            Ok((input, Header::$variant(ty)))
        }
    }
}

macro_rules! impl_lang_parser {
    ($name:tt, $tag:tt, $variant:ident) => {
        pub fn $name<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Header, E> {
            let (input, _) = tag_no_case($tag)(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, _) = char(':')(input)?;
            let (input, _) = opt(take_while(is_space))(input)?;
            let (input, ty) = parse_language(input).map_err(|_| nom::Err::Failure(
                E::from_error_kind(input, nom::error::ErrorKind::IsNot)
              )
            )?;
            Ok((input, Header::$variant(ty)))
        }
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
impl_string_parser!(
    parse_authentication_info_header,
    "Authentication-Info",
    AuthenticationInfo
);
impl_string_parser!(parse_call_info_header, "Call-Info", CallInfo);
impl_string_parser!(parse_in_reply_to_header, "In-Reply-To", InReplyTo);
impl_string_parser!(
    parse_content_disposition_header,
    "Content-Disposition",
    ContentDisposition
);
impl_string_parser!(parse_date_header, "Date", Date);
impl_string_parser!(parse_organization_header, "Organization", Organization);
impl_string_parser!(
    parse_proxy_authenticate_header,
    "Proxy-Authenticate",
    ProxyAuthenticate
);
impl_string_parser!(
    parse_proxy_authorization_header,
    "Proxy-Authorization",
    ProxyAuthorization
);
impl_string_parser!(parse_proxy_require_header, "Proxy-Require", ProxyRequire);
impl_string_parser!(parse_require_header, "Require", Require);
impl_string_parser!(parse_retry_after_header, "Retry-After", RetryAfter);
impl_string_parser!(parse_route_header, "Route", Route);
impl_string_parser!(parse_subject_header, "Subject", Subject);
impl_string_parser!(parse_record_route_header, "Record-Route", RecordRoute);
impl_string_parser!(parse_server_header, "Server", Server);
impl_string_parser!(parse_unsupported_header, "Unsupported", Unsupported);
impl_string_parser!(parse_warning_header, "Warning", Warning);
impl_string_parser!(
    parse_xfs_sending_message_header,
    "X-FS-Sending-Message",
    XFsSendingMessage
);
impl_string_parser!(parse_priority_header, "Priority", Priority);
impl_u32_parser!(parse_timestamp_header, "Timestamp", Timestamp);
impl_array_parser!(parse_accept_header, "Accept", Accept, parse_method);
impl_array_parser!(parse_allow_header, "Allow", Allow, parse_method);
impl_array_parser!(parse_supported_header, "Supported", Supported, parse_string);
impl_named_parser!(parse_to_header, "To", To);
impl_named_parser!(parse_from_header, "From", From);
impl_named_parser!(parse_contact_header, "Contact", Contact);
impl_named_parser!(parse_reply_to_header, "Reply-To", ReplyTo);
impl_type_parser!(parse_content_type_header, "Content-Type", ContentType);
impl_type_parser!(
    parse_content_encoding_header,
    "Content-Encoding",
    ContentEncoding
);
impl_type_parser!(
    parse_accept_encoding_header,
    "Accept-Encoding",
    AcceptEncoding
);
impl_lang_parser!(
    parse_content_language_header,
    "Content-Language",
    ContentLanguage
);
impl_lang_parser!(
    parse_accept_language_header,
    "Accept-Language",
    AcceptLanguage
);

fn parse_auth_header_vars<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> ParserResult<HashMap<String, String>, E> {
    let mut map = HashMap::new();
    let mut data = input;
    while let Ok((remains, (key, value))) = parse_key_value_pair::<E>(data) {
        map.insert(key, value);
        data = remains;
    }
    Ok((data, map))
}

pub fn parse_other_header<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Header, E> {
    let (input, _) = opt(tag("\r\n"))(input)?;
    let (input, key) = map_res(take_while(|item| is_alphanumeric(item) || item == b'-'), slice_to_string::<E>)(input)?;
    let (input, _) = char(':')(input)?;
    let (input, value) = map_res(take_until("\r"), slice_to_string::<E>)(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Header::Other(key, value)))
}

pub fn parse_cseq_header<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Header, 
E> {
    let (input, _) = opt(tag("\r\n"))(input)?;
    let (input, _) = tag_no_case("CSeq")(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, value) = map_res(take_while(is_digit), parse_u32)(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, method) = parse_method(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Header::CSeq(value, method)))
}

pub fn parse_via_header<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Header, E> {
    let (input, _) = tag_no_case("Via")(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, version) = parse_version(input)?;
    let (input, _) = char('/')(input)?;
    let (input, transport) = parse_transport(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, uri) = parse_uri(input)?;
    Ok((input, Header::Via(via::ViaHeader { version, transport, uri })))
}

pub fn parse_www_authenticate_header<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Header, E> {
    let (input, _) = opt(tag("\r\n"))(input)?;
    let (input, _) = tag_no_case("WWW-Authenticate")(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, schema) = parse_auth_schema::<E>(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, res) = parse_auth_header_vars(input)?;
    let (input, _) = opt(char(' '))(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Header::WwwAuthenticate(auth::AuthHeader(schema, res))))
}

pub fn parse_authorization_header<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Header, E> {
    let (input, _) = opt(tag("\r\n"))(input)?;
    let (input, _) = tag_no_case("Authorization")(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, schema) = parse_auth_schema(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, res) = parse_auth_header_vars(input)?;
    let (input, _) = opt(char(' '))(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Header::Authorization(auth::AuthHeader(schema, res))))
}

pub fn parse_key_value_pair<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], (String, String), E> {
    let (input, _) = opt(char(','))(input)?;
    let (input, _) = opt(char(' '))(input)?;
    let (input, key) = map_res(take_while(is_alphanumeric), slice_to_string::<E>)(input)?;
    let (input, _) = opt(char('='))(input)?;
    let (input, value) = parse_possibly_quoted_string(input)?;
    Ok((input, (key, value)))
}

pub fn parse_auth_schema<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], auth::Schema, E> {
    Ok(map(tag_no_case("Digest"), |_| auth::Schema::Digest)(input)?)
}
