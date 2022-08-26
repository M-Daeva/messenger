#[cfg(not(feature = "library"))]
use cosmwasm_std::{coin, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::{
    error::ContractError,
    state::{Message, User, BOOK, COMMON, DENOM, EPIC, MESSAGES, RARE, TAG, USERS},
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
        _ if tag == TAG::ATOM || tag == TAG::OSMO || tag == TAG::JUNO => tag,
        _ => TAG::ATOM.to_string(),
    };

    // NEED REFACTORING
    let base = match rarity.as_ref() {
        "Rare" => RARE,
        "Epic" => {
            if info.funds.len() == 1 && info.funds[0] == coin(EPIC.price.0, EPIC.price.1) {
                EPIC
            } else {
                return Err(ContractError::CustomError {
                    val: "There is no funds!".to_string(),
                });
            }
        }
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

pub fn delete_msg(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u128,
) -> Result<Response, ContractError> {
    let message = MESSAGES.load(deps.storage, id)?;
    let book = BOOK.load(deps.storage)?;

    // verify sender
    if info.sender != message.sender && info.sender != book.admin {
        return Err(ContractError::CustomError {
            val: "Sender is not an owner!".to_string(),
        });
    }

    MESSAGES.remove(deps.storage, id);

    Ok(Response::new().add_attributes(vec![
        ("method", "delete_msg"),
        ("id", id.to_string().as_ref()),
    ]))
}

pub fn edit_msg(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u128,
    body: String,
) -> Result<Response, ContractError> {
    let mut message = MESSAGES.load(deps.storage, id)?;

    // verify sender
    if info.sender != message.sender {
        return Err(ContractError::CustomError {
            val: "Sender is not an owner!".to_string(),
        });
    }

    // verify rarity
    if message.rarity == COMMON.rar {
        return Err(ContractError::CustomError {
            val: "It is impossible to edit common messages!".to_string(),
        });
    }

    message.body = body;
    MESSAGES.save(deps.storage, id, &message)?;

    Ok(Response::new().add_attributes(vec![
        ("method", "edit_msg"),
        ("id", id.to_string().as_ref()),
        ("body", message.body.as_ref()),
    ]))
}

pub fn swap_tag(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u128,
    tag: String,
) -> Result<Response, ContractError> {
    let mut message = MESSAGES.load(deps.storage, id)?;

    // verify sender
    if info.sender != message.sender {
        return Err(ContractError::CustomError {
            val: "Sender is not an owner!".to_string(),
        });
    }

    // verify a tag
    let new_tag = match tag {
        _ if tag == TAG::ATOM || tag == TAG::OSMO || tag == TAG::JUNO => tag,
        _ => TAG::ATOM.to_string(),
    };

    message.tag = new_tag.clone();
    MESSAGES.save(deps.storage, id, &message)?;

    Ok(Response::new().add_attributes(vec![
        ("method", "swap_tag"),
        ("id", id.to_string().as_ref()),
        ("tag", new_tag.as_ref()),
    ]))
}

pub fn stake_tokens(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // verify funds
    if info.funds.is_empty() {
        return Err(ContractError::CustomError {
            val: "There is no funds!".to_string(),
        });
    }
    // verify denom
    if info.funds[0].denom != DENOM {
        return Err(ContractError::CustomError {
            val: "Wrong denom!".to_string(),
        });
    }

    let user_addr = info.sender;
    let mut user = match USERS.load(deps.storage, user_addr.to_string()) {
        Ok(user) => user,
        _ => User::new(user_addr.to_string(), coin(0, DENOM)),
    };

    let amount = (user.stake.amount + info.funds[0].amount).u128();
    user.stake = coin(amount, DENOM);
    USERS.save(deps.storage, user_addr.to_string(), &user)?;

    Ok(Response::new().add_attributes(vec![
        ("method", "stake_tokens"),
        ("sender", user.addr.as_ref()),
        ("stake", amount.to_string().as_ref()),
    ]))
}

pub fn unstake_tokens(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: u128,
) -> Result<Response, ContractError> {
    let user_addr = info.sender;
    let mut user = USERS.load(deps.storage, user_addr.to_string())?;

    let new_amount = match user.stake.amount.checked_sub(Uint128::new(amount)) {
        Ok(res) => res,
        Err(e) => return Err(ContractError::CustomError { val: e.to_string() }),
    };

    user.stake.amount = new_amount;
    USERS.save(deps.storage, user_addr.to_string(), &user)?;

    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: user_addr.to_string(),
        amount: vec![coin(new_amount.u128(), DENOM)],
    });

    Ok(Response::new().add_message(msg).add_attributes(vec![
        ("method", "unstake_tokens"),
        ("sender", user.addr.as_ref()),
        ("stake", amount.to_string().as_ref()),
    ]))
}
