use nom::{branch::alt, bytes::complete::tag_no_case, combinator::map, error::ParseError, IResult};
use std::fmt;

/// SIP protocol methods.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Method {
    Invite,
    Ack,
    Bye,
    Cancel,
    Register,
    Options,
    PRack,
    Subscribe,
    Notify,
    Publish,
    Info,
    Refer,
    Message,
    Update,
}

impl Method {
    /// Return all possible Methods. This comes in handy when
    /// adding an AllowedMethods Header.
    pub fn all() -> Vec<Method> {
        vec![
            Method::Invite,
            Method::Ack,
            Method::Bye,
            Method::Cancel,
            Method::Register,
            Method::Options,
            Method::PRack,
            Method::Subscribe,
            Method::Notify,
            Method::Publish,
            Method::Info,
            Method::Refer,
            Method::Message,
            Method::Update,
        ]
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::Invite => write!(f, "INVITE"),
            Method::Ack => write!(f, "ACK"),
            Method::Bye => write!(f, "BYE"),
            Method::Cancel => write!(f, "CANCEL"),
            Method::Register => write!(f, "REGISTER"),
            Method::Options => write!(f, "OPTIONS"),
            Method::PRack => write!(f, "PRACK"),
            Method::Subscribe => write!(f, "SUBSCRIBE"),
            Method::Notify => write!(f, "NOTIFY"),
            Method::Publish => write!(f, "PUBLISH"),
            Method::Info => write!(f, "INFO"),
            Method::Refer => write!(f, "REFER"),
            Method::Message => write!(f, "MESSAGE"),
            Method::Update => write!(f, "UPDATE"),
        }
    }
}

/// Parse SIP request Method.
pub fn parse_method<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Method, E> {
    alt::<_, _, E, _>((
        map(tag_no_case("INVITE"), |_| Method::Invite),
        map(tag_no_case("ACK"), |_| Method::Ack),
        map(tag_no_case("BYE"), |_| Method::Bye),
        map(tag_no_case("CANCEL"), |_| Method::Cancel),
        map(tag_no_case("REGISTER"), |_| Method::Register),
        map(tag_no_case("OPTIONS"), |_| Method::Options),
        map(tag_no_case("PRACK"), |_| Method::PRack),
        map(tag_no_case("SUBSCRIBE"), |_| Method::Subscribe),
        map(tag_no_case("NOTIFY"), |_| Method::Notify),
        map(tag_no_case("PUBLISH"), |_| Method::Publish),
        map(tag_no_case("INFO"), |_| Method::Info),
        map(tag_no_case("REFER"), |_| Method::Refer),
        map(tag_no_case("MESSAGE"), |_| Method::Message),
        map(tag_no_case("UPDATE"), |_| Method::Update),
    ))(input)
}
