use nom::character::is_alphanumeric;

use crate::parse::slice_to_string;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct UriAuth {
    username: String,
    password: Option<String>
}

impl UriAuth {

    pub fn new<S: Into<String>>(username: S) -> UriAuth {
        UriAuth { username: username.into(), password: None }
    }

    pub fn password<S: Into<String>>(mut self, p: S) -> UriAuth {
        self.password = Some(p.into());
        self
    }
}

impl fmt::Display for UriAuth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(pass) = &self.password {
            write!(f, "{}:{}", &self.username, pass)?;
        } else {
            write!(f, "{}", &self.username)?;
        }
        Ok(())
    }
}

named!(pub parse_uriauth<UriAuth>, do_parse!(
    username: map_res!(take_while!(is_alphanumeric), slice_to_string) >>
    password: opt!(parse_password) >>
    char!('@') >>
    (UriAuth { username, password })
));

named!(parse_password<String>, do_parse!(
    char!(':') >>
    password: map_res!(take_while!(is_alphanumeric), slice_to_string) >>
    (password)
));
