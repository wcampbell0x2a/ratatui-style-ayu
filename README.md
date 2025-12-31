# Ratatui Style Ayu

[<img alt="github" src="https://img.shields.io/badge/github-wcampbell0x2a/ratatui_style_ayu-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/wcampbell0x2a/ratatui-style-ayu)
[<img alt="crates.io" src="https://img.shields.io/crates/v/ratatui-style-ayu.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ratatui-style-ayu)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-ratatui_style_ayu-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/ratatui-style-ayu)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/wcampbell0x2a/ratatui-style-ayu/main.yml?branch=master&style=for-the-badge" height="20">](https://github.com/wcampbell0x2a/ratatui-style-ayu/actions?query=branch%3Amaster)

Ayu color theme for Ratatui applications.

## Usage

Add the following to your `Cargo.toml` file:
```toml
[dependencies]
ratatui-style-ayu = "0.1.0"
```

In your rust code:
```rust
use ratatui_style_ayu::ayu_dark;

// Use the color constants
let bg = ayu_dark::BACKGROUND;
let fg = ayu_dark::FOREGROUND;
```

## Available Themes

- `ayu_dark` - Dark variant of the Ayu theme
- `ayu_light` - Light variant of the Ayu theme
