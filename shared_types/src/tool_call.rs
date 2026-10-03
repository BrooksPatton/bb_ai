use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolCall {
    pub function: ToolCallFunction,
    pub id: String,
    pub index: usize,
    #[serde(rename = "type")]
    pub function_type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolCallFunction {
    pub arguments: String,
    pub name: String,
}
