#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum MissingTagError {
    From,
    To,
}

#[derive(Debug, Clone, Copy)]
pub enum MissingUsernameError {
    From,
    To,
}

#[derive(Debug, Clone, Copy)]
pub struct MissingViaBranchError;

#[derive(Debug, Clone, Copy)]
pub struct MissingContactExpiresError;
