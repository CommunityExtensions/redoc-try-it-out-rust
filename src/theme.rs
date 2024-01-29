use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct Spacing {
    pub unit: Option<u32>,
    pub section_horizontal: Option<u32>,
    pub section_vertical: Option<u32>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct Breakpoints {
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct Colors {
    pub tonal_offset: Option<f32>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct Headings {
    pub font_family: Option<String>,
    pub font_weight: Option<u32>,
    pub line_height: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub color: Option<String>,
    pub visited: Option<String>,
    pub hover: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupItems {
    pub text_transform: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct Level1Items {
    pub text_transform: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct Arrow {
    pub size: Option<String>,
    pub color: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct Logo {
    pub max_height: Option<String>,
    pub max_width: Option<String>,
    pub gutter: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct RightPanel {
    pub background_color: Option<String>,
    pub width: Option<String>,
    pub text_color: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_wasm_bindgen::{from_value, to_value};
    use wasm_bindgen_futures::js_sys;
    use wasm_bindgen_test::*;

    // Verify that struct can be deserialized from JSON with camelCase keys
    #[wasm_bindgen_test]
    fn test_right_panel_options_from_json() {
        let json = r###"
            {
                "backgroundColor": "#fafafa",
                "width": "40%",
                "textColor": "#333333"
            }
        "###;

        let expected = RightPanel {
            background_color: Some("#fafafa".to_string()),
            width: Some("40%".to_string()),
            text_color: Some("#333333".to_string()),
        };

        let deserialized: RightPanel = from_value(js_sys::JSON::parse(json).unwrap()).unwrap();

        assert_eq!(expected, deserialized);
    }

    #[wasm_bindgen_test]
    fn test_theme_options() {
        let theme_options: ThemeOptions = ThemeOptions {
            spacing: Some(Spacing {
                unit: Some(10),
                section_horizontal: Some(10),
                section_vertical: Some(10),
            }),
            breakpoints: Some(Breakpoints {
                small: Some("10".to_string()),
                medium: Some("10".to_string()),
                large: Some("10".to_string()),
            }),
            colors: Some(Colors {
                tonal_offset: Some(10.0),
            }),
            typography: Some(Typography {
                font_size: Some("10".to_string()),
                line_height: Some("10".to_string()),
                font_weight_regular: Some(10),
                font_weight_bold: Some(10),
                font_weight_light: Some(10),
                font_family: Some("10".to_string()),
                smoothing: Some("10".to_string()),
                optimize_speed: Some(true),
                headings: Some(Headings {
                    font_family: Some("10".to_string()),
                    font_weight: Some(10),
                    line_height: Some("10".to_string()),
                }),
                code: Some(Code {
                    font_size: Some("10".to_string()),
                    font_family: Some("10".to_string()),
                    line_height: Some(1),
                    font_weight: Some(10),
                    color: Some("10".to_string()),
                    background_color: Some("10".to_string()),
                    wrap: Some(true),
                }),
                links: Some(Links {
                    color: Some("10".to_string()),
                    visited: Some("10".to_string()),
                    hover: Some("10".to_string()),
                }),
            }),
            right_panel: Some(RightPanel {
                background_color: Some("10".to_string()),
                width: Some("10".to_string()),
                text_color: Some("10".to_string()),
            }),
            logo: Some(Logo {
                gutter: Some("10".to_string()),
                max_height: Some("10".to_string()),
                max_width: Some("10".to_string()),
            }),
            sidebar: Some(Menu {
                width: Some("10".to_string()),
                background_color: Some("10".to_string()),
                text_color: Some("10".to_string()),
                active_text_color: Some("10".to_string()),
                group_items: Some(GroupItems {
                    text_transform: Some("10".to_string()),
                }),
                level1_items: Some(Level1Items {
                    text_transform: Some("10".to_string()),
                }),
                arrow: Some(Arrow {
                    size: Some("10".to_string()),
                    color: Some("10".to_string()),
                }),
            }),
        };

        // Serialize the struct to a JsValue
        let js_value = to_value(&theme_options).unwrap();

        // Deserialize the JsValue back into a struct
        let deserialized: ThemeOptions = from_value(js_value).unwrap();

        // Check that the original struct and the deserialized struct are the same
        assert_eq!(theme_options, deserialized);
    }
}
