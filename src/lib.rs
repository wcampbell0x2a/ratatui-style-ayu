//! Ayu color theme for Ratatui applications.
//!
//! ## Available Themes
//!
//! - [`ayu_dark`] - Dark variant of the Ayu theme from [ayu-theme/ayu-colors](https://github.com/ayu-theme/ayu-colors)
//! - [`ayu_dark_helix`] - Ayu Dark variant from [Helix editor](https://github.com/helix-editor/helix)
//! - [`ayu_dark_heretek`] - Custom Ayu Dark variant from [heretek](https://github.com/wcampbell0x2a/heretek)
//! - [`ayu_light`] - Light variant of the Ayu theme
//!
//! ## Comparison of Dark Variants
//!
//! <style>
//! .color-swatch { display: inline-block; width: 15px; height: 15px; border: 1px solid #888; vertical-align: middle; margin-right: 4px; }
//! </style>
//!
//! | Color | Official | Helix | Heretek |
//! |-------|----------|-------|---------|
//! | **Background** | <span class="color-swatch" style="background-color:#10141c"></span> `#10141c` | <span class="color-swatch" style="background-color:#0f1419"></span> `#0f1419` | <span class="color-swatch" style="background-color:#202734"></span> `#202734` |
//! | **Foreground** | <span class="color-swatch" style="background-color:#bfbdb6"></span> `#bfbdb6` | <span class="color-swatch" style="background-color:#bfbdb6"></span> `#bfbdb6` | <span class="color-swatch" style="background-color:#646464"></span> `#646464` |
//! | **Black** | - | <span class="color-swatch" style="background-color:#131721"></span> `#131721` | - |
//! | **Selection** | <span class="color-swatch" style="background-color:#273747"></span> `#273747` | <span class="color-swatch" style="background-color:#2d3640"></span> `#2d3640` | <span class="color-swatch" style="background-color:#273747"></span> `#273747` |
//! | **Comment** | <span class="color-swatch" style="background-color:#626a73"></span> `#626a73` | <span class="color-swatch" style="background-color:#5c6773"></span> `#5c6773` | <span class="color-swatch" style="background-color:#626a73"></span> `#626a73` |
//! | **Orange** | <span class="color-swatch" style="background-color:#ff8f40"></span> `#ff8f40` | <span class="color-swatch" style="background-color:#ff8f40"></span> `#ff8f40` | <span class="color-swatch" style="background-color:#ff8f40"></span> `#ff8f40` |
//! | **Yellow** | <span class="color-swatch" style="background-color:#ffb454"></span> `#ffb454` | <span class="color-swatch" style="background-color:#e6b450"></span> `#e6b450` | <span class="color-swatch" style="background-color:#e6b450"></span> `#e6b450` |
//! | **Green** | <span class="color-swatch" style="background-color:#aad94c"></span> `#aad94c` | <span class="color-swatch" style="background-color:#aad94c"></span> `#aad94c` | <span class="color-swatch" style="background-color:#aad94c"></span> `#aad94c` |
//! | **Cyan** | <span class="color-swatch" style="background-color:#95e6cb"></span> `#95e6cb` | <span class="color-swatch" style="background-color:#73b8ff"></span> `#73b8ff` | <span class="color-swatch" style="background-color:#95e6cb"></span> `#95e6cb` |
//! | **Blue** | <span class="color-swatch" style="background-color:#39bae6"></span> `#39bae6` | <span class="color-swatch" style="background-color:#59c2ff"></span> `#59c2ff` | <span class="color-swatch" style="background-color:#59c2ff"></span> `#59c2ff` |
//! | **Purple/Magenta** | <span class="color-swatch" style="background-color:#d2a6ff"></span> `#d2a6ff` | <span class="color-swatch" style="background-color:#d2a6ff"></span> `#d2a6ff` | <span class="color-swatch" style="background-color:#d2a6ff"></span> `#d2a6ff` |
//! | **Red** | <span class="color-swatch" style="background-color:#f07178"></span> `#f07178` | <span class="color-swatch" style="background-color:#f07178"></span> `#f07178` | <span class="color-swatch" style="background-color:#ff3333"></span> `#ff3333` |
//!
//! **Key differences:**
//! - **Official**: True cyan with greenish tint (<span class="color-swatch" style="background-color:#95e6cb"></span> `#95e6cb`), brighter yellow (<span class="color-swatch" style="background-color:#ffb454"></span> `#ffb454`), darkest background (<span class="color-swatch" style="background-color:#10141c"></span> `#10141c`)
//! - **Helix**: Bluer cyan and blue tones, has additional BLACK constant, slightly lighter background (<span class="color-swatch" style="background-color:#0f1419"></span> `#0f1419`)
//! - **Heretek**: Much lighter background (<span class="color-swatch" style="background-color:#202734"></span> `#202734`), significantly dimmer foreground (<span class="color-swatch" style="background-color:#646464"></span> `#646464`), brighter red (<span class="color-swatch" style="background-color:#ff3333"></span> `#ff3333`)

pub mod ayu_dark;
pub mod ayu_dark_helix;
pub mod ayu_dark_heretek;
pub mod ayu_light;
