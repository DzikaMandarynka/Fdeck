use std::{
    fs::{self},
    path::{Path, PathBuf},
    result,
};

use crate::{
    errors::ActionError,
    io_utility::{self},
    paths,
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
    let save_path: PathBuf = paths::get_save_path();

    let group_name = group_name.ok_or(ActionError::missing_param())?;

    if !io_utility::is_save_present() {
        io_utility::create_save().map_err(|e| ActionError::create_dir(&save_path.clone(), e))?;
    }

    let dir_path = save_path.join(group_name);

    let group_created = if dir_path.exists() {
        println!(
            "WARNING {} is already present, do you want to overwrite it [y/n]?",
            group_name
        );

        let response = io_utility::request_input().map_err(|e| ActionError::read_input(e))?;

        if response.trim().to_lowercase() == "y" {
            io_utility::overwrite_dir(Path::new(&dir_path))
                .map_err(|e| ActionError::overwrite_dir(&dir_path, e))?;
            true
        } else {
            false
        }
    } else {
        fs::create_dir(&dir_path).map_err(|e| ActionError::create_dir(&dir_path, e))?;
        true
    };

    if group_created {
        println!("Successfully created group: {}", group_name);
    }

    Ok(())
}

pub fn unknown_action(action_name: &str) {
    println!("Unknown action [action: {}]", action_name);
    help();
}
