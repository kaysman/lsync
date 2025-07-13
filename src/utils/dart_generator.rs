use crate::utils::logging::*;
use std::process::Command;

pub fn run_flutter_generator() {
    let flutter_installed = Command::new("flutter")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !flutter_installed {
        log_error("Flutter is not installed or not in PATH.");
        return;
    }

    let pubspec_path = "pubspec.yaml";
    if !std::path::Path::new(pubspec_path).exists() {
        log_error("No `pubspec.yaml` found. Please make sure you're in the root directory of a Flutter project!");
        std::process::exit(1);
    }

    add_flutter_intl_to_pubspec(&pubspec_path);

    add_required_packages();

    let gen_status = Command::new("dart")
        .args(["run", "intl_utils:generate"])
        .status();

    match gen_status {
        Ok(s) if s.success() => {
            log_success("Done! Use the generated Lsync class in your Flutter app.")
        }
        Ok(s) => log_error(&format!("Dart generator exited with code: {}", s)),
        Err(e) => log_error(&format!("Failed to run generator: {}", e)),
    }
}

fn add_flutter_intl_to_pubspec(pubspec_path: &str) {
    let pubspec = std::fs::read_to_string(pubspec_path).expect("Failed to read pubspec.yaml");

    if pubspec.contains("flutter_intl:") {
        return;
    }

    log_info("Adding flutter_intl config to pubspec.yaml...");

    let append = r#"
flutter_intl:
  enabled: true
  class_name: "Lsync"
  arb_dir: lib/l10n
"#;

    if let Err(e) = std::fs::OpenOptions::new()
        .append(true)
        .open(pubspec_path)
        .and_then(|mut f| std::io::Write::write_all(&mut f, append.as_bytes()))
    {
        log_error(&format!("Failed to write flutter_intl config: {}", e));
        std::process::exit(0);
    } else {
        log_info("Added flutter_intl configuration.");
    }
}

fn add_required_packages() {
    log_info("📦 Adding packages to pubspec.yaml...");

    // -- CHECK REQUIRED PACKAGES --

    // flutter_localizations
    Command::new("flutter")
        .args(["pub", "add", "flutter_localizations", "--sdk=flutter"])
        .status()
        .expect("Failed to add `flutter_localizations` package.");

    // intl
    Command::new("flutter")
        .args(["pub", "add", "intl"])
        .status()
        .expect("Failed to add `intl` package.");

    // intl_utils
    Command::new("flutter")
        .args(["pub", "add", "intl_utils"])
        .status()
        .expect("Failed to add `intl_utils` package.");

    log_info("Required packages have been added to pubspec.yaml.");
}
