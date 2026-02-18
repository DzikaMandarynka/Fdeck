use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    result,
};

use crate::errors::ActionError;

type Result<T> = result::Result<T, ActionError>;

pub fn help() {
    println!(
        "
[flash-deck]
syntax:
    f-deck <action> [params]
    <> -> needed
    [] -> optional
actions:
    help: display this screen
    add_card: add a flash card
        params: <card_name> <card_question> <card_answer> <card_group>
    add_group: add a flash group
        params: <group_name>
    review: review flash cards from a group
        params: <group_name>
        "
    )
}

pub fn add_group(group_name: Option<&String>) -> Result<()> {
    let group_name = group_name.ok_or(ActionError::invalid_param())?;

    let file_name = "group.deck";
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .map_err(|_| ActionError::file_read(file_name))?;

    file.write_all((format!("\n{}", group_name)).as_bytes())
        .unwrap();
    Ok(())
}

pub fn unknown_action(action_name: &str) {
    println!("Unknown action [action: {}]", action_name);
    help();
}
