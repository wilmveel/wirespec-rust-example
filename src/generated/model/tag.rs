use super::super::wirespec::*;
use regex;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: Option<String>,
}
impl Tag {
    pub fn validate(&self) -> Vec<String> {
        return Vec::<String>::new();
    }
}
