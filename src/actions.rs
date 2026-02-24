use std::{fs::File, io::Write, path::PathBuf, result};

use crate::{
    errors::ActionError,
    f_io::{self},
    paths,
};

type Result<T> = result::Result<T, ActionError>;

pub fn help() {
    println!(
        "
[flash-deck]
syntax:
    f-deck [flags] <action> <action_param_1> .. <action_param_n>
    <> -> needed
    [] -> optional
    (NOTE: for now this app has no flags)
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

    if !f_io::is_save_present() {
        f_io::create_save()?;
    }

    let dir_path = save_path.join(group_name);

    if dir_path.exists() {
        println!(
            "group {} is already present, do you wish to overwrite it [y/n]?",
            group_name
        );

        let response = f_io::request_input()?.trim().to_lowercase();

        if response == "y" {
            f_io::overwrite_dir(&dir_path)?
        }
    } else {
        f_io::create_dir(&dir_path)?;
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
            let response = f_io::request_input()?.trim().to_lowercase();
            if response == "y" {
                f_io::remove_file(&card_path)?;
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
        return Err(ActionError::invalid_group());
    }

    let mut correct_counter = 0;
    let mut incorrect_counter = 0;

    let files = f_io::get_files(&group_path)?;
    for file in files {
        let content = f_io::read_file_to_string(&file)?;
        let content = content.split(',');

        let mut values = Vec::new();

        for value in content {
            values.push(value);
        }

        let card_name = values[0];
        let card_question = values[1];
        let card_answer = values[2];

        println!("card: {}\nquestion: {}\n", card_name, card_question);
        let user_answer = f_io::request_input()?.trim().to_lowercase();

        if !(user_answer == card_answer.to_lowercase()) {
            println!("Your answer: {}\nCard answer: {}", user_answer, card_answer);
            println!("Where you correct? [y/n]");

            let user_answer = f_io::request_input()?.trim().to_lowercase();

            if user_answer == "y" {
                correct_counter += 1;
                println!("Good job!")
            } else {
                incorrect_counter += 1;
            }
        } else {
            println!("Your answer was correct, it was {}", card_answer);
            correct_counter += 1;
        }
        println!();
    }
    println!(
        "Correct answers: {} \nIncorrect answers: {}",
        correct_counter, incorrect_counter
    );

    Ok(())
}
