use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    result,
};

use crate::{
    constants::SAVE_DIRECTORY,
    errors::ActionError,
    io_utility::{create_save, is_save_present},
};

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

    if !is_save_present() {
        create_save().map_err(|e| ActionError::create_dir(SAVE_DIRECTORY, e))?;
    }

    let file_path = format!("{}{}", SAVE_DIRECTORY, "group.deck");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_path)
        .map_err(|e| ActionError::file_read(file_path.as_str(), e))?;

    file.write_all((format!("\n{}", group_name)).as_bytes())
        .unwrap();

    Ok(())
}

pub fn unknown_action(action_name: &str) {
    println!("Unknown action [action: {}]", action_name);
    help();
}
