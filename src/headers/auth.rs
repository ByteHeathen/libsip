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
        write!(f, "{}", self.0)?;
        for (index, (key, value)) in self.1.iter().enumerate() {
            if index == 0 {
                if key == &"qop".to_string() {
                    write!(f, "{}={}", key, value)?;
                } else if key == &"auth".to_string() {
                    write!(f, "{}={:08}", key, value)?;
                } else {
                    write!(f, " {}=\"{}\"", key, value)?;
                }
            } else {
                if key == &"qop".to_string() {
                    write!(f, ", {}={}", key, value)?;
                } else if key == &"auth".to_string() {
                    write!(f, ", {}={:08}", key, value)?;
                } else {
                    write!(f, ", {}=\"{}\"", key, value)?;
                }
            }
        }
        Ok(())
    }
}
