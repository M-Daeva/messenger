use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{Rarity, Tag};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateMessage {
        tag: Tag,
        body: String,
        rarity: Rarity,
    },
}
