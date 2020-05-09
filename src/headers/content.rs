use std::fmt;

/// Sip protocol Content-Type value.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ContentType {
    Csv,
    Sdp,
    Xml,
    Html,
    VCard,
    Calendar,
    MarkDown,
    MsWord,
    Pdf,
    Png,
    PlainText,
    Zip,
    GZip,
    Sql,
    Json,
    Javascript,
    Css,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContentType::Sdp => write!(f, "application/sdp"),
            ContentType::PlainText => write!(f, "text/plain"),
            ContentType::Xml => write!(f, "application/xml"),
            ContentType::Html => write!(f, "text/html"),
            ContentType::VCard => write!(f, "text/vcard"),
            ContentType::MarkDown => write!(f, "text/markdown"),
            ContentType::Calendar => write!(f, "text/calendar"),
            ContentType::MsWord => write!(f, "applciation/msword"),
            ContentType::Pdf => write!(f, "application/pdf"),
            ContentType::Png => write!(f, "image/png"),
            ContentType::Csv => write!(f, "text/csv"),
            ContentType::Zip => write!(f, "application/zip"),
            ContentType::Sql => write!(f, "application/sql"),
            ContentType::Json => write!(f, "application/json"),
            ContentType::GZip => write!(f, "application/gzip"),
            ContentType::Javascript => write!(f, "application/javascript"),
            ContentType::Css => write!(f, "application/css"),
        }
    }
}

use nom::{
    IResult,
    branch::alt,
    combinator::map,
    bytes::complete::tag_no_case
};

pub fn parse_content_type(input: &[u8]) -> IResult<&[u8], ContentType> {
    Ok(alt((
      map(tag_no_case("application/sdp"), |_| ContentType::Sdp),
      map(tag_no_case("text/plain"), |_| ContentType::PlainText),
      map(tag_no_case("text/html"), |_| ContentType::Html),
      map(tag_no_case("applications/xml"), |_| ContentType::Xml),
      map(tag_no_case("text/vcard"), |_| ContentType::VCard),
      map(tag_no_case("text/calendar"), |_| ContentType::Calendar),
      map(tag_no_case("text/markdown"), |_| ContentType::MarkDown),
      map(tag_no_case("application/msword"), |_| ContentType::MsWord),
      map(tag_no_case("application/pdf"), |_| ContentType::Pdf),
      map(tag_no_case("image/png"), |_| ContentType::Png),
      map(tag_no_case("text/csv"), |_| ContentType::Csv),
      map(tag_no_case("application/zip"), |_| ContentType::Zip),
      map(tag_no_case("application/sql"), |_| ContentType::Sql),
      map(tag_no_case("application/json"), |_| ContentType::Json),
      map(tag_no_case("application/gzip"), |_| ContentType::GZip),
      map(tag_no_case("application/javascript"), |_| ContentType::Javascript),
      map(tag_no_case("application/css"), |_| ContentType::Css)
    ))(input)?)
}
