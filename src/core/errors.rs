#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissingHeaderError {
    From,
    To,
    Via,
    CallId,
    CSeq,
    Contact,
    Expires,
    Event,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissingTagError {
    From,
    To,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissingUsernameError {
    Uri,
    From,
    To,
    Contact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissingViaBranchError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissingContactExpiresError;
