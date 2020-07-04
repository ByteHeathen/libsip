use nom::{
    branch::alt,
    bytes::complete::take_while,
    character::{complete::char, *},
    combinator::{map_res, opt},
    error::ParseError,
    IResult,
};

use crate::{headers::parse::parse_generic_param, parse::*, uri::parse_uri, Uri};

use std::{
    collections::{hash_map::Entry, HashMap},
    fmt,
};

/// Header Value for Named Headers,
/// e.g. From, To, Contact
#[derive(Debug, PartialEq, Clone)]
pub struct NamedHeader {
    pub display_name: Option<String>,
    pub uri: Uri,
    pub parameters: HashMap<String, Option<String>>,
}

impl NamedHeader {
    pub fn new(uri: Uri) -> NamedHeader {
        NamedHeader {
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
        V: Into<String>,
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
        V: Into<String>,
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

impl fmt::Display for NamedHeader {
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

/// Parse the name part of the NamedHeader.
pub fn parse_name<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], String, E> {
    Ok(alt::<_, _, E, _>((
        parse_quoted_string::<E>,
        parse_unquoted_string::<E>,
    ))(input)?)
}

/// Parse a stream of text that is not quoted. This will stop
/// at the first ' ' char the input contains.
pub fn parse_unquoted_string<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], String, E> {
    let (input, string_data) = map_res(take_while(is_alphabetic), slice_to_string::<E>)(input)?;
    let (input, _) = char(' ')(input)?;
    Ok((input, string_data))
}

/// Parse a single NamedHeader value.
pub fn parse_named_field_value<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], (Option<String>, Uri), E> {
    let (input, name) = opt(parse_name)(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, _) = opt(char('<'))(input)?;
    let (input, value) = parse_uri(input)?;
    let (input, _) = opt(char('>'))(input)?;
    Ok((input, (name, value)))
}

/// Parse as many valid named field params as the input contains.
pub fn parse_named_field_params<'a, E: ParseError<&'a [u8]>>(
    mut input: &'a [u8],
) -> IResult<&'a [u8], HashMap<String, Option<String>>, E> {
    let mut map = HashMap::new();
    while let Ok((data, (key, value))) = parse_generic_param::<E>(input) {
        map.insert(key, value);
        input = data;
    }
    Ok((input, map))
}
