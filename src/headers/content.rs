use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ContentType {
    Sdp
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContentType::Sdp => write!(f, "application/sdp")
        }
    }
}

named!(pub parse_content_type<ContentType>, alt!(
    map!(tag!("application/sdp"), |_| ContentType::Sdp)
));
