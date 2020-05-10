use serde::{Deserialize, Serialize};

use std::{
    fmt,
    io::Result as IoResult,
    str::FromStr
};

use nom::{
    IResult,
    character::complete::char,
    combinator::{opt},
    sequence::pair,
    error::ParseError
};

pub mod schema;
pub use self::schema::{parse_schema, UriSchema};

pub mod domain;
pub use self::domain::{parse_domain, Domain};

pub mod params;
pub use self::params::{parse_param, parse_params, UriParam};

pub mod auth;
pub use self::auth::{parse_uriauth, UriAuth};

/// Universal Rescource Identifier for libsip.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Uri {
    pub schema: Option<UriSchema>,
    pub host: Domain,
    pub auth: Option<UriAuth>,
    pub parameters: Vec<UriParam>,
}

impl Uri {
    pub fn new(schema: UriSchema, host: Domain) -> Uri {
        Uri {
            schema: Some(schema),
            host,
            auth: None,
            parameters: vec![],
        }
    }

    /// Create a new Uri without schema.
    pub fn new_schemaless(host: Domain) -> Uri {
        Uri {
            schema: None,
            host,
            auth: None,
            parameters: vec![],
        }
    }

    /// Create a new Uri With schema set to `Schema::Sip`.
    pub fn sip(host: Domain) -> Uri {
        Uri::new(UriSchema::Sip, host)
    }

    /// Create a new Uri With schema set to `Schema::Sips`.
    pub fn sips(host: Domain) -> Uri {
        Uri::new(UriSchema::Sips, host)
    }

    /// Add a `UriAuth` section to this Uri.
    pub fn auth(mut self, auth: UriAuth) -> Uri {
        self.auth = Some(auth);
        self
    }

    /// Remove authentication if there is any.
    pub fn authless(mut self) -> Uri {
        self.auth = None;
        self
    }

    /// Add a new parameter to the parameter list.
    pub fn parameter(mut self, p: UriParam) -> Uri {
        self.parameters.push(p);
        self
    }

    /// Add a list of new parameters. This will remove
    /// all old parameters.
    pub fn parameters(mut self, p: Vec<UriParam>) -> Uri {
        self.parameters = p;
        self
    }

    /// Remove the Schema if there is any.
    pub fn schemaless(mut self) -> Uri {
        self.schema = None;
        self
    }

    /// Set a new Schema.
    pub fn schema(mut self, schema: UriSchema) -> Uri {
        self.schema = Some(schema);
        self
    }

    /// Set the host value.
    pub fn host(&self) -> String {
        format!("{}", self.host)
    }

    /// Retrieve a formatted string containing host and parameters.
    /// This can be used in the Via header.
    pub fn host_and_params(&self) -> IoResult<String> {
        let mut host = self.host();
        for param in &self.parameters {
            host += &format!("{}", param);
        }
        Ok(host)
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(schema) = self.schema {
            write!(f, "{}:", schema)?;
        }
        if let Some(auth) = &self.auth {
            write!(f, "{}@{}", auth, self.host)?;
        } else {
            write!(f, "{}", self.host)?;
        }
        for param in &self.parameters {
            write!(f, "{}", param)?;
        }
        Ok(())
    }
}

pub fn parse_uri<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Uri, E> {
    let (input, schema) = opt(pair(parse_schema::<E>, char(':')))(input)?;
    let (input, auth) = opt(parse_uriauth::<E>)(input)?;
    let (input, host) = parse_domain::<E>(input)?;
    let (input, parameters) = parse_params::<E>(input)?;
    Ok((input, Uri { schema: schema.map(|item|item.0), host, parameters, auth}))
}

impl FromStr for Uri {
    type Err = nom::Err<nom::error::ErrorKind>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_uri(s.as_bytes()).map_err(map_error)?.1)
    }
}

fn map_error(n: nom::Err<(&[u8], nom::error::ErrorKind)>) -> nom::Err<nom::error::ErrorKind> {
    match n {
        nom::Err::Error((_, a)) => nom::Err::Error(a),
        nom::Err::Failure((_, b)) => nom::Err::Failure(b),
        nom::Err::Incomplete(a) => nom::Err::Incomplete(a),
    }
}
