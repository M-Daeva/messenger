use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetMessages {},
    GetMessageById { id: u128 },
    GetMessagesByAddr { addr: String },
    GetContractBalance {},
    GetUserStake { addr: String },
}
