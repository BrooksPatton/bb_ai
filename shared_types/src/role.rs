use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    #[default]
    Assistant,
    User,
    Tool,
}
