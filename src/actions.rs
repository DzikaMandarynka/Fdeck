use std::{
    fs::{self, File},
    io::Write,
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

pub fn add_card(
    card_name: Option<&String>,
    card_question: Option<&String>,
    card_answer: Option<&String>,
    card_group: Option<&String>,
) -> Result<()> {
    let save_path = paths::get_save_path();

    let card_name = card_name.ok_or(ActionError::missing_param())?;
    let card_name = format!("{}.deck", card_name);
    let card_question = card_question.ok_or(ActionError::missing_param())?;
    let card_answer = card_answer.ok_or(ActionError::missing_param())?;
    let card_group = card_group.ok_or(ActionError::missing_param())?;

    if !io_utility::is_save_present() {
        io_utility::create_save().map_err(|e| ActionError::create_dir(&save_path, e))?;
    }
    let group_path = save_path.join(card_group);
    if !group_path.exists() {
        Err(ActionError::invalid_group())
    } else {
        let card_path = group_path.join(&card_name);
        if card_path.exists() {
            println!(
                "Card \"{}\" already exists in group \"{}\", do you wish to overwrite it? [y/n]",
                card_name, card_group
            );
            let response = io_utility::request_input()
                .map_err(|e| ActionError::read_input(e))?
                .trim()
                .to_lowercase();
            if response == "y" {
                fs::remove_file(&card_path).map_err(|e| ActionError::remove_file(&card_path, e))?;
            } else {
                return Ok(());
            }
        }
        let mut file =
            File::create_new(&card_path).map_err(|e| ActionError::create_file(&card_path, e))?;

        file.write_all(format!("{},{},{}", &card_name, card_question, card_answer).as_bytes())
            .map_err(|e| ActionError::write_file(&card_path, e))?;
        println!("card created at {:?}", card_path);

        Ok(())
    }
}
