use fdesk::actions;
use fdesk::errors::{ActionError, ActionErrorKind};
use std::env;
use std::error::Error;

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

    #[cfg(debug_assertions)]
    println!("Input action: {}", action);

    if let Err(e) = exec_action(action, param1) {
        let e = match e.kind() {
            ActionErrorKind::InvalidParams => "Invalid Parameters were inputed into an action",
            ActionErrorKind::MissingParams => "Some essential parameters were missing",
            ActionErrorKind::OpenFile(file, io_err) => &format!(
                "Couldn't open a file [file: {}] because [io error: {}]",
                file, io_err
            ),
            ActionErrorKind::CreateDirectory(dir, io_err) => &format!(
                "Couldn't create a directory [dir: {}] because [io error: {}]",
                dir, io_err
            ),
        };
        eprintln!("Error: {e}");
    }
}

fn exec_action(action: &str, param1: Option<&String>) -> Result<(), ActionError> {
    match action {
        "help" => actions::help(),
        "add_group" => actions::add_group(param1)?,
        _ => actions::unknown_action(action),
    }
    Ok(())
}
