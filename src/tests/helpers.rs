use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Coin, Empty, Env, MessageInfo, OwnedDeps, Response,
};

use crate::{
    contract::{execute, instantiate},
    error::ContractError,
    messages::{execute::ExecuteMsg, instantiate::InstantiateMsg},
    state::{COMMON, EPIC, RARE, TAG},
};

pub const ADMIN_ADDR: &str = "juno1gjqnuhv52pd2a7ets2vhw9w9qa9knyhyqd4qeg";
pub const ALICE_ADDR: &str = "juno1chgwz55h9kepjq0fkj5supl2ta3nwu638camkg";
pub const BOB_ADDR: &str = "juno18tnvnwkklyv4dyuj8x357n7vray4v4zulm2dr9";

pub const BODY1: &str = "Together we can rule the galaxy!";
pub const BODY2: &str = "Thank you, Max!";
pub const BODY3: &str = "BUIDL!!!";

pub type Instance = (
    OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    Env,
    MessageInfo,
    Result<Response, ContractError>,
);

pub fn get_instance(addr: &str) -> Instance {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(addr, &[]);
    let msg = InstantiateMsg {};

    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg);
    (deps, env, info, res)
}

pub fn add_msg(instance: Instance, msg: ExecuteMsg, user_addr: &str, funds: &[Coin]) -> Instance {
    let (mut deps, env, _info, _) = instance;
    let info = mock_info(user_addr, funds);
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg);
    (deps, env, info, res)
}

pub fn create_some_msgs() -> Instance {
    let (deps, env, info, res) = add_msg(
        get_instance(ADMIN_ADDR),
        ExecuteMsg::CreateMessage {
            tag: TAG::JUNO.to_string(),
            body: BODY1.to_string(),
            rarity: COMMON.rar.to_string(),
        },
        &ALICE_ADDR,
        &[],
    );

    let (deps, env, _msg, res) = add_msg(
        (deps, env.clone(), info.clone(), res),
        ExecuteMsg::CreateMessage {
            tag: TAG::JUNO.to_string(),
            body: BODY2.to_string(),
            rarity: COMMON.rar.to_string(),
        },
        &BOB_ADDR,
        &[],
    );

    let (deps, env, _msg, res) = add_msg(
        (deps, env.clone(), info.clone(), res),
        ExecuteMsg::CreateMessage {
            tag: TAG::JUNO.to_string(),
            body: BODY3.to_string(),
            rarity: COMMON.rar.to_string(),
        },
        &ALICE_ADDR,
        &[],
    );

    (deps, env, info, res)
}
