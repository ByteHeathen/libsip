use std::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Schema {
    Digest
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Schema::Digest => write!(f, "Digest")
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AuthHeader(pub Schema, pub HashMap<String, String>);

impl fmt::Display for AuthHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WWW-Authenticate: {}", self.0)?;
        for (index, (key, value)) in self.1.iter().enumerate() {
            if index == 0 {
                write!(f, " {}=\"{}\"", key, value)?;
            } else {
                write!(f, ", {}=\"{}\"", key, value)?;
            }
        }
        Ok(())
    }
}
