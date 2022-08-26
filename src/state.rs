use cosmwasm_std::{coin, Addr, Coin};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const DENOM: &str = "ujunox";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Base<'a> {
    pub rar: &'a str,
    pub lft: u128,
    pub cd: u128,
    pub price: (u128, &'a str),
    pub stake_req: (u128, &'a str),
}

pub const COMMON: Base = Base {
    rar: "Common",
    lft: 5,
    cd: 1,
    price: (0, DENOM),
    stake_req: (0, DENOM),
};

pub const RARE: Base = Base {
    rar: "Rare",
    lft: 30,
    cd: 2,
    price: (0, DENOM),
    stake_req: (100_000, DENOM),
};

pub const EPIC: Base = Base {
    rar: "Epic",
    lft: 100_000,
    cd: 1,
    price: (100_000, DENOM),
    stake_req: (0, DENOM),
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TAG;

impl<'a> TAG {
    pub const ATOM: &'a str = "Atom";
    pub const OSMO: &'a str = "Osmo";
    pub const JUNO: &'a str = "Juno";
}

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

impl Message {
    pub fn new(id: u128, sender: Addr, tag: &str, body: &str, base: &Base) -> Self {
        Message {
            id,
            sender,
            tag: tag.to_string(),
            body: body.to_string(),
            rarity: base.rar.to_string(),
            lifetime_cnt: base.lft,
            cooldown_cnt: base.cd,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct User {
    pub addr: String,
    pub stake: Coin,
}

impl User {
    pub fn new(addr: String, stake: Coin) -> Self {
        User { addr, stake }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Props {
    pub name: String,
    pub lifetime: u128,
    pub cooldown: u128,
    pub price: Coin,
    pub stake_req: Coin,
}

impl Props {
    pub fn new(
        name: &str,
        lifetime: u128,
        cooldown: u128,
        price: (u128, &str),
        stake_req: (u128, &str),
    ) -> Self {
        Props {
            name: name.to_string(),
            lifetime,
            cooldown,
            price: coin(price.0, price.1),
            stake_req: coin(stake_req.0, stake_req.1),
        }
    }
}

pub const BOOK: Item<Book> = Item::new("book");
pub const MESSAGES: Map<u128, Message> = Map::new("message");
pub const USERS: Map<String, User> = Map::new("user");
