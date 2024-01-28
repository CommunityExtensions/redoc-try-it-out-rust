use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug)]
pub struct ThemOptions {
    spacing: Spacing,
    breakpoints: Breakpoints,
    colors: Colors,
    typography: Typography,
    sidebar: Menu,
    logo: Logo,
    right_panel: RightPanel,
}

impl Default for ThemOptions {
    fn default() -> Self {
        Self {
            spacing: Default::default(),
            breakpoints: Default::default(),
            colors: Default::default(),
            typography: Default::default(),
            sidebar: Default::default(),
            logo: Default::default(),
            right_panel: Default::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Spacing {
    unit: Option<u32>,
    section_horizontal: Option<u32>,
    section_vertical: Option<u32>,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            unit: Option::default(),
            section_horizontal: Option::default(),
            section_vertical: Option::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Breakpoints {
    small: Option<String>,
    medium: Option<String>,
    large: Option<String>,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            small: Option::default(),
            medium: Option::default(),
            large: Option::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Colors {
    tonal_offset: Option<f32>,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            tonal_offset: Option::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Typography {
    font_size: Option<String>,
    line_height: Option<String>,
    font_weight_regular: Option<u32>,
    font_weight_bold: Option<u32>,
    font_weight_light: Option<u32>,
    font_family: Option<String>,
    smoothing: Option<String>,
    optimize_speed: Option<bool>,
    headings: Headings,
    code: Code,
    links: Links,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_size: Option::default(),
            line_height: Option::default(),
            font_weight_regular: Option::default(),
            font_weight_bold: Option::default(),
            font_weight_light: Option::default(),
            font_family: Option::default(),
            smoothing: Option::default(),
            optimize_speed: Option::default(),
            headings: Default::default(),
            code: Default::default(),
            links: Default::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Headings {
    font_family: Option<String>,
    font_weight: Option<u32>,
    line_height: Option<String>,
}

impl Default for Headings {
    fn default() -> Self {
        Self {
            font_family: Option::default(),
            font_weight: Option::default(),
            line_height: Option::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Code {
    font_size: Option<String>,
    font_family: Option<String>,
    line_height: Option<u32>,
    font_weight: Option<u32>,
    color: Option<String>,
    background_color: Option<String>,
    wrap: Option<bool>,
}

impl Default for Code {
    fn default() -> Self {
        Self {
            font_size: Option::default(),
            font_family: Option::default(),
            line_height: Option::default(),
            font_weight: Option::default(),
            color: Option::default(),
            background_color: Option::default(),
            wrap: Option::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Links {
    color: String,
    visited: String,
    hover: String,
}

impl Default for Links {
    fn default() -> Self {
        Self {
            color: Default::default(),
            visited: Default::default(),
            hover: Default::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Menu {
    width: Option<String>,
    background_color: Option<String>,
    text_color: Option<String>,
    active_text_color: Option<String>,
    group_items: Option<GroupItems>,
    level1_items: Option<Level1Items>,
    arrow: Arrow,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            width: Option::default(),
            background_color: Option::default(),
            text_color: Option::default(),
            active_text_color: Option::default(),
            group_items: Option::default(),
            level1_items: Option::default(),
            arrow: Default::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GroupItems {
    text_transform: Option<String>,
}

impl Default for GroupItems {
    fn default() -> Self {
        Self {
            text_transform: Option::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Level1Items {
    text_transform: Option<String>,
}

impl Default for Level1Items {
    fn default() -> Self {
        Self {
            text_transform: Option::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Arrow {
    size: Option<String>,
    color: Option<String>,
}

impl Default for Arrow {
    fn default() -> Self {
        Self {
            size: Option::default(),
            color: Option::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Logo {
    max_height: Option<String>,
    max_width: Option<String>,
    gutter: Option<String>,
}

impl Default for Logo {
    fn default() -> Self {
        Self {
            max_height: Option::default(),
            max_width: Option::default(),
            gutter: Option::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug)]
pub struct RightPanel {
    background_color: Option<String>,
    width: Option<String>,
    text_color: Option<String>,
}

impl Default for RightPanel {
    fn default() -> Self {
        Self {
            background_color: Option::default(),
            width: Option::default(),
            text_color: Option::default(),
        }
    }
}
