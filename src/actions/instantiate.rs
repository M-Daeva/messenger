#[cfg(not(feature = "library"))]
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::{
    error::ContractError,
    messages::instantiate::InstantiateMsg,
    state::{Book, BOOK},
};

const CONTRACT_NAME: &str = "crates.io:boilerplate-test";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn init(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let initial_book = Book {
        admin: deps.api.addr_validate(info.sender.as_str())?,
        id_cnt: 0,
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    BOOK.save(deps.storage, &initial_book)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", initial_book.admin))
}
