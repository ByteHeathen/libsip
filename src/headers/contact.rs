use nom::{error::ParseError, IResult};

use crate::{headers::parse::parse_generic_param_with_possibly_quoted_value, Uri};

use std::{
    collections::{hash_map::Entry, HashMap},
    fmt,
    str::FromStr,
};

/// Value used in the Contact Header.
#[derive(Debug, PartialEq, Clone)]
pub struct ContactHeader {
    pub display_name: Option<String>,
    pub uri: Uri,
    pub parameters: HashMap<String, Option<GenValue>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GenValue {
    Token(String),
    QuotedString(String),
}

impl From<&str> for GenValue {
    fn from(string: &str) -> Self {
        if string.starts_with("\"") && string.ends_with("\"") {
            Self::QuotedString(string[1..=(string.len() - 2)].into())
        } else {
            Self::Token(string.into())
        }
    }
}

impl From<String> for GenValue {
    fn from(mut string: String) -> Self {
        if string.starts_with("\"") && string.ends_with("\"") {
            Self::QuotedString(string.drain(1..=(string.len() - 2)).collect())
        } else {
            Self::Token(string)
        }
    }
}

impl Into<String> for GenValue {
    fn into(self) -> String {
        match self {
            Self::Token(token) => token,
            Self::QuotedString(quoted_string) => format!("\"{}\"", quoted_string),
        }
    }
}

impl GenValue {
    pub fn parse<F: FromStr>(&self) -> Result<F, F::Err> {
        match self {
            Self::Token(inner) => FromStr::from_str(inner),
            Self::QuotedString(inner) => FromStr::from_str(inner),
        }
    }
}

impl ContactHeader {
    pub fn new(uri: Uri) -> ContactHeader {
        ContactHeader {
            display_name: None,
            parameters: HashMap::new(),
            uri,
        }
    }

    /// Sets `display_name` of this header
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.display_name = Some(name.into());
        self
    }

    /// Adds a parameter with a given name and a given value to `params`.
    ///
    /// If there is already a parameter with a given name, its value is changed because [RFC3261: Page 31, Header Field Format](https://tools.ietf.org/html/rfc3261#page-31) defines that "any given parameter-name MUST NOT appear more than once"
    pub fn param<N, V>(mut self, name: N, value: Option<V>) -> Self
    where
        N: Into<String>,
        V: Into<GenValue>,
    {
        self.set_param(name, value);
        self
    }

    /// Adds a parameter with a given name and a given value to `params`.
    ///
    /// If there is already a parameter with a given name, its value is changed because [RFC3261: Page 31, Header Field Format](https://tools.ietf.org/html/rfc3261#page-31) defines that "any given parameter-name MUST NOT appear more than once"
    pub fn set_param<N, V>(&mut self, name: N, value: Option<V>)
    where
        N: Into<String>,
        V: Into<GenValue>,
    {
        let name = name.into();
        let value = value.map(Into::into);
        match self.parameters.entry(name) {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
            },
            Entry::Vacant(entry) => {
                entry.insert(value);
            },
        }
    }
}

impl fmt::Display for ContactHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = &self.display_name {
            if name.contains(' ') {
                write!(f, "\"{}\" <{}>", name, self.uri)?;
            } else if name.is_empty() {
                write!(f, "\"\" <{}>", self.uri)?;
            } else {
                write!(f, "{} <{}>", name, self.uri)?;
            }
        } else {
            write!(f, "{}", self.uri)?;
        }
        for (key, value) in self.parameters.iter() {
            write!(f, ";{}", key)?;
            if let Some(value) = value {
                write!(f, "={}", value)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for GenValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GenValue::Token(string) => write!(f, "{}", string),
            GenValue::QuotedString(string) => write!(f, "\"{}\"", string),
        }
    }
}

/// Parse as many valid named field params as the input contains.
pub fn parse_contact_field_params<'a, E: ParseError<&'a [u8]>>(
    mut input: &'a [u8],
) -> IResult<&'a [u8], HashMap<String, Option<GenValue>>, E> {
    let mut map = HashMap::new();
    while let Ok((data, (key, value))) = parse_generic_param_with_possibly_quoted_value::<E>(input)
    {
        map.insert(key, value);
        input = data;
    }
    Ok((input, map))
}
