use crate::google_sheet_api_client::GoogleSheetApiClient;
use crate::models::config::Config;
use crate::utils::logging::*;
use crate::utils::prompt::{
    prompt_ask_ending_column, prompt_ask_sheet_id, prompt_ask_starting_cell, prompt_yes_no,
};
use crate::utils::required_files as files;
use files::{
    config_exists, config_path, create_config_file, credentials_exists,
    ensure_gitignore_has_credentials_path, ensure_localyze_dir_exists,
};

pub struct Setup {}

impl Setup {
    pub async fn run() {
        if let Err(e) = ensure_localyze_dir_exists() {
            log_error(&format!("Failed to create .localyze directory: {}", e));
            std::process::exit(1);
        }

        if config_exists() && !credentials_exists() {
            let sheet_id = get_sheet_id_from_config();

            let proceed_login =
                prompt_yes_no(&format!("Login to access Google Sheet ID: {}?", sheet_id));

            if !proceed_login {
                log_error("Login cancelled.");
                std::process::exit(0);
            }

            if let Err(e) = ensure_gitignore_has_credentials_path() {
                log_error(&format!("Failed to update .gitignore: {}", e));
                std::process::exit(1);
            }

            GoogleSheetApiClient::get_client().await;

            log_success("Setup done! You can run `lsync sync` anytime to fetch translations.");

            std::process::exit(0);
        }

        if config_exists() {
            let should_override = prompt_yes_no("Override current configuration?");
            if !should_override {
                std::process::exit(0);
            }
        }

        let sheet_id = prompt_ask_sheet_id();

        if sheet_id.clone().is_empty() {
            log_error("Sheet ID cannot be empty. Please enter a valid Google Sheet ID.");
            std::process::exit(1);
        }

        let start_cell = prompt_ask_starting_cell();

        if start_cell.is_empty() {
            log_error("KEY starting cell cannot be empty.");
            std::process::exit(1);
        }

        let end_column = prompt_ask_ending_column();

        if end_column.is_empty() {
            log_error("Ending column can not empty.");
            std::process::exit(1);
        }

        let config = Config {
            sheet_id: sheet_id.clone(),
            start_cell,
            end_column,
        };

        match serde_json::to_string_pretty(&config) {
            Ok(json) => {
                if let Err(e) = create_config_file(&json) {
                    log_error(&format!("Failed to write config file: {}", e));
                } else {
                    log_info("Config file created successfully.");
                }
            }
            Err(e) => {
                log_error(&format!("Failed to serialize config: {}", e));
            }
        }

        if let Err(e) = ensure_gitignore_has_credentials_path() {
            log_error(&format!(
                "Failed to add credentials file to .gitignore: {}",
                e
            ));
            std::process::exit(1);
        }

        GoogleSheetApiClient::get_client().await;

        log_success("Setup done! You can run `lsync sync` anytime to fetch translations.");

        std::process::exit(0);
    }
}

fn get_sheet_id_from_config() -> String {
    let contents =
        std::fs::read_to_string(config_path()).expect("Unable to get sheet_id from config");

    let json_value = serde_json::from_str::<serde_json::Value>(&contents)
        .expect("Error while parsing config into json");

    let result = json_value
        .get("sheet_id")
        .and_then(|v| v.as_str())
        .expect("Unable to parse sheet id");

    result.to_string()
}
