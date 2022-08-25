#[cfg(not(feature = "library"))]
use cosmwasm_std::{coin, DepsMut, Env, MessageInfo, Response};

use crate::{
    error::ContractError,
    state::{Message, BOOK, COMMON, EPIC, MESSAGES, RARE, TAG},
};

pub fn create_msg(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    body: String,
    tag: String,
    rarity: String,
) -> Result<Response, ContractError> {
    let mut book = BOOK.load(deps.storage)?;

    // verify a tag
    let new_tag = match tag {
        _ if &tag == TAG::ATOM || &tag == TAG::OSMO || &tag == TAG::JUNO => tag,
        _ => TAG::ATOM.to_string(),
    };

    let base = match rarity.as_ref() {
        "Rare" => RARE,
        "Epic" if info.funds[0] == coin(EPIC.price.0, EPIC.price.1) => EPIC,
        _ => COMMON,
    };

    let msg = Message::new(book.id_cnt, info.sender, &new_tag, &body, &base);

    MESSAGES.save(deps.storage, msg.id, &msg)?;
    book.id_cnt += 1;
    BOOK.save(deps.storage, &book)?;

    Ok(Response::new().add_attributes(vec![
        ("method", "create_msg"),
        ("sender", msg.sender.as_ref()),
        ("tag", msg.tag.as_ref()),
        ("body", msg.body.as_ref()),
        ("rarity", msg.rarity.as_ref()),
        ("lifetime_cnt", msg.lifetime_cnt.to_string().as_ref()),
        ("cooldown_cnt", msg.cooldown_cnt.to_string().as_ref()),
    ]))
}
