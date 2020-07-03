use crate::{
    core::extract_opt_param,
    headers::{
        named::parse_named_field_params,
        write::{write_generic_params, write_optional_param},
        Header,
    },
    parse::is_token,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while},
    character::{complete::char, is_space},
    combinator::{map, opt},
    error::ParseError,
    IResult,
};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug, PartialEq)]
pub enum SubscriptionState {
    Active {
        expires: Option<u32>,
        params: HashMap<String, Option<String>>,
    },
    Pending {
        expires: Option<u32>,
        params: HashMap<String, Option<String>>,
    },
    Terminated {
        retry_after: Option<u32>,
        reason: Option<String>,
        params: HashMap<String, Option<String>>,
    },
    Other {
        state: String,
        params: HashMap<String, Option<String>>,
    },
}

impl SubscriptionState {
    pub fn set_params(&mut self, params: HashMap<String, Option<String>>) {
        match self {
            Self::Active { params: p, .. }
            | Self::Pending { params: p, .. }
            | Self::Terminated { params: p, .. }
            | Self::Other { params: p, .. } => {
                *p = params;
            },
        }
    }
}

impl fmt::Display for SubscriptionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Active { expires, params } => {
                write!(f, "active")?;
                write_optional_param("expires", expires, f)?;
                write_generic_params(params, f)
            },
            Self::Pending { expires, params } => {
                write!(f, "pending")?;
                write_optional_param("expires", expires, f)?;
                write_generic_params(params, f)
            },
            Self::Terminated {
                retry_after,
                reason,
                params,
            } => {
                write!(f, "terminated")?;
                write_optional_param("retry-after", retry_after, f)?;
                write_optional_param("reason", reason, f)?;
                write_generic_params(params, f)
            },
            Self::Other { state, params } => {
                write!(f, "{}", state)?;
                write_generic_params(params, f)
            },
        }
    }
}

/// Parses Subscription-State header ([RFC6665: Page 45, "Subscription-State"](https://tools.ietf.org/html/rfc6665#page-45))
/// # Examples
///
/// ```
/// use libsip::{
///     headers::subscription_state::{parse_subscription_state_header, SubscriptionState},
///     Header,
/// };
/// use nom::error::VerboseError;
/// use std::collections::HashMap;
///
/// let mut params = HashMap::new();
/// params.insert(String::from("wow"), None);
/// assert_eq!(
///     parse_subscription_state_header::<VerboseError<_>>(
///         b"Subscription-State: active;expires=600;wow\r\n"
///     ),
///     Ok((
///         "".as_bytes(),
///         Header::SubscriptionState(SubscriptionState::Active {
///             expires: Some(600),
///             params: params.clone(),
///         })
///     ))
/// );
///
/// assert_eq!(
///     parse_subscription_state_header::<VerboseError<_>>(
///         b"Subscription-State: pending;expires=600;wow\r\n"
///     ),
///     Ok((
///         "".as_bytes(),
///         Header::SubscriptionState(SubscriptionState::Pending {
///             expires: Some(600),
///             params: params.clone()
///         })
///     ))
/// );
///
/// assert_eq!(
///     parse_subscription_state_header::<VerboseError<_>>(
///         b"Subscription-State: terminated;retry-after=600;reason=giveup;wow\r\n"
///     ),
///     Ok((
///         "".as_bytes(),
///         Header::SubscriptionState(SubscriptionState::Terminated {
///             retry_after: Some(600),
///             reason: Some(String::from("giveup")),
///             params
///         })
///     ))
/// );
/// ```
pub fn parse_subscription_state_header<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], Header, E> {
    let (input, _) = opt(tag("\r\n"))(input)?;
    let (input, _) = tag_no_case("Subscription-State")(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = opt(take_while(is_space))(input)?;
    let (input, mut state) = parse_subscription_state_without_params(input)?;
    let (input, _) = parse_subscription_state_params(input, &mut state)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Header::SubscriptionState(state)))
}

pub fn parse_subscription_state_without_params<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], SubscriptionState, E> {
    alt::<_, _, E, _>((
        map(tag("active"), |_| SubscriptionState::Active {
            expires: None,
            params: HashMap::new(),
        }),
        map(tag("pending"), |_| SubscriptionState::Pending {
            expires: None,
            params: HashMap::new(),
        }),
        map(tag("terminated"), |_| SubscriptionState::Terminated {
            retry_after: None,
            reason: None,
            params: HashMap::new(),
        }),
        map(take_while(is_token), |state: &[u8]| {
            SubscriptionState::Other {
                state: String::from_utf8_lossy(state).to_string(),
                params: HashMap::new(),
            }
        }),
    ))(input)
}

pub fn parse_subscription_state_params<'a, E: ParseError<&'a [u8]>>(
    input: &'a [u8],
    state: &mut SubscriptionState,
) -> IResult<&'a [u8], (), E> {
    let (input, mut parsed_params) = parse_named_field_params(input)?;
    match state {
        SubscriptionState::Active { expires, .. } | SubscriptionState::Pending { expires, .. } => {
            extract_opt_param(&mut parsed_params, "expires", expires);
        },
        SubscriptionState::Terminated {
            retry_after,
            reason,
            ..
        } => {
            extract_opt_param(&mut parsed_params, "retry-after", retry_after);
            extract_opt_param(&mut parsed_params, "reason", reason);
        },
        SubscriptionState::Other { .. } => {},
    }
    state.set_params(parsed_params);
    Ok((input, ()))
}
