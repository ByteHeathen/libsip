use serde::{ Serialize, Deserialize };

use std::fmt;
use std::io;
use std::str::FromStr;

pub mod schema;
pub use self::schema::Schema;
pub use self::schema::parse_schema;

pub mod domain;
pub use self::domain::Domain;
pub use self::domain::parse_domain;

pub mod params;
pub use self::params::Param;
pub use self::params::parse_params;
pub use self::params::parse_param;

pub mod auth;
pub use self::auth::UriAuth;
pub use self::auth::parse_uriauth;

/// Universal Rescource Identifier for libsip.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Uri {
    pub schema: Option<Schema>,
    pub host: Domain,
    pub auth: Option<UriAuth>,
    pub parameters: Vec<Param>
}

impl Uri {

    pub fn new(schema: Schema, host: Domain) -> Uri {
        Uri {
            schema: Some(schema),
            host,
            auth: None,
            parameters: vec![]
        }
    }

    /// Create a new Uri without schema.
    pub fn new_schemaless(host: Domain) -> Uri {
        Uri {
            schema: None,
            host,
            auth: None,
            parameters: vec![]
        }
    }

    /// Create a new Uri With schema set to `Schema::Sip`.
    pub fn sip(host: Domain) -> Uri {
        Uri::new(Schema::Sip, host)
    }

    /// Create a new Uri With schema set to `Schema::Sips`.
    pub fn sips(host: Domain) -> Uri {
        Uri::new(Schema::Sips, host)
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
    pub fn parameter(mut self, p: Param) -> Uri {
        self.parameters.push(p);
        self
    }

    /// Add a list of new parameters. This will remove
    /// all old parameters.
    pub fn parameters(mut self, p: Vec<Param>) -> Uri {
        self.parameters = p;
        self
    }

    /// Remove the Schema if there is any.
    pub fn schemaless(mut self) -> Uri {
        self.schema = None;
        self
    }

    /// Set a new Schema.
    pub fn schema(mut self, schema: Schema) -> Uri {
        self.schema = Some(schema);
        self
    }

    /// Set the host value.
    pub fn host(&self) -> String {
        format!("{}", self.host)
    }

    /// Retrieve a formatted string containing host and parameters.
    /// This can be used in the Via header.
    pub fn host_and_params(&self) -> Result<String, io::Error> {
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

named!(pub parse_uri<Uri>, do_parse!(
    schema: opt!(pair!(parse_schema, char!(':'))) >>
    auth: opt!(parse_uriauth) >>
    host: parse_domain >>
    parameters: parse_params >>
    (Uri { schema: schema.map(|item|item.0), host, parameters, auth })
));

impl FromStr for Uri {
    type Err = nom::Err<nom::error::ErrorKind>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_uri(s.as_bytes()).map_err(map_error)?.1)
    }
}

fn map_error<'a>(n: nom::Err<(&'a [u8], nom::error::ErrorKind)>) -> nom::Err<nom::error::ErrorKind> {
    match n {
        nom::Err::Error((_, a)) => nom::Err::Error(a),
        nom::Err::Failure((_, b)) => nom::Err::Failure(b),
        nom::Err::Incomplete(a) => nom::Err::Incomplete(a)
    }
}
