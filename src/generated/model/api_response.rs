use super::super::wirespec::*;
use regex;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApiResponse {
    pub code: Option<i32>,
    pub r#type: Option<String>,
    pub message: Option<String>,
}
impl ApiResponse {
    pub fn validate(&self) -> Vec<String> {
        return Vec::<String>::new();
    }
}
