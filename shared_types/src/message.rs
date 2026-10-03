use crate::role::Role;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub role: Role,
    pub content: Option<String>,
    pub name: Option<String>,
    pub reasoning: Option<String>,
    pub reasoning_content: Option<String>,
    pub tool_call_id: Option<String>,
    pub tool_calls: Option<Vec<Value>>,
}

impl Message {
    pub fn new_system(content: impl Display) -> Self {
        Self {
            role: Role::System,
            content: Some(content.to_string()),
            name: None,
            reasoning: None,
            reasoning_content: None,
            tool_call_id: None,
            tool_calls: None,
        }
    }

    pub fn new_user(content: impl Display) -> Self {
        Self {
            role: Role::User,
            content: Some(content.to_string()),
            name: None,
            reasoning: None,
            reasoning_content: None,
            tool_call_id: None,
            tool_calls: None,
        }
    }
}
