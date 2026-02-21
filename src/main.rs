use fdesk::actions;
use fdesk::errors::{ActionError, ActionErrorKind, FileSystemCause};
use std::env;

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let args: Vec<_> = args.collect();

    #[cfg(debug_assertions)]
    println!("Args: {:?}", args);

    if args.is_empty() {
        actions::help();
        return;
    }

    let action = &args[0];
    let param1 = args.get(1);
    let param2 = args.get(2);
    let param3 = args.get(3);
    let param4 = args.get(4);

    #[cfg(debug_assertions)]
    {
        use fdesk::paths;
        println!("Save directory: {:?}", paths::get_save_path());
        println!("Input action: {}", action);
    }

    if let Err(e) = exec_action(action, param1, param2, param3, param4) {
        let e = match e.kind() {
            ActionErrorKind::InvalidParams => "Invalid Parameters were inputed into an action",
            ActionErrorKind::MissingParams => "Some essential parameters were missing",
            ActionErrorKind::InvalidGroup => "Specified group doesn't exist",
            ActionErrorKind::ReadInput(io_err) => {
                &format!("Couldn't read user input because [io error: {}]", io_err)
            }
            ActionErrorKind::FileSystem(cause, path, io_err) => match cause {
                FileSystemCause::RemoveFile => &format!(
                    "Couldn't remove a file [file: {:?}] because [io error: {}]",
                    path, io_err
                ),
                FileSystemCause::CreateFile => &format!(
                    "Couldn't create a file [file: {:?}] because [io error: {}]",
                    path, io_err
                ),
                FileSystemCause::OpenFile => &format!(
                    "Couldn't open a file [file: {:?}] because [io error: {}]",
                    path, io_err
                ),
                FileSystemCause::WriteFile => &format!(
                    "Couldn't write to a file [file: {:?}] because [io error: {}]",
                    path, io_err
                ),
                FileSystemCause::ReadFile => &format!(
                    "Couldn't read a file [file: {:?}] because [io error: {}]",
                    path, io_err
                ),
                FileSystemCause::CreateDirectory => &format!(
                    "Couldn't create a directory [dir: {:?}] because [io error: {}]",
                    path, io_err
                ),
                FileSystemCause::OverwriteDirectory => &format!(
                    "Couldn't overwrite a directory [dir: {:?}] because [io error: {}]",
                    path, io_err
                ),
            },
        };
        eprintln!("Error: {e}");
    } else {
        println!("Operation completed successfully")
    }
}

fn exec_action(
    action: &str,
    param1: Option<&String>,
    param2: Option<&String>,
    param3: Option<&String>,
    param4: Option<&String>,
) -> Result<(), ActionError> {
    match action {
        "help" => actions::help(),
        "add_group" => actions::add_group(param1)?,
        "add_card" => actions::add_card(param1, param2, param3, param4)?,
        "review" => actions::review(param1)?,
        _ => actions::unknown_action(action),
    }
    Ok(())
}
