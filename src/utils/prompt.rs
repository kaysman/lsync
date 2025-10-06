use colored::*;
use crate::models::config::Target;
use std::io::{self, Write};

pub fn prompt_yes_no(question: &str) -> bool {
    println!("{} [Y/n]", question.bold().cyan());
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        matches!(input.trim().to_lowercase().as_str(), "y" | "yes" | "")
    } else {
        false
    }
}

pub fn prompt_ask_sheet_id() -> String {
    println!("{}", format!("Enter Google Sheet ID: ").bold().cyan());
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to retrieve sheet id");

    input.trim().to_string()
}

pub fn prompt_ask_starting_cell() -> String {
    println!(
        "{}",
        format!("Enter selected sheet starting KEY cell. Options: A1, A2, B2, C2")
            .bold()
            .cyan()
    );
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to retrieve starting KEY cell");

    input.trim().to_string()
}

pub fn prompt_ask_ending_column() -> String {
    println!(
        "{}",
        format!("Enter ending column ('C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K'):")
            .bold()
            .cyan()
    );
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to retrieve ending column");

    input.trim().to_string()
}

pub fn prompt_ask_target() -> Target {
    println!(
        "{}",
        "Select project type: [f]lutter / [r]eact".bold().cyan()
    );
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return Target::Flutter;
    }

    match input.trim().to_lowercase().as_str() {
        "f" | "flutter" => Target::Flutter,
        "r" | "react" | "next" | "nextjs" => Target::React,
        _ => Target::Flutter,
    }
}
