mod theme;
use serde::{Deserialize, Serialize};
use theme::ThemeOptions;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, window, Document, Element, HtmlScriptElement};

fn try_it_out_enabled_default() -> bool {
    true
}

fn redoc_version_default() -> String {
    "2.1.3".to_string()
}

fn try_it_box_container_id_default() -> String {
    "try-out-wrapper".to_string()
}

fn container_id_default() -> String {
    "redoc-container".to_string()
}

fn operation_box_selector_default() -> String {
    "[data-section-id]".to_string()
}

fn selected_operation_class_default() -> String {
    "try".to_string()
}

fn jquery_default() -> String {
    "3.5.1".to_string()
}

fn jquery_scroll_to_default() -> String {
    "2.1.3".to_string()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_name = init, js_namespace = Redoc)]
    fn initRedoc(docUrl: String, options: JsValue, element: Element, callback: &js_sys::Function);
}

#[wasm_bindgen]
pub struct RedocTryItOut {
    document: Document,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct DependenciesVersions {
    #[serde(default = "jquery_default")]
    jquery: String,
    #[serde(default = "jquery_scroll_to_default")]
    jquery_scroll_to: String,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthBtnOptions {
    pos_selector: Option<String>,
    text: Option<String>,
    class_name: Option<String>,
}

impl Default for AuthBtnOptions {
    fn default() -> Self {
        Self {
            pos_selector: Option::default(),
            text: Option::default(),
            class_name: Option::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TryBtnOptions {
    sibling_selector: Option<String>,
    text: Option<String>,
    class_name: Option<String>,
    selected_class_name: Option<String>,
}

impl Default for TryBtnOptions {
    fn default() -> Self {
        Self {
            sibling_selector: Option::default(),
            text: Option::default(),
            class_name: Option::default(),
            selected_class_name: Option::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, serde_derive_default::Default)]
#[serde(rename_all = "camelCase")]
pub struct RedocTryItOutOptions {
    #[serde(default = "redoc_version_default")]
    redoc_version: String,
    #[serde(default = "try_it_out_enabled_default")]
    try_it_out_enabled: bool,
    #[serde(default = "try_it_box_container_id_default")]
    try_it_box_container_id: String,
    #[serde(default = "container_id_default")]
    container_id: String,
    #[serde(default = "operation_box_selector_default")]
    operation_box_selector: String,
    #[serde(default = "selected_operation_class_default")]
    selected_operation_class: String,
    #[serde(default)]
    dependencies_versions: DependenciesVersions,
    #[serde(default)]
    auth_btn: AuthBtnOptions,
    #[serde(default)]
    try_btn: TryBtnOptions,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RedocOptions {
    /** disable search indexing and search box */
    disable_search: Option<bool>,
    /** enable expanding default server variables, default false.*/
    expand_default_server_variables: Option<bool>,
    /** specify which responses to expand by default by response codes.
     * Values should be passed as comma-separated list without spaces e.g. expandResponses="200,201".
     * Special value "all" expands all responses by default.
     * Be careful: this option can slow-down documentation rendering time.
     **/
    expand_responses: Option<bool>,
    /** set the maximum render depth for JSON payload samples (responses and request body). The default value is 10.*/
    generated_payload_samples_max_depth: Option<u32>,
    /** display only specified number of enum values. hide rest values under spoiler. */
    max_displayed_enum_values: Option<u32>,
    /** do not show "Download" spec button. THIS DOESN'T MAKE YOUR SPEC PRIVATE, it just hides the button. */
    hide_download_button: Option<bool>,
    /** if set, the protocol and hostname is not shown in the operation definition. */
    hide_hostname: Option<bool>,
    /** do not show loading animation. Useful for small docs. */
    hide_loading: Option<bool>,
    /** if set, the pattern is not shown in the schema. */
    hide_schema_pattern: Option<bool>,
    /** do not show the request sample tab for requests with only one sample. */
    hide_single_request_sample_tab: Option<bool>,
    /** automatically expand single field in a schema */
    expand_single_schema_field: Option<bool>,
    /** set the default expand level for JSON payload samples (responses and request body).
     * Special value "all" expands all levels.
     * The default value is 2.
     **/
    json_sample_expand_level: Option<u32>,
    /** do not display schema title next to to the type */
    hide_schema_titles: Option<bool>,
    /** show only unique oneOf types in the label without titles */
    simple_one_of_type_label: Option<bool>,
    /** Not implemented yet if set, enables lazy rendering mode in ReDoc.
     * This mode is useful for APIs with big number of operations (e.g. > 50).
     * In this mode ReDoc shows initial screen ASAP and then renders the rest
     * operations asynchronously while showing progress bar on the top.
     * Check out the demo for the example.
     **/
    lazy_rendering: Option<bool>,
    /** if true clicking second time on expanded menu item will collapse it, default true. */
    menu_toggle: Option<bool>,
    /** use native scrollbar for sidemenu instead of perfect-scroll (scrolling performance optimization for big specs). */
    native_scrollbars: Option<bool>,
    /** do not inject Authentication section automatically. */
    no_auto_auth: Option<bool>,
    /** shows only required fields in request samples. */
    only_required_in_samples: Option<bool>,
    /** show path link and HTTP verb in the middle panel instead of the right one. */
    path_in_middle_panel: Option<bool>,
    /** show required properties first ordered in the same order as in required array. */
    required_props_first: Option<bool>,
    /** If set, specifies a vertical scroll-offset.
     * This is often useful when there are fixed positioned elements at the top of the page, such as navbars, headers etc;
     * scrollYOffset can be specified in various ways: number: A fixed number of pixels to be used as offset.
     **/
    scroll_y_offset: Option<String>,
    /** selector of the element to be used for specifying the offset.
     * The distance from the top of the page to the element's bottom will be used as offset.
     **/
    selector: Option<String>,
    /** A getter function. Must return a number representing the offset (in pixels). */
    //function: Option<js_sys::Function>,
    /** show vendor extensions ("x-" fields).
     * Extensions used by ReDoc are ignored. Can be an array of string with names of extensions to display.
     **/
    show_extensions: Option<Vec<String>>,
    /** sort properties alphabetically. */
    sort_props_alphabetically: Option<bool>,
    /** if set, payload sample will be inserted at this index or last. Indexes start from 0.*/
    payload_sample_idx: Option<u32>,
    /** ReDoc theme. For details check theme docs. */
    theme: Option<ThemeOptions>,
    /** if set, the spec is considered untrusted and all HTML/markdown is sanitized to prevent XSS.
     * Disabled by default for performance reasons. Enable this option if you work with untrusted user data!
     **/
    untrusted_spec: Option<bool>,
}

#[wasm_bindgen]
impl RedocTryItOut {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<RedocTryItOut, JsValue> {
        let window = window().ok_or("no global `window` exists")?;
        let document = window
            .document()
            .ok_or("should have a document on window")?;
        Ok(RedocTryItOut { document })
    }

    pub async fn init(
        &self,
        doc_url: String,
        raw_config: JsValue,
        element: Option<Element>,
    ) -> Result<(), JsValue> {
        let config: RedocTryItOutOptions = serde_wasm_bindgen::from_value(raw_config.clone())
            .map_err(|e| JsValue::from_str(&format!("Failed to parse config: {:?}", e)))?;

        let redoc_config: RedocOptions = serde_wasm_bindgen::from_value(raw_config)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse redoc config: {:?}", e)))?;

        self.add_script_tag(format!(
            "https://cdn.jsdelivr.net/npm/redoc@{}/bundles/redoc.standalone.min.js",
            config.redoc_version
        ))
        .await?;

        let redoc_container = element
            .or_else(|| {
                self.document
                    .get_element_by_id(config.container_id.as_str())
            })
            .ok_or_else(|| JsValue::from_str("should have a redoc container"))?;

        let options = serde_wasm_bindgen::to_value(&redoc_config)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize: {:?}", e)))?;

        let init_promise = js_sys::Promise::new(&mut move |resolve, reject| {
            let init_callback = Closure::wrap(Box::new(move |err: JsValue| {
                if err.is_undefined() {
                    resolve.call0(&JsValue::NULL).unwrap();
                } else {
                    reject.call1(&JsValue::NULL, &err).unwrap();
                }
            }) as Box<dyn FnMut(JsValue)>);

            initRedoc(
                doc_url.clone(),
                options.clone(),
                redoc_container.clone(),
                init_callback.as_ref().unchecked_ref(),
            );

            init_callback.forget();
        });

        JsFuture::from(init_promise).await?;
        Ok(())
    }

    async fn add_script_tag(&self, url: String) -> Result<(), JsValue> {
        let script = self
            .document
            .create_element("script")?
            .dyn_into::<HtmlScriptElement>()?;
        script.set_src(&url);

        let promise = js_sys::Promise::new(&mut |resolve, reject| {
            let onload = Closure::wrap(Box::new(move |_| {
                if let Err(e) = resolve.call0(&JsValue::NULL) {
                    log(&format!("Failed to resolve: {:?}", e));
                }
            }) as Box<dyn FnMut(JsValue)>);

            let onerror = Closure::wrap(Box::new(move |_| {
                if let Err(e) = reject.call0(&JsValue::NULL) {
                    log(&format!("Failed to reject: {:?}", e));
                }
            }) as Box<dyn FnMut(JsValue)>);

            script.set_onload(Some(onload.as_ref().unchecked_ref()));
            script.set_onerror(Some(onerror.as_ref().unchecked_ref()));

            onload.forget();
            onerror.forget();
        });

        // Append the script to the body
        let body = self
            .document
            .body()
            .ok_or_else(|| JsValue::from_str("should have a body"))?;
        body.append_child(&script)?;

        // Wait for the script to load
        JsFuture::from(promise).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_wasm_bindgen::{from_value, to_value};
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
