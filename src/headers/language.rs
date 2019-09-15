use std::fmt;

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
