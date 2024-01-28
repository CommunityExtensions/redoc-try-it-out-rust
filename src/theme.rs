use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ThemeOptions {
    pub spacing: Option<Spacing>,
    pub breakpoints: Option<Breakpoints>,
    pub colors: Option<Colors>,
    pub typography: Option<Typography>,
    pub sidebar: Option<Menu>,
    pub logo: Option<Logo>,
    pub right_panel: Option<RightPanel>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Spacing {
    pub unit: Option<u32>,
    pub section_horizontal: Option<u32>,
    pub section_vertical: Option<u32>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Breakpoints {
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Colors {
    pub tonal_offset: Option<f32>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Typography {
    pub font_size: Option<String>,
    pub line_height: Option<String>,
    pub font_weight_regular: Option<u32>,
    pub font_weight_bold: Option<u32>,
    pub font_weight_light: Option<u32>,
    pub font_family: Option<String>,
    pub smoothing: Option<String>,
    pub optimize_speed: Option<bool>,
    pub headings: Option<Headings>,
    pub code: Option<Code>,
    pub links: Option<Links>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Headings {
    pub font_family: Option<String>,
    pub font_weight: Option<u32>,
    pub line_height: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Code {
    pub font_size: Option<String>,
    pub font_family: Option<String>,
    pub line_height: Option<u32>,
    pub font_weight: Option<u32>,
    pub color: Option<String>,
    pub background_color: Option<String>,
    pub wrap: Option<bool>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub color: Option<String>,
    pub visited: Option<String>,
    pub hover: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub width: Option<String>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub active_text_color: Option<String>,
    pub group_items: Option<GroupItems>,
    pub level1_items: Option<Level1Items>,
    pub arrow: Option<Arrow>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GroupItems {
    pub text_transform: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Level1Items {
    pub text_transform: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Arrow {
    pub size: Option<String>,
    pub color: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Logo {
    pub max_height: Option<String>,
    pub max_width: Option<String>,
    pub gutter: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RightPanel {
    pub background_color: Option<String>,
    pub width: Option<String>,
    pub text_color: Option<String>,
}
