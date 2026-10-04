use crate::{role::Role, tool_call::ToolCall};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::AddAssign};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Message {
    pub role: Role,
    pub content: Option<String>,
    pub name: Option<String>,
    pub reasoning: Option<String>,
    pub reasoning_content: Option<String>,
    pub tool_call_id: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
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

    pub fn new_tool(content: impl Display, id: String) -> Self {
        Self {
            role: Role::Tool,
            content: Some(content.to_string()),
            name: None,
            reasoning: None,
            reasoning_content: None,
            tool_call_id: Some(id),
            tool_calls: None,
        }
    }
}

impl AddAssign for Message {
    fn add_assign(&mut self, rhs: Self) {
        self.content = if let Some((mut left, right)) = self.content.take().zip(rhs.content) {
            left.push_str(&right);
            Some(left)
        } else {
            None
        };

        if rhs.name.is_some() {
            self.name = rhs.name;
        }

        self.reasoning_content = if let Some((mut left, right)) =
            self.reasoning_content.take().zip(rhs.reasoning_content)
        {
            left.push_str(&right);
            Some(left)
        } else {
            None
        };

        if rhs.tool_call_id.is_some() {
            self.tool_call_id = rhs.tool_call_id;
        }

        if rhs.tool_calls.is_some() {
            self.tool_calls = rhs.tool_calls;
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.content)
    }
}
