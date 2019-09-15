#[derive(Debug, PartialEq, Clone)]
pub struct BuilderConfig {
    pub req_content_length_header: bool,
    pub res_content_length_header: bool
}

impl Default for BuilderConfig {
    fn default() -> BuilderConfig {
        BuilderConfig {
            req_content_length_header: true,
            res_content_length_header: true
        }
    }
}
