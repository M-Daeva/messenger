use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Book {
    pub admin: Addr,
    pub id_cnt: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Message {
    pub id: u128,
    pub sender: Addr,
    pub tag: String,
    pub body: String,
    pub rarity: String,
    pub lifetime_cnt: u128,
    pub cooldown_cnt: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Tag {
    Atom,
    Osmo,
    Juno,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Bank {
    pub addr: Addr,
    pub balance: Coin,
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct User {
    pub addr: Addr,
    pub stake: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Props {
    pub name: String,
    pub lifetime: u128,
    pub cooldown: u128,
    pub price: Coin,
    pub stake_req: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
}

pub const BOOK: Item<Book> = Item::new("book");
pub const MESSAGES: Map<u128, Message> = Map::new("message");
