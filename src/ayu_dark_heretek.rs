//! Ayu Dark color palette used in [heretek](https://github.com/wcampbell0x2a/heretek/blob/master/src/ui/mod.rs)

use ratatui_core::style::Color;

/// Editor background
/// `DARK_GRAY`
pub const BACKGROUND: Color = Color::Rgb(0x20, 0x27, 0x34);

/// Main editor text color
/// `GRAY_FG`
pub const FOREGROUND: Color = Color::Rgb(0x64, 0x64, 0x64);

pub const SELECTION: Color = Color::Rgb(0x27, 0x37, 0x47);

pub const COMMENT: Color = Color::Rgb(0x62, 0x6A, 0x73);

/// Keywords, storage types, template expressions
/// `ASM_COLOR`
pub const ORANGE: Color = Color::Rgb(0xFF, 0x8F, 0x40);

/// Function names, function calls, tag attributes
/// `STRING_COLOR`
pub const YELLOW: Color = Color::Rgb(0xE6, 0xB4, 0x50);

/// String literals, imports/packages, markup headings
/// `HEAP_COLOR`
pub const GREEN: Color = Color::Rgb(0xAA, 0xD9, 0x4C);

/// Regular expressions, escape characters, blockquotes
pub const CYAN: Color = Color::Rgb(0x95, 0xE6, 0xCB);

/// HTML/XML tags, language variables, CSS properties
pub const BLUE: Color = Color::Rgb(0x59, 0xC2, 0xFF);

/// Named constants, function parameters
/// STACK_COLOR
pub const PURPLE: Color = Color::Rgb(0xD2, 0xA6, 0xFF);

/// Member variables, library functions, markup italic/bold
/// TEXT_COLOR
pub const RED: Color = Color::Rgb(0xFF, 0x33, 0x33);
