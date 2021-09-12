use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FeatureDisabledError {
    msg: String,
}

impl FeatureDisabledError {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.to_string() }
    }
}

impl error::Error for FeatureDisabledError {}

impl fmt::Display for FeatureDisabledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "feature disabled: {}", self.msg)
    }
}
