#[cfg(not(feature = "library"))]
use cosmwasm_std::{coin, DepsMut, Env, MessageInfo, Response};

use crate::{
    error::ContractError,
    state::{Message, Props, Rarity, Tag, BOOK, MESSAGES},
};

pub fn create_msg(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    body: String,
    tag: Tag,
    rarity: Rarity,
) -> Result<Response, ContractError> {
    let mut book = BOOK.load(deps.storage)?;

    let new_tag = match tag {
        Tag::Atom => String::from("ATOM"),
        Tag::Osmo => String::from("OSMO"),
        Tag::Juno => String::from("JUNO"),
    };

    let props = match rarity {
        Rarity::Common => Props {
            name: String::from("Common"),
            lifetime: 5,
            cooldown: 1,
            price: coin(0, "ujunox"),
            stake_req: coin(0, "ujunox"),
        },
        Rarity::Rare => Props {
            name: String::from("Rare"),
            lifetime: 30,
            cooldown: 2,
            price: coin(0, "ujunox"),
            stake_req: coin(1_000_000, "ujunox"),
        },
        Rarity::Epic => Props {
            name: String::from("Epic"),
            lifetime: 1_000_000,
            cooldown: 1,
            price: coin(1_000_000, "ujunox"),
            stake_req: coin(0, "ujunox"),
        },
    };

    let new_msg = Message {
        id: book.id_cnt,
        sender: deps.api.addr_validate(info.sender.as_str())?,
        tag: new_tag,
        body,
        rarity: props.name,
        lifetime_cnt: props.lifetime,
        cooldown_cnt: props.cooldown,
    };

    MESSAGES.save(deps.storage, new_msg.id, &new_msg)?;
    book.id_cnt += 1;
    BOOK.save(deps.storage, &book)?;

    Ok(Response::new()
        .add_attribute("method", "create_msg")
        .add_attribute("sender", new_msg.sender)
        .add_attribute("tag", new_msg.tag)
        .add_attribute("body", new_msg.body)
        .add_attribute("rarity", new_msg.rarity)
        .add_attribute("lifetime_cnt", new_msg.lifetime_cnt.to_string())
        .add_attribute("cooldown_cnt", new_msg.cooldown_cnt.to_string()))
}
