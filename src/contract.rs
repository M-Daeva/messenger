#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coin, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MessageResponse, MessagesResponse, QueryMsg};
use crate::state::{Book, Message, Props, Rarity, Tag, BOOK, MSG_BY_ID};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:messenger";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let initial_book = Book {
        admin: deps.api.addr_validate(info.sender.as_str())?,
        id_cnt: 0,
        messages: Vec::<Message>::new(),
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    BOOK.save(deps.storage, &initial_book)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", initial_book.admin)
        .add_attribute("message amount", initial_book.messages.len().to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateMessage { body, tag, rarity } => {
            create_msg(deps, info, body, tag, rarity)
        }
    }
}

pub fn create_msg(
    deps: DepsMut,
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

    MSG_BY_ID.save(deps.storage, new_msg.id, &new_msg);
    book.messages.push(new_msg.clone());
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMessages {} => get_messages(deps),
        QueryMsg::GetMessageById { id } => get_msg_by_id(deps, id),
        QueryMsg::GetMessagesByAddr { addr } => todo!(),
    }
}

pub fn get_messages(deps: Deps) -> StdResult<Binary> {
    let book = BOOK.load(deps.storage)?;

    to_binary(&MessagesResponse {
        messages: book.messages,
    })
}

pub fn get_msg_by_id(deps: Deps, id: u128) -> StdResult<Binary> {
    let message = MSG_BY_ID.load(deps.storage, id)?;
    to_binary(&MessageResponse { message })
}

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, InstantiateMsg, MessageResponse, MessagesResponse, QueryMsg};
    use crate::state::{Book, Message, Rarity, Tag};
    use crate::ContractError;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{
        attr, from_binary, Addr, Coin, Empty, Env, MessageInfo, OwnedDeps, Response,
    };

    pub const ADMIN_ADDR: &str = "juno1gjqnuhv52pd2a7ets2vhw9w9qa9knyhyqd4qeg";
    pub const ALICE_ADDR: &str = "juno1chgwz55h9kepjq0fkj5supl2ta3nwu638camkg";
    pub const BOB_ADDR: &str = "juno18tnvnwkklyv4dyuj8x357n7vray4v4zulm2dr9";

    pub const BODY1: &str = "Together we can rule the galaxy!";
    pub const BODY2: &str = "Thank you, Max!";
    pub const BODY3: &str = "BUIDL!!!";

    type Instance = (
        OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        Env,
        MessageInfo,
        Result<Response, ContractError>,
    );

    fn get_instance(addr: &str) -> Instance {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(addr, &[]);
        let msg = InstantiateMsg {};

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg);
        (deps, env, info, res)
    }

    fn add_msg(instance: Instance, msg: ExecuteMsg, user_addr: &str, funds: &[Coin]) -> Instance {
        let (mut deps, env, info, _) = instance;
        let info = mock_info(user_addr, funds);
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg);
        (deps, env, info, res)
    }

    fn create_some_msgs() -> Instance {
        let (deps, env, info, res) = add_msg(
            get_instance(ADMIN_ADDR),
            ExecuteMsg::CreateMessage {
                tag: Tag::Juno,
                body: BODY1.to_string(),
                rarity: Rarity::Epic,
            },
            ALICE_ADDR,
            &[],
        );

        let (deps, env, msg, res) = add_msg(
            (deps, env.clone(), info.clone(), res),
            ExecuteMsg::CreateMessage {
                tag: Tag::Juno,
                body: BODY2.to_string(),
                rarity: Rarity::Epic,
            },
            BOB_ADDR,
            &[],
        );

        let (deps, env, msg, res) = add_msg(
            (deps, env.clone(), info.clone(), res),
            ExecuteMsg::CreateMessage {
                tag: Tag::Juno,
                body: BODY3.to_string(),
                rarity: Rarity::Epic,
            },
            ALICE_ADDR,
            &[],
        );

        (deps, env, info, res)
    }

    #[test]
    fn test_init() {
        let (_, _, _, res) = get_instance(ADMIN_ADDR);

        assert_eq!(
            res.unwrap().attributes,
            vec![
                attr("method", "instantiate"),
                attr("admin", ADMIN_ADDR),
                attr("message amount", "0")
            ]
        )
    }

    #[test]
    fn test_create_msg() {
        let (_, _, _, res) = add_msg(
            get_instance(ADMIN_ADDR),
            ExecuteMsg::CreateMessage {
                tag: Tag::Juno,
                body: BODY1.to_string(),
                rarity: Rarity::Epic,
            },
            ALICE_ADDR,
            &[],
        );

        assert_eq!(
            res.unwrap().attributes,
            vec![
                attr("method", "create_msg"),
                attr("sender", ALICE_ADDR),
                attr("tag", "JUNO"),
                attr("body", BODY1),
                attr("rarity", "Epic"),
                attr("lifetime_cnt", "1000000"),
                attr("cooldown_cnt", "1")
            ]
        )
    }

    #[test]
    fn test_query() {
        let (deps, env, _, _) = create_some_msgs();
        let msg = QueryMsg::GetMessages {};
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res = from_binary::<MessagesResponse>(&bin).unwrap();

        assert_eq!(res.messages.len(), 3);
    }

    #[test]
    fn test_query_msg_by_id() {
        let (deps, env, _, _) = create_some_msgs();
        let msg = QueryMsg::GetMessageById { id: 1 };
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res = from_binary::<MessageResponse>(&bin).unwrap();

        assert_eq!(
            res.message,
            Message {
                id: 1,
                sender: Addr::unchecked(BOB_ADDR), // for tests
                tag: "JUNO".to_string(),
                body: BODY2.to_string(),
                rarity: "Epic".to_string(),
                lifetime_cnt: 1_000_000,
                cooldown_cnt: 1,
            }
        );
    }
}
