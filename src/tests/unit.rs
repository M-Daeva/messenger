use cosmwasm_std::{attr, from_binary, Addr};

use crate::{
    contract::query,
    messages::{
        execute::ExecuteMsg,
        query::QueryMsg,
        response::{MessageResponse, MessagesResponse},
    },
    state::{Message, Rarity, Tag},
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
        Message {
            id: 1,
            sender: Addr::unchecked(BOB_ADDR),
            tag: "JUNO".to_string(),
            body: BODY2.to_string(),
            rarity: "Epic".to_string(),
            lifetime_cnt: 1_000_000,
            cooldown_cnt: 1,
        }
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
            Message {
                id: 0,
                sender: Addr::unchecked(ALICE_ADDR),
                tag: "JUNO".to_string(),
                body: BODY1.to_string(),
                rarity: "Epic".to_string(),
                lifetime_cnt: 1_000_000,
                cooldown_cnt: 1,
            },
            Message {
                id: 2,
                sender: Addr::unchecked(ALICE_ADDR),
                tag: "JUNO".to_string(),
                body: BODY3.to_string(),
                rarity: "Epic".to_string(),
                lifetime_cnt: 1_000_000,
                cooldown_cnt: 1,
            },
        ]
    );
}
