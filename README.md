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

- [Localyze Sync (lsync)](#localyze-sync-lsync)
  - [Overview](#overview)
  - [Quick Navigation](#quick-navigation)
  - [Installation](#installation)
  - [Setup](#setup)
  - [Usage](#usage)
  - [FAQ](#faq)
    - [🚧 This tool is under active development — updates are coming soon!](#-this-tool-is-under-active-development--updates-are-coming-soon)

---

## Installation

```bash
cargo install lsync
```

Or build from source:

```
git clone https://github.com/kaysman/lsync.git
cd lsync
cargo build --release
```

## Setup

Run the command to setup localyze in your project.

```
lsync setup
```

## Usage

Fetch and Generate:

```
lsync sync
```

- Fetches latest translations from your configured sheet.
- Generates `.arb` files in `lib/l10n/`.
- Runs Dart generator.

## FAQ

What are the requirements?

- `flutter` must be installed and sourced from your PATH.

`lsync` can install and configure these packages automatically:

- `intl`
- `intl_utils`
- `flutter_localizations`

### 🚧 This tool is under active development — updates are coming soon!
