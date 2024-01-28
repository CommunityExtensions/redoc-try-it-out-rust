use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ThemeOptions {
    spacing: Option<Spacing>,
    breakpoints: Option<Breakpoints>,
    colors: Option<Colors>,
    typography: Option<Typography>,
    sidebar: Option<Menu>,
    logo: Option<Logo>,
    right_panel: Option<RightPanel>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Spacing {
    unit: Option<u32>,
    section_horizontal: Option<u32>,
    section_vertical: Option<u32>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Breakpoints {
    small: Option<String>,
    medium: Option<String>,
    large: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Colors {
    tonal_offset: Option<f32>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Typography {
    font_size: Option<String>,
    line_height: Option<String>,
    font_weight_regular: Option<u32>,
    font_weight_bold: Option<u32>,
    font_weight_light: Option<u32>,
    font_family: Option<String>,
    smoothing: Option<String>,
    optimize_speed: Option<bool>,
    headings: Option<Headings>,
    code: Option<Code>,
    links: Option<Links>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Headings {
    font_family: Option<String>,
    font_weight: Option<u32>,
    line_height: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Code {
    font_size: Option<String>,
    font_family: Option<String>,
    line_height: Option<u32>,
    font_weight: Option<u32>,
    color: Option<String>,
    background_color: Option<String>,
    wrap: Option<bool>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Links {
    color: Option<String>,
    visited: Option<String>,
    hover: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Menu {
    width: Option<String>,
    background_color: Option<String>,
    text_color: Option<String>,
    active_text_color: Option<String>,
    group_items: Option<GroupItems>,
    level1_items: Option<Level1Items>,
    arrow: Option<Arrow>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GroupItems {
    text_transform: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Level1Items {
    text_transform: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Arrow {
    size: Option<String>,
    color: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Logo {
    max_height: Option<String>,
    max_width: Option<String>,
    gutter: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RightPanel {
    background_color: Option<String>,
    width: Option<String>,
    text_color: Option<String>,
}
