use std::fmt;

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

#[derive(Debug, PartialEq, Clone)]
pub struct Uri {
    schema: Schema,
    host: Domain,
    auth: Option<UriAuth>,
    parameters: Vec<Param>
}

impl Uri {

    pub fn new(schema: Schema, host: Domain) -> Uri {
        Uri {
            schema,
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

    pub fn parameter(mut self, p: Param) -> Uri {
        self.parameters.push(p);
        self
    }

    pub fn parameters(mut self, p: Vec<Param>) -> Uri {
        self.parameters = p;
        self
    }
    pub fn host_and_params(&self) -> Result<String, fmt::Error> {
        let mut auth = format!("{}", self.host);
        for param in &self.parameters {
            auth += &format!("{}", param);
        }
        Ok(auth)
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:", self.schema)?;
        if let Some(auth) = &self.auth {
            write!(f, "{}@", auth)?;
        }
        write!(f, "{}", self.host)?;
        for param in &self.parameters {
            write!(f, "{}", param)?;
        }
        Ok(())
    }
}

named!(pub parse_uri<Uri>, do_parse!(
    schema: parse_schema >>
    char!(':') >>
    auth: opt!(parse_uriauth) >>
    host: parse_domain >>
    parameters: parse_params >>
    (Uri { schema, host, parameters, auth })
));
