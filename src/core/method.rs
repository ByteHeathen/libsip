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
    Update
}

impl Method {

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
            Method::Update
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
            Method::Update => write!(f, "UPDATE")
        }
    }
}

named!(pub parse_method<Method>, alt!(
    map!(tag!("INVITE"), |_| Method::Invite) |
    map!(tag!("ACK"), |_| Method::Ack) |
    map!(tag!("BYE"), |_| Method::Bye) |
    map!(tag!("CANCEL"), |_| Method::Cancel) |
    map!(tag!("REGISTER"), |_| Method::Register) |
    map!(tag!("OPTIONS"), |_| Method::Options) |
    map!(tag!("PRACK"), |_| Method::PRack) |
    map!(tag!("SUBSCRIBE"), |_| Method::Subscribe) |
    map!(tag!("NOTIFY"), |_| Method::Notify) |
    map!(tag!("PUBLISH"), |_| Method::Publish) |
    map!(tag!("INFO"), |_| Method::Info) |
    map!(tag!("REFER"), |_| Method::Refer) |
    map!(tag!("MESSAGE"), |_| Method::Message) |
    map!(tag!("UPDATE"), |_| Method::Update)
));
