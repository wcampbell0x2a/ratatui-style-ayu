//! Ayu Dark color palette from [Helix editor](https://github.com/helix-editor/helix/blob/496f0e1b41e6735873c1376ac10a6a722767f328/runtime/themes/ayu_dark.toml)

use ratatui_core::style::Color;

/// Editor background
pub const BACKGROUND: Color = Color::Rgb(0x0F, 0x14, 0x19);

/// Main editor text color
pub const FOREGROUND: Color = Color::Rgb(0xBF, 0xBD, 0xB6);

pub const BLACK: Color = Color::Rgb(0x13, 0x17, 0x21);

pub const SELECTION: Color = Color::Rgb(0x2D, 0x36, 0x40);

pub const COMMENT: Color = Color::Rgb(0x5C, 0x67, 0x73);

/// Keywords, storage types, template expressions
pub const ORANGE: Color = Color::Rgb(0xFF, 0x8F, 0x40);

/// Function names, function calls, tag attributes
pub const YELLOW: Color = Color::Rgb(0xE6, 0xB4, 0x50);

/// String literals, imports/packages, markup headings
pub const GREEN: Color = Color::Rgb(0xAA, 0xD9, 0x4C);

/// Regular expressions, escape characters, blockquotes
pub const CYAN: Color = Color::Rgb(0x73, 0xB8, 0xFF);

/// HTML/XML tags, language variables, CSS properties
pub const BLUE: Color = Color::Rgb(0x59, 0xC2, 0xFF);

/// Named constants, function parameters
pub const MAGENTA: Color = Color::Rgb(0xD2, 0xA6, 0xFF);

/// Member variables, library functions, markup italic/bold
pub const RED: Color = Color::Rgb(0xF0, 0x71, 0x78);
