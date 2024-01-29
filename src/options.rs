use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use wasm_bindgen::prelude::*;
use super::theme::ThemeOptions;

#[serde_inline_default]
#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct DependenciesVersions {
    #[serde_inline_default("3.5.1".to_string())]
    pub jquery: String,
    #[serde_inline_default("2.1.3".to_string())]
    pub jquery_scroll_to: String,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthBtnOptions {
    pub pos_selector: Option<String>,
    pub text: Option<String>,
    pub class_name: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct TryBtnOptions {
    pub sibling_selector: Option<String>,
    pub text: Option<String>,
    pub class_name: Option<String>,
    pub selected_class_name: Option<String>,
}

#[serde_inline_default]
#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct RedocTryItOutOptions {
    #[serde_inline_default("2.1.3".to_string())]
    pub redoc_version: String,
    #[serde_inline_default(true)]
    pub try_it_out_enabled: bool,
    #[serde_inline_default("try-out-wrapper".to_string())]
    pub try_it_box_container_id: String,
    #[serde_inline_default("redoc-container".to_string())]
    pub container_id: String,
    #[serde_inline_default("[data-section-id]".to_string())]
    pub operation_box_selector: String,
    #[serde_inline_default("try".to_string())]
    pub selected_operation_class: String,
    #[serde(default)]
    pub dependencies_versions: DependenciesVersions,
    #[serde(default)]
    pub auth_btn: AuthBtnOptions,
    #[serde(default)]
    pub try_btn: TryBtnOptions,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RedocOptions {
    /** disable search indexing and search box */
    pub disable_search: Option<bool>,
    /** enable expanding default server variables, default false.*/
    pub expand_default_server_variables: Option<bool>,
    /** specify which responses to expand by default by response codes.
     * Values should be passed as comma-separated list without spaces e.g. expandResponses="200,201".
     * Special value "all" expands all responses by default.
     * Be careful: this option can slow-down documentation rendering time.
     **/
     pub expand_responses: Option<bool>,
    /** set the maximum render depth for JSON payload samples (responses and request body). The default value is 10.*/
    pub generated_payload_samples_max_depth: Option<u32>,
    /** display only specified number of enum values. hide rest values under spoiler. */
    pub max_displayed_enum_values: Option<u32>,
    /** do not show "Download" spec button. THIS DOESN'T MAKE YOUR SPEC PRIVATE, it just hides the button. */
    pub hide_download_button: Option<bool>,
    /** if set, the protocol and hostname is not shown in the operation definition. */
    pub hide_hostname: Option<bool>,
    /** do not show loading animation. Useful for small docs. */
    pub hide_loading: Option<bool>,
    /** if set, the pattern is not shown in the schema. */
    pub hide_schema_pattern: Option<bool>,
    /** do not show the request sample tab for requests with only one sample. */
    pub hide_single_request_sample_tab: Option<bool>,
    /** automatically expand single field in a schema */
    pub expand_single_schema_field: Option<bool>,
    /** set the default expand level for JSON payload samples (responses and request body).
     * Special value "all" expands all levels.
     * The default value is 2.
     **/
    pub json_sample_expand_level: Option<u32>,
    /** do not display schema title next to to the type */
    pub hide_schema_titles: Option<bool>,
    /** show only unique oneOf types in the label without titles */
    pub simple_one_of_type_label: Option<bool>,
    /** Not implemented yet if set, enables lazy rendering mode in ReDoc.
     * This mode is useful for APIs with big number of operations (e.g. > 50).
     * In this mode ReDoc shows initial screen ASAP and then renders the rest
     * operations asynchronously while showing progress bar on the top.
     * Check out the demo for the example.
     **/
     pub lazy_rendering: Option<bool>,
    /** if true clicking second time on expanded menu item will collapse it, default true. */
    pub menu_toggle: Option<bool>,
    /** use native scrollbar for sidemenu instead of perfect-scroll (scrolling performance optimization for big specs). */
    pub native_scrollbars: Option<bool>,
    /** do not inject Authentication section automatically. */
    pub no_auto_auth: Option<bool>,
    /** shows only required fields in request samples. */
    pub only_required_in_samples: Option<bool>,
    /** show path link and HTTP verb in the middle panel instead of the right one. */
    pub path_in_middle_panel: Option<bool>,
    /** show required properties first ordered in the same order as in required array. */
    pub required_props_first: Option<bool>,
    /** If set, specifies a vertical scroll-offset.
     * This is often useful when there are fixed positioned elements at the top of the page, such as navbars, headers etc;
     * scrollYOffset can be specified in various ways: number: A fixed number of pixels to be used as offset.
     **/
     pub scroll_y_offset: Option<String>,
    /** selector of the element to be used for specifying the offset.
     * The distance from the top of the page to the element's bottom will be used as offset.
     **/
    pub selector: Option<String>,
    /** A getter function. Must return a number representing the offset (in pixels). */
    //function: Option<js_sys::Function>,
    /** show vendor extensions ("x-" fields).
     * Extensions used by ReDoc are ignored. Can be an array of string with names of extensions to display.
     **/
    pub show_extensions: Option<Vec<String>>,
    /** sort properties alphabetically. */
    pub sort_props_alphabetically: Option<bool>,
    /** if set, payload sample will be inserted at this index or last. Indexes start from 0.*/
    pub payload_sample_idx: Option<u32>,
    /** ReDoc theme. For details check theme docs. */
    pub theme: Option<ThemeOptions>,
    /** if set, the spec is considered untrusted and all HTML/markdown is sanitized to prevent XSS.
     * Disabled by default for performance reasons. Enable this option if you work with untrusted user data!
     **/
     pub untrusted_spec: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_wasm_bindgen::{from_value, to_value};
    use wasm_bindgen_futures::js_sys;
    use wasm_bindgen_test::*;
    use pretty_assertions::assert_eq;

    // verify that not all fields are required in the JSON
    #[wasm_bindgen_test]
    fn test_try_it_out_options_from_json() {
        let json = r###"
            {
                "redocVersion": "9.9.9",
                "tryItOutEnabled": true,
                "containerId": "custom-redoc-container",
                "dependenciesVersions": {
                    "jqueryScrollTo": "4.4.4"
                },
                "authBtn": {
                    "posSelector": "#redoc-container",
                    "text": "Authorize",
                    "className": "auth-btn"
                },
                "tryBtn": {
                    "text": "Try it out",
                    "className": "try-btn",
                    "selectedClassName": "try-btn-selected"
                }
            }
        "###;

        let expected = RedocTryItOutOptions {
            redoc_version: "9.9.9".to_string(),
            try_it_out_enabled: true,
            try_it_box_container_id: "try-out-wrapper".to_string(),
            container_id: "custom-redoc-container".to_string(),
            operation_box_selector: "[data-section-id]".to_string(),
            selected_operation_class: "try".to_string(),
            dependencies_versions: DependenciesVersions {
                jquery: "3.5.1".to_string(),
                jquery_scroll_to: "4.4.4".to_string(),
            },
            auth_btn: AuthBtnOptions {
                pos_selector: Some("#redoc-container".to_string()),
                text: Some("Authorize".to_string()),
                class_name: Some("auth-btn".to_string()),
            },
            try_btn: TryBtnOptions {
                sibling_selector: None,
                text: Some("Try it out".to_string()),
                class_name: Some("try-btn".to_string()),
                selected_class_name: Some("try-btn-selected".to_string()),
            },
        };

        let actual: RedocTryItOutOptions =
            serde_wasm_bindgen::from_value(js_sys::JSON::parse(json).unwrap()).unwrap();

        assert_eq!(expected, actual);
    }

    // Verify that struct can be deserialized from JSON with camelCase keys
    #[wasm_bindgen_test]
    fn test_auth_btn_options_from_json() {
        let json = r###"
            {
                "posSelector": "#redoc-container",
                "text": "Authorize",
                "className": "auth-btn"
            }
        "###;

        let expected = AuthBtnOptions {
            pos_selector: Some("#redoc-container".to_string()),
            text: Some("Authorize".to_string()),
            class_name: Some("auth-btn".to_string()),
        };

        let actual: AuthBtnOptions =
            serde_wasm_bindgen::from_value(js_sys::JSON::parse(json).unwrap()).unwrap();

        assert_eq!(expected, actual);
    }

    #[wasm_bindgen_test]
    fn test_auth_btn_options() {
        let auth_btn_options: AuthBtnOptions = AuthBtnOptions {
            pos_selector: Some("selector".to_string()),
            text: Some("text".to_string()),
            class_name: Some("class".to_string()),
        };

        // Serialize the struct to a JsValue
        let js_value = to_value(&auth_btn_options).unwrap();

        // Deserialize the JsValue back into a struct
        let deserialized: AuthBtnOptions = from_value(js_value).unwrap();

        // Check that the original struct and the deserialized struct are the same
        assert_eq!(auth_btn_options, deserialized);
    }

    #[wasm_bindgen_test]
    fn test_try_btn_options() {
        let try_btn_options: TryBtnOptions = TryBtnOptions {
            sibling_selector: Some("selector".to_string()),
            text: Some("text".to_string()),
            class_name: Some("class".to_string()),
            selected_class_name: Some("selected_class".to_string()),
        };

        // Serialize the struct to a JsValue
        let js_value = to_value(&try_btn_options).unwrap();

        // Deserialize the JsValue back into a struct
        let deserialized: TryBtnOptions = from_value(js_value).unwrap();

        // Check that the original struct and the deserialized struct are the same
        assert_eq!(try_btn_options, deserialized);
    }

    #[wasm_bindgen_test]
    fn test_dependencies_versions() {
        let dependencies_versions: DependenciesVersions = DependenciesVersions {
            jquery: "3.5.1".to_string(),
            jquery_scroll_to: "2.1.3".to_string(),
        };

        // Serialize the struct to a JsValue
        let js_value = to_value(&dependencies_versions).unwrap();

        // Deserialize the JsValue back into a struct
        let deserialized: DependenciesVersions = from_value(js_value).unwrap();

        // Check that the original struct and the deserialized struct are the same
        assert_eq!(dependencies_versions, deserialized);
    }

    #[wasm_bindgen_test]
    fn test_redoc_try_it_out_options() {
        let redoc_try_it_out_options: RedocTryItOutOptions = RedocTryItOutOptions {
            redoc_version: "2.1.3".to_string(),
            try_it_out_enabled: true,
            try_it_box_container_id: "try-out-wrapper".to_string(),
            container_id: "redoc-container".to_string(),
            operation_box_selector: "[data-section-id]".to_string(),
            selected_operation_class: "try".to_string(),
            dependencies_versions: DependenciesVersions {
                jquery: "3.5.1".to_string(),
                jquery_scroll_to: "2.1.3".to_string(),
            },
            auth_btn: AuthBtnOptions {
                pos_selector: Some("selector".to_string()),
                text: Some("text".to_string()),
                class_name: Some("class".to_string()),
            },
            try_btn: TryBtnOptions {
                sibling_selector: Some("selector".to_string()),
                text: Some("text".to_string()),
                class_name: Some("class".to_string()),
                selected_class_name: Some("selected_class".to_string()),
            },
        };

        // Serialize the struct to a JsValue
        let js_value = to_value(&redoc_try_it_out_options).unwrap();

        // Deserialize the JsValue back into a struct
        let deserialized: RedocTryItOutOptions = from_value(js_value).unwrap();

        // Check that the original struct and the deserialized struct are the same
        assert_eq!(redoc_try_it_out_options, deserialized);
    }

    #[wasm_bindgen_test]
    fn test_redoc_options() {
        let redoc_options: RedocOptions = RedocOptions {
            disable_search: Some(true),
            expand_default_server_variables: Some(true),
            expand_responses: Some(true),
            generated_payload_samples_max_depth: Some(10),
            max_displayed_enum_values: Some(10),
            hide_download_button: Some(true),
            hide_hostname: Some(true),
            hide_loading: Some(true),
            hide_schema_pattern: Some(true),
            hide_single_request_sample_tab: Some(true),
            expand_single_schema_field: Some(true),
            json_sample_expand_level: Some(10),
            hide_schema_titles: Some(true),
            simple_one_of_type_label: Some(true),
            lazy_rendering: Some(true),
            menu_toggle: Some(true),
            native_scrollbars: Some(true),
            no_auto_auth: Some(true),
            only_required_in_samples: Some(true),
            path_in_middle_panel: Some(true),
            required_props_first: Some(true),
            scroll_y_offset: Some("10".to_string()),
            selector: Some("10".to_string()),
            //function: Some(js_sys::Function::new_no_args(&mut || {})),
            show_extensions: Some(vec!["x-".to_string()]),
            sort_props_alphabetically: Some(true),
            payload_sample_idx: Some(10),
            theme: None,
            untrusted_spec: Some(true),
        };

        // Serialize the struct to a JsValue
        let js_value = to_value(&redoc_options).unwrap();

        // Deserialize the JsValue back into a struct
        let deserialized: RedocOptions = from_value(js_value).unwrap();

        // Check that the original struct and the deserialized struct are the same
        assert_eq!(redoc_options, deserialized);
    }
}
