# Localyze Sync (lsync)

A blazing-fast CLI tool to sync translations from Google Sheets and generate `.arb` files for your Flutter project.

---

## Overview

`lsync` simplifies localization by:

- Fetching translations directly from a Google Sheet
- Generating `.arb` files (`intl_en.arb`, `intl_de.arb`, etc.)
- Running Dart's `intl_utils` to generate `*.g.dart` files
- Keeping your `lib/l10n/` folder up-to-date
- Fully configurable, no manual editing of `.arb` files required

---

## Quick Navigation

- [📦 Installation](#installation)
- [⚙️ Setup](#setup)
- [🔁 Usage](#usage)
- [🌍 Google Sheets Integration](#google-sheets-integration)
- [🛠 Configuration](#configuration)
- [❓ FAQ](#faq)

---

## Installation

```bash
cargo install lsync
```

Or build from source:

```
git clone https://github.com/YOUR_ORG/lsync.git
cd lsync
cargo build --release
```
