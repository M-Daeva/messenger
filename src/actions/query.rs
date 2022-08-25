#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, Binary, Deps, Env, MessageInfo, Order, StdError, StdResult};

use crate::{
    messages::response::{BalanceResponse, MessageResponse, MessagesResponse},
    state::{Message, DENOM, MESSAGES, USERS},
};

pub fn get_messages(deps: Deps, _env: Env, _info: MessageInfo) -> StdResult<Binary> {
    let messages = MESSAGES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&MessagesResponse { messages })
}

pub fn get_msg_by_id(deps: Deps, _env: Env, _info: MessageInfo, id: u128) -> StdResult<Binary> {
    let message = MESSAGES.load(deps.storage, id)?;
    to_binary(&MessageResponse { message })
}

pub fn get_msgs_by_addr(
    deps: Deps,
    _env: Env,
    _info: MessageInfo,
    addr: String,
) -> StdResult<Binary> {
    fn compare(p: &Result<(u128, Message), StdError>, addr: &String) -> bool {
        let (_, v) = p.as_ref().unwrap();
        v.sender == String::from(addr)
    }

    let messages = MESSAGES
        .range(deps.storage, None, None, Order::Ascending)
        .filter(|p| compare(p, &addr))
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&MessagesResponse { messages })
}

pub fn get_contract_balance(deps: Deps, env: Env, _info: MessageInfo) -> StdResult<Binary> {
    let balance = deps.querier.query_balance(env.contract.address, DENOM)?;
    to_binary(&BalanceResponse { balance })
}

pub fn get_user_stake(
    deps: Deps,
    _env: Env,
    _info: MessageInfo,
    addr: String,
) -> StdResult<Binary> {
    let user = USERS.load(deps.storage, addr)?;
    to_binary(&BalanceResponse {
        balance: user.stake,
    })
}
