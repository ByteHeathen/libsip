use serde::{ Serialize, Deserialize };

use std::fmt;
use std::io;

pub mod schema;
pub use self::schema::Schema;
pub use self::schema::parse_schema;

pub mod domain;
pub use self::domain::Domain;
pub use self::domain::parse_domain;

pub mod params;
pub use self::params::Param;
pub use self::params::parse_params;

pub mod auth;
pub use self::auth::UriAuth;
pub use self::auth::parse_uriauth;

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

    pub fn new_schemaless(host: Domain) -> Uri {
        Uri {
            schema: None,
            host,
            auth: None,
            parameters: vec![]
        }
    }

    pub fn sip(host: Domain) -> Uri {
        Uri::new(Schema::Sip, host)
    }

    pub fn sips(host: Domain) -> Uri {
        Uri::new(Schema::Sips, host)
    }

    pub fn auth(mut self, auth: UriAuth) -> Uri {
        self.auth = Some(auth);
        self
    }

    pub fn authless(mut self) -> Uri {
        self.auth = None;
        self
    }

    pub fn parameter(mut self, p: Param) -> Uri {
        self.parameters.push(p);
        self
    }

    pub fn parameters(mut self, p: Vec<Param>) -> Uri {
        self.parameters = p;
        self
    }

    pub fn schemaless(mut self) -> Uri {
        self.schema = None;
        self
    }

    pub fn schema(mut self, schema: Schema) -> Uri {
        self.schema = Some(schema);
        self
    }

    pub fn host(&self) -> String {
        format!("{}", self.host)
    }

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
