use colored::Colorize;

pub fn log_info(msg: &str) {
    println!("{}", format!("{}", msg));
}

pub fn log_success(msg: &str) {
    println!("{}", format!("{}", msg).green());
}

pub fn log_error(msg: &str) {
    eprintln!("{}", format!("{}", msg).red());
}
