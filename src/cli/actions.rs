use seahorse::Context;
use super::commands::*;

pub fn init_action(_c: &Context) {
    if let Err(e) = init::init_pollen() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

pub fn parse_action(c: &Context) {
    if let Err(e) = parse::parse_config(c) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

pub fn list_action(c: &Context) {
    if let Err(e) = list::list_entries(c) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

pub fn validate_action(c: &Context) {
    if let Err(e) = validate::validate_config(c) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

pub fn gather_action(c: &Context) {
    if let Err(e) = gather::gather_files(c) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

pub fn scatter_action(c: &Context) {
    if let Err(e) = scatter::scatter_files(c) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

pub fn undo_action(c: &Context) {
    if let Err(e) = undo::undo_last_operation(c) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

pub fn git_action(c: &Context) {
    if let Err(e) = git::handle_git_command(c) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

pub fn cd_action(c: &Context) {
    if let Err(e) = cd::change_to_pollen_dir(c) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

pub fn config_action(c: &Context) {
    if let Err(e) = config::show_config(c) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
