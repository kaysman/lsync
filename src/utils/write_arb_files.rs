use crate::utils::logging::*;
use serde_json::to_string_pretty;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn write_arb_files(header: &[String], translations: &[Vec<String>]) {
    let out_dir = Path::new("lib/l10n");

    if let Err(e) = fs::create_dir_all(out_dir) {
        log_error(&format!("Failed to create output directory: {}", e));
        return;
    }

    let languages = &header[1..];

    let mut lang_maps: HashMap<&str, HashMap<String, String>> = HashMap::new();

    for lang in languages {
        lang_maps.insert(lang.as_str(), HashMap::new());
    }

    for row in translations {
        if row.is_empty() || row.len() < languages.len() + 1 {
            continue;
        }

        let key = &row[0];

        for (i, lang) in languages.iter().enumerate() {
            let text = row.get(i + 1).map_or("", |v| v);
            if let Some(map) = lang_maps.get_mut(lang.as_str()) {
                map.insert(key.clone(), text.to_string());
            }
        }
    }

    for (lang, map) in lang_maps {
        let filepath = out_dir.join(format!("intl_{}.arb", lang.to_lowercase()));
        match to_string_pretty(&map) {
            Ok(json) => {
                if let Err(e) = fs::write(&filepath, json) {
                    log_error(&format!("Failed to write {}: {}", filepath.display(), e));
                } else {
                    log_info(&format!("Generated {}", filepath.display()));
                }
            }
            Err(e) => log_error(&format!("Failed to serialize {}: {}", lang, e)),
        }
    }
}
