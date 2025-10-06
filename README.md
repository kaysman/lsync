# Localyze Sync (lsync)

A blazing-fast CLI tool to sync translations from Google Sheets and generate translation files for Flutter and React/Next.js projects.

---

## Overview

`lsync` simplifies localization by:

- Fetching translations directly from a Google Sheet
- Flutter: Generates `.arb` files (`intl_en.arb`, `intl_de.arb`, etc.)
- Flutter: Runs Dart's `intl_utils` to generate `*.g.dart` files
- React/Next.js: Generates JSON locale files (e.g., `public/locales/en/common.json`)
- Keeps your localization folders up-to-date
- Fully configurable, no manual editing of translation files required

---

## Quick Navigation

- [Localyze Sync (lsync)](#localyze-sync-lsync)
  - [Overview](#overview)
  - [Quick Navigation](#quick-navigation)
  - [Installation](#installation)
    - [🍺 Homebrew Tap for `lsync`](#-homebrew-tap-for-lsync)
    - [Manual Installation](#manual-installation)
  - [Setup](#setup)
  - [Usage](#usage)
  - [React/Next.js](#reactnextjs)
  - [FAQ](#faq)
    - [🚧 This tool is under active development — updates are coming soon!](#-this-tool-is-under-active-development--updates-are-coming-soon)

---

## Installation

### 🍺 Homebrew Tap for `lsync`

Install via Homebrew:

```bash
brew tap kaysman/lsync
brew install lsync
```

### Manual Installation

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

During setup, you'll be asked which platform you are using:

- Flutter
- React/Next.js

This choice determines the output format and folder structure for your translations.

## Usage

Fetch and Generate:

```
lsync sync
```

- Fetches latest translations from your configured sheet
- Flutter: Generates `.arb` files in `lib/l10n/` and runs Dart generator
- React/Next.js: Generates JSON files in `public/locales/<lang>/common.json`

How to use the generated Lsync class in your Flutter app?

1. Import and use Lsync class in root of your app:

```
return MaterialApp(
  localizationsDelegates: [
    Lsync.delegate,
    GlobalCupertinoLocalizations.delegate,
    GlobalMaterialLocalizations.delegate,
    GlobalWidgetsLocalizations.delegate,
  ],
  supportedLocales: Lsync.delegate.supportedLocales,
  // ...
);

```

2. Use in any widget:

```
Text(
  Lsync.of(context).push_count_text
),
```

Replace "push_count_text" with your actual translation key from the sheet.

## React/Next.js

When you choose React/Next.js during `lsync setup`, `lsync sync` will generate locale files in the following structure:

```
public/
  locales/
    en/
      common.json
    de/
      common.json
    ...
```

Each `common.json` contains a flat JSON object mapping keys to translated strings, using language codes derived from your sheet's header row (e.g., `en`, `de`, `fr`).

You can use these files with a variety of i18n libraries. Two common options:

- next-i18next: Configure `next-i18next.config.js` with `localePath: 'public/locales'` and `defaultNS: 'common'`.
- next-intl or other libraries: Load `public/locales/<lang>/common.json` manually in your routing/layouts.

Example (next-i18next):

```
// next-i18next.config.js
module.exports = {
  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'de'],
  },
  localePath: 'public/locales',
  defaultNS: 'common',
};
```

Then in a component:

```
import { useTranslation } from 'next-i18next';

export default function Example() {
  const { t } = useTranslation('common');
  return <div>{t('push_count_text')}</div>;
}
```

## FAQ

What are the requirements?

- Flutter: `flutter` must be installed and available on your PATH.
- React/Next.js: No additional tooling is required. `lsync` writes JSON files only.

`lsync` can install and configure these packages automatically:

- `intl` (Flutter)
- `intl_utils` (Flutter)
- `flutter_localizations` (Flutter)

### 🚧 This tool is under active development — updates are coming soon!

Planned/Recent updates:

- React/Next.js JSON output support
- Automatic platform prompt during `setup`
- Safer defaults and improved sheet parsing
