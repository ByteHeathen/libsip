use std::fmt;

/// Sip Protocol languages.
///
/// TODO: Finish this enum
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Language {
    English
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::English => write!(f, "en")
        }
    }
}

named!(pub parse_language<Language>, alt!(
    map!(tag!("en"), |_| Language::English)
));
