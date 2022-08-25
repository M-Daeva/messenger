use cosmwasm_std::{attr, from_binary, Addr};

use crate::{
    contract::query,
    messages::{
        execute::ExecuteMsg,
        query::QueryMsg,
        response::{MessageResponse, MessagesResponse},
    },
    state::{Message, COMMON, EPIC, RARE, TAG},
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
fn test_create_msg() {
    let (_, _, _, res) = add_msg(
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
        res.unwrap().attributes,
        vec![
            attr("method", "create_msg"),
            attr("sender", ALICE_ADDR),
            attr("tag", TAG::JUNO),
            attr("body", BODY1),
            attr("rarity", COMMON.rar),
            attr("lifetime_cnt", COMMON.lft.to_string()),
            attr("cooldown_cnt", COMMON.cd.to_string())
        ]
    )
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
