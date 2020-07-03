use std::{collections::HashMap, str::FromStr};

/// Extracts a parameter with a given name from parameters if this parameter has any value
/// # Examples
/// ```
/// use libsip::core::extract_opt_param;
/// use std::collections::HashMap;
///
/// let mut params = HashMap::new();
/// params.insert("param1".to_string(), Some("value".to_string()));
/// params.insert("param2".to_string(), None);
///
/// let mut param1: Option<String> = None;
/// extract_opt_param(&mut params, "param1", &mut param1);
/// assert_eq!(param1, Some("value".to_string()));
/// assert_eq!(params.get("param1"), None);
///
/// let mut param2: Option<String> = None;
/// extract_opt_param(&mut params, "param2", &mut param2);
/// assert_eq!(param2, None);
/// assert_eq!(params.get("param2"), Some(&None));
/// ```
pub fn extract_opt_param<V: FromStr>(
    params: &mut HashMap<String, Option<String>>,
    param: &str,
    extracted_value: &mut Option<V>,
) {
    if let Some(Some(value)) = params.get(param) {
        if let Ok(value) = V::from_str(value) {
            *extracted_value = Some(value)
        }
    }
    if extracted_value.is_some() {
        params.remove(param);
    }
}
