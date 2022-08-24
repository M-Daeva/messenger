use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use messenger::messages::{
    execute::ExecuteMsg,
    instantiate::InstantiateMsg,
    query::QueryMsg,
    response::{MessageResponse, MessagesResponse},
};
use messenger::state::{Bank, Book, Message, User};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(Bank), &out_dir);
    export_schema(&schema_for!(Book), &out_dir);
    export_schema(&schema_for!(Message), &out_dir);
    export_schema(&schema_for!(User), &out_dir);
    export_schema(&schema_for!(MessageResponse), &out_dir);
    export_schema(&schema_for!(MessagesResponse), &out_dir);
}
