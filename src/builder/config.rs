#[derive(Debug, PartialEq, Clone)]
pub struct BuilderConfig {
    pub content_length: bool,
    pub add_cseq: bool,
    pub expires_header: Option<u32>,
    pub user_agent: Option<String>
}

impl Default for BuilderConfig {
    fn default() -> BuilderConfig {
        BuilderConfig {
            content_length: true,
            add_cseq: true,
            expires_header: Some(60),
            user_agent: Some(format!("libsip {}", env!("CARGO_PKG_VERSION")))
        }
    }
}
