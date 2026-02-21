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
        io_utility::create_save()?;
    }

    let dir_path = save_path.join(group_name);

    if dir_path.exists() {
        println!(
            "group {} is already present, do you wish to overwrite it [y/n]?",
            group_name
        );

        let response = io_utility::request_input()?.trim().to_lowercase();

        if response == "y" {
            io_utility::overwrite_dir(&dir_path)?
        }
    } else {
        fs::create_dir(&dir_path).map_err(|e| ActionError::create_dir(&dir_path, e))?;
    };

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
    let card_question = card_question.ok_or(ActionError::missing_param())?;
    let card_answer = card_answer.ok_or(ActionError::missing_param())?;
    let card_group = card_group.ok_or(ActionError::missing_param())?;
    let group_path = save_path.join(card_group);

    if !group_path.exists() {
        Err(ActionError::invalid_group())
    } else {
        let card_path = group_path.join(&format!("{}.deck", card_name));
        if card_path.exists() {
            println!(
                "Card \"{}\" already exists in group \"{}\", do you wish to overwrite it? [y/n]",
                card_name, card_group
            );
            let response = io_utility::request_input()?.trim().to_lowercase();
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

        Ok(())
    }
}

pub fn review(group_name: Option<&String>) -> Result<()> {
    let save_path: PathBuf = paths::get_save_path();

    let group_name = group_name.ok_or(ActionError::missing_param())?;
    let group_path = save_path.join(group_name);

    if !group_path.exists() {
        Err(ActionError::invalid_group())
    } else {
        let files = io_utility::get_files(&group_path)?;
        for file in files {
            let content =
                fs::read_to_string(file).map_err(|e| ActionError::read_file(&group_path, e))?;
            let content = content.split(',');

            let mut values = Vec::new();

            for value in content {
                values.push(value);
            }

            let card_name = values[0];
            let card_question = values[1];
            let card_answer = values[2];

            println!("card: {}\nquestion: {}", card_name, card_question);
            let user_answer = io_utility::request_input()?.trim().to_lowercase();

            if !(user_answer == card_answer.to_lowercase()) {
                println!("Your answer: {}\nCard answer: {}", user_answer, card_answer);
                println!("Where you correct? [y/n]");
                let user_answer = io_utility::request_input()?.trim().to_lowercase();

                if user_answer == "y" {
                    println!("Good job!")
                }
            } else {
                println!("Your answer was correct, it was {}", card_answer);
            }
        }
        Ok(())
    }
}
