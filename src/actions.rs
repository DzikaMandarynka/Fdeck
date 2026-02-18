use std::{
    fs::{self, OpenOptions},
    io::{self, Read, Write},
    path::Path,
    result,
};

use crate::{
    constants::SAVE_DIRECTORY,
    errors::ActionError,
    io_utility::{self, create_save, is_save_present},
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
    let group_created: bool;
    let group_name = group_name.ok_or(ActionError::invalid_param())?;

    if !is_save_present() {
        create_save().map_err(|e| ActionError::create_dir(SAVE_DIRECTORY, e))?;
    }

    let dir_path = format!("{}{}", SAVE_DIRECTORY, group_name);

    if Path::new(&dir_path).exists() {
        println!(
            "WARNING {} is already present, do you want to overwrite it [y/n]?",
            group_name
        );

        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .map_err(|e| ActionError::read_input(e))?;

        if response.trim().to_lowercase() == "y" {
            io_utility::overwrite_dir(Path::new(&dir_path))
                .map_err(|e| ActionError::overwrite_dir(group_name, e))?;
            group_created = true;
        } else {
            group_created = false
        }
    } else {
        fs::create_dir(&dir_path).map_err(|e| ActionError::create_dir(group_name, e))?;
        group_created = true;
    }

    if group_created {
        println!("Successfully created group: {}", group_name);
    }

    Ok(())
}

pub fn unknown_action(action_name: &str) {
    println!("Unknown action [action: {}]", action_name);
    help();
}
