use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Target {
    #[serde(rename = "flutter")]
    Flutter,
    #[serde(rename = "react")]
    React,
}

impl Default for Target {
    fn default() -> Self {
        Target::Flutter
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub sheet_id: String,
    pub start_cell: String,
    pub end_column: String,
    #[serde(default)]
    pub target: Target,
}
