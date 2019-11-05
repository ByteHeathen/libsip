use std::fmt;

/// Sip protocol Content-Type value.
///
/// TODO: Finish this enum.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ContentType {
    Sdp,
    PlainText
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContentType::Sdp => write!(f, "application/sdp"),
            ContentType::PlainText => write!(f, "text/plain")
        }
    }
}

named!(pub parse_content_type<ContentType>, alt!(
    map!(tag!("application/sdp"), |_| ContentType::Sdp) |
    map!(tag!("text/plain"), |_| ContentType::PlainText)
));
