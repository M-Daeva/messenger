use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Message;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MessagesResponse {
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MessageResponse {
    pub message: Message,
}
