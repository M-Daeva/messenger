#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{
    actions::{
        execute::create_msg,
        instantiate::init,
        migrate::migrate_contract,
        query::{
            get_contract_balance, get_messages, get_msg_by_id, get_msgs_by_addr, get_user_stake,
        },
    },
    error::ContractError,
    messages::{
        execute::ExecuteMsg, instantiate::InstantiateMsg, migrate::MigrateMsg, query::QueryMsg,
    },
};

/// Creates a new contract with the specified parameters packed in the "msg" variable
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    init(deps, env, info, msg)
}

/// Exposes all the execute functions available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateMessage { tag, body, rarity } => {
            create_msg(deps, env, info, body, tag, rarity)
        }
    }
}

/// Exposes all the queries available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, info: MessageInfo, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMessageById { id } => get_msg_by_id(deps, env, info, id),
        QueryMsg::GetMessages {} => get_messages(deps, env, info),
        QueryMsg::GetMessagesByAddr { addr } => get_msgs_by_addr(deps, env, info, addr),
        QueryMsg::GetContractBalance {} => get_contract_balance(deps, env, info),
        QueryMsg::GetUserStake { addr } => get_user_stake(deps, env, info, addr),
    }
}

/// Used for contract migration
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MigrateMsg,
) -> Result<Response, ContractError> {
    migrate_contract(deps, env, info, msg)
}

// /// The entry point to the contract for processing replies from submessages
// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn reply(
//     deps: Deps,
//     env: Env,
//     info: MessageInfo,
//     msg: Reply,
// ) -> Result<Response, ContractError> {
// }
