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
    pub unit: Option<u32>,
    pub section_horizontal: Option<u32>,
    pub section_vertical: Option<u32>,
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
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
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
    pub tonal_offset: Option<f32>,
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
    pub font_size: Option<String>,
    pub line_height: Option<String>,
    pub font_weight_regular: Option<u32>,
    pub font_weight_bold: Option<u32>,
    pub font_weight_light: Option<u32>,
    pub font_family: Option<String>,
    pub smoothing: Option<String>,
    pub optimize_speed: Option<bool>,
    pub headings: Headings,
    pub code: Code,
    pub links: Links,
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
    pub font_family: Option<String>,
    pub font_weight: Option<u32>,
    pub line_height: Option<String>,
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
    pub font_size: Option<String>,
    pub font_family: Option<String>,
    pub line_height: Option<u32>,
    pub font_weight: Option<u32>,
    pub color: Option<String>,
    pub background_color: Option<String>,
    pub wrap: Option<bool>,
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
    pub color: String,
    pub visited: String,
    pub hover: String,
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
    pub width: Option<String>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub active_text_color: Option<String>,
    pub group_items: Option<GroupItems>,
    pub level1_items: Option<Level1Items>,
    pub arrow: Arrow,
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
    pub text_transform: Option<String>,
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
    pub text_transform: Option<String>,
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
    pub size: Option<String>,
    pub color: Option<String>,
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
    pub max_height: Option<String>,
    pub max_width: Option<String>,
    pub gutter: Option<String>,
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
    pub background_color: Option<String>,
    pub width: Option<String>,
    pub text_color: Option<String>,
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