use crate::Uri;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct NamedHeader {
    pub display_name: Option<String>,
    pub uri: Uri,
    pub params: HashMap<String, String>
}
