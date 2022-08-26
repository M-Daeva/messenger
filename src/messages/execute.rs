use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateMessage {
        tag: String,
        body: String,
        rarity: String,
    },
    DeleteMessage {
        id: u128,
    },
    EditMessage {
        id: u128,
        body: String,
    },
    SwapTag {
        id: u128,
        tag: String,
    },
    StakeTokens {},
    UnstakeTokens {
        amount: u128,
    },
}
