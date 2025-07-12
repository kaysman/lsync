use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub sheet_id: String,
    pub start_cell: String,
    pub end_column: String,
}
