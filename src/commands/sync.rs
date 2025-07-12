use crate::google_sheet_api_client::GoogleSheetApiClient;
use crate::models::config::Config;
use crate::sheets4::api::ValueRange;
use crate::utils::dart_generator::run_flutter_generator;
use crate::utils::logging::*;
use crate::utils::required_files::{config_exists, open_config_file};
use crate::utils::write_arb_files::*;

pub struct Sync {}

impl Sync {
    pub async fn run() {
        if !config_exists() {
            log_error("Project has not been initialized: run `lsync setup`");
            std::process::exit(1);
        }

        let config = open_config_file();

        let sheet_api_client = GoogleSheetApiClient::get_client().await;

        let range = format!("{}:{}", config.start_cell, config.end_column);

        match sheet_api_client
            .spreadsheets()
            .values_get(&config.sheet_id, &range)
            .doit()
            .await
        {
            Ok((_, result)) => {
                process_sheets(result, &config);
            }
            Err(e) => {
                log_error(&format!("Failed to fetch sheet: {}", e));
            }
        }

        run_flutter_generator();
    }
}

fn process_sheets(value_range: ValueRange, config: &Config) {
    let values: Option<Vec<Vec<String>>> = value_range.values.map(|rows| {
        rows.into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect()
            })
            .collect()
    });
    match values {
        Some(rows) if !rows.is_empty() => {
            let start_row_index = parse_row_index(&config.start_cell);
            let header = &rows[start_row_index];
            let translations = &rows[start_row_index + 1..];

            write_arb_files(header, translations);
        }
        Some(_) | None => {
            log_error("Sheet is empty or malformed.");
        }
    }
}

fn parse_row_index(cell: &str) -> usize {
    let digits: String = cell.chars().filter(|c| c.is_ascii_digit()).collect();
    digits.parse::<usize>().unwrap_or(1).saturating_sub(1)
}
