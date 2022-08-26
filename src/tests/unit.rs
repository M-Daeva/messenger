use cosmwasm_std::{attr, coin, from_binary, Addr};

use crate::{
    contract::query,
    messages::{
        execute::ExecuteMsg,
        query::QueryMsg,
        response::{MessageResponse, MessagesResponse},
    },
    state::{Message, COMMON, DENOM, EPIC, RARE, TAG},
    tests::helpers::{
        add_msg, create_some_msgs, get_instance, ADMIN_ADDR, ALICE_ADDR, BOB_ADDR, BODY1, BODY2,
        BODY3,
    },
};

#[test]
fn test_init() {
    let (_, _, _, res) = get_instance(ADMIN_ADDR);

    assert_eq!(
        res.unwrap().attributes,
        vec![attr("method", "instantiate"), attr("admin", ADMIN_ADDR),]
    )
}

#[test]
fn test_common_msg() {
    // create msg
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

    assert_eq!(
        res.as_ref().unwrap().attributes,
        vec![
            attr("method", "create_msg"),
            attr("sender", ALICE_ADDR),
            attr("tag", TAG::JUNO),
            attr("body", BODY1),
            attr("rarity", COMMON.rar),
            attr("lifetime_cnt", COMMON.lft.to_string()),
            attr("cooldown_cnt", COMMON.cd.to_string())
        ]
    );

    // change tag
    let (_, _, _, res2) = add_msg(
        (deps, env, info, res),
        ExecuteMsg::SwapTag {
            id: 0,
            tag: TAG::OSMO.to_string(),
        },
        &ALICE_ADDR,
        &[],
    );

    assert_eq!(
        res2.unwrap().attributes,
        vec![
            attr("method", "swap_tag"),
            attr("id", "0"),
            attr("tag", TAG::OSMO),
        ]
    );
}

#[test]
fn test_query() {
    let (deps, env, info, _) = create_some_msgs();
    let msg = QueryMsg::GetMessages {};
    let bin = query(deps.as_ref(), env, info, msg).unwrap();
    let res = from_binary::<MessagesResponse>(&bin).unwrap();

    assert_eq!(res.messages.len(), 3);
}

#[test]
fn test_query_msg_by_id() {
    let (deps, env, info, _) = create_some_msgs();
    let msg = QueryMsg::GetMessageById { id: 1 };
    let bin = query(deps.as_ref(), env, info, msg).unwrap();
    let res = from_binary::<MessageResponse>(&bin).unwrap();

    assert_eq!(
        res.message,
        Message::new(1, Addr::unchecked(BOB_ADDR), TAG::JUNO, BODY2, &COMMON)
    );
}

#[test]
fn test_query_msgs_by_addr() {
    let (deps, env, info, _) = create_some_msgs();
    let msg = QueryMsg::GetMessagesByAddr {
        addr: ALICE_ADDR.to_string(),
    };
    let bin = query(deps.as_ref(), env, info, msg).unwrap();
    let res = from_binary::<MessagesResponse>(&bin).unwrap();

    assert_eq!(
        res.messages,
        vec![
            Message::new(0, Addr::unchecked(ALICE_ADDR), TAG::JUNO, BODY1, &COMMON),
            Message::new(2, Addr::unchecked(ALICE_ADDR), TAG::JUNO, BODY3, &COMMON),
        ]
    );
}

#[test]
fn test_epic_msg() {
    // create epic message
    let (deps, env, info, res) = add_msg(
        get_instance(ADMIN_ADDR),
        ExecuteMsg::CreateMessage {
            tag: TAG::JUNO.to_string(),
            body: BODY1.to_string(),
            rarity: EPIC.rar.to_string(),
        },
        &ALICE_ADDR,
        &[coin(EPIC.price.0, DENOM)],
    );

    assert_eq!(
        res.as_ref().unwrap().attributes,
        vec![
            attr("method", "create_msg"),
            attr("sender", ALICE_ADDR),
            attr("tag", TAG::JUNO),
            attr("body", BODY1),
            attr("rarity", EPIC.rar),
            attr("lifetime_cnt", EPIC.lft.to_string()),
            attr("cooldown_cnt", EPIC.cd.to_string())
        ]
    );

    // edit body
    let (_, _, _, res2) = add_msg(
        (deps, env, info, res),
        ExecuteMsg::EditMessage {
            id: 0,
            body: BODY2.to_string(),
        },
        &ALICE_ADDR,
        &[],
    );

    assert_eq!(
        res2.unwrap().attributes,
        vec![
            attr("method", "edit_msg"),
            attr("id", "0"),
            attr("body", BODY2),
        ]
    );
}

#[test]
fn test_rare_msg() {
    // stake funds
    let (deps, env, info, res) = add_msg(
        get_instance(ADMIN_ADDR),
        ExecuteMsg::StakeTokens {},
        &ALICE_ADDR,
        &[coin(RARE.stake_req.0, DENOM)],
    );

    assert_eq!(
        res.as_ref().unwrap().attributes,
        vec![
            attr("method", "stake_tokens"),
            attr("sender", ALICE_ADDR),
            attr("stake", RARE.stake_req.0.to_string()),
        ]
    );

    // create rare message
    let (deps2, env2, info2, res2) = add_msg(
        (deps, env, info, res),
        ExecuteMsg::CreateMessage {
            tag: TAG::JUNO.to_string(),
            body: BODY1.to_string(),
            rarity: RARE.rar.to_string(),
        },
        &ALICE_ADDR,
        &[],
    );

    assert_eq!(
        res2.as_ref().unwrap().attributes,
        vec![
            attr("method", "create_msg"),
            attr("sender", ALICE_ADDR),
            attr("tag", TAG::JUNO),
            attr("body", BODY1),
            attr("rarity", RARE.rar),
            attr("lifetime_cnt", RARE.lft.to_string()),
            attr("cooldown_cnt", RARE.cd.to_string())
        ]
    );

    // unstake funds
    let (_, _, _, res3) = add_msg(
        (deps2, env2, info2, res2),
        ExecuteMsg::UnstakeTokens {
            amount: RARE.stake_req.0 / 2,
        },
        &ALICE_ADDR,
        &[],
    );

    assert_eq!(
        res3.as_ref().unwrap().attributes,
        vec![
            attr("method", "unstake_tokens"),
            attr("sender", ALICE_ADDR),
            attr("stake", (RARE.stake_req.0 / 2).to_string()),
        ]
    );
}

#[test]
fn test_delete_msg() {
    // create common msg
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

    // delete massage
    let (deps2, env2, info2, res2) = add_msg(
        (deps, env, info, res),
        ExecuteMsg::DeleteMessage { id: 2 },
        &ADMIN_ADDR,
        &[],
    );

    assert_eq!(
        res2.unwrap().attributes,
        vec![attr("method", "delete_msg"), attr("id", "2"),]
    );
}
