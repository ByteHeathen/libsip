use nom::character::is_alphanumeric;

/// Checks if a given character is a token ([RFC3261: Page 221, "token"](https://tools.ietf.org/html/rfc3261#page-221))
/// # Examples
///
/// ```
/// use libsip::core::parse::is_token;
/// assert!(is_token('a' as u8));
/// assert!(is_token('+' as u8));
/// assert!(!is_token('=' as u8));
/// ```
pub fn is_token(chr: u8) -> bool {
    is_alphanumeric(chr) || "-.!%*_+`'~".contains(char::from(chr))
}
