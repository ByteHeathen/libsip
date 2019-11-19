use serde::{Deserialize, Serialize};

use std::fmt;

/// Sip URI Schema.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Schema {
    Sip,
    Sips,
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Schema::Sip => write!(f, "sip"),
            Schema::Sips => write!(f, "sips"),
        }
    }
}

named!(pub parse_schema<Schema>, alt!(
    map!(tag_no_case!("sip"), |_| Schema::Sip) |
    map!(tag_no_case!("sips"), |_| Schema::Sips)
));
