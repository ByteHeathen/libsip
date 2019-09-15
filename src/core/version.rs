use nom::character::is_digit;

use crate::parse::parse_u8;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Version(u8, u8);

impl Default for Version {
    fn default() -> Version {
        Version(2, 0)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SIP/{}.{}", self.0, self.1)
    }
}

impl Version {

    pub fn new(maj: u8, min: u8) -> Version {
        Version(maj, min)
    }
}

named!(
	pub parse_version<Version>,
	do_parse!(
		tag!("SIP/") >>
		major: map_res!(take_while1!(is_digit), parse_u8) >>
		char!('.') >>
		minor: map_res!(take_while1!(is_digit), parse_u8) >>
		(Version ( major, minor ))
	)
);
