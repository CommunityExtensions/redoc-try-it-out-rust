use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, window, Document, HtmlScriptElement};

fn set_option<T>(
    value: JsValue,
    type_str: &str,
    convert: impl Fn(&JsValue) -> Option<T>,
) -> Result<T, JsValue> {
    if type_str == value.js_typeof().as_string().unwrap_or_default() {
        match convert(&value) {
            Some(val) => Ok(val),
            None => Err(JsValue::from_str(&format!(
                "failed to convert value to {}",
                type_str
            ))),
        }
    } else {
        Err(JsValue::from_str(&format!(
            "only accepts {} values",
            type_str
        )))
    }
}

fn set_boolean_option(value: JsValue) -> Result<bool, JsValue> {
    set_option(value, "boolean", JsValue::as_bool)
}

fn set_64_option(value: JsValue) -> Result<f64, JsValue> {
    set_option(value, "number", JsValue::as_f64)
}

fn set_boolean_option_for(value: JsValue, mut set_value: impl FnMut(bool)) -> Result<(), JsValue> {
    match set_boolean_option(value) {
        Ok(val) => {
            set_value(val);
            Ok(())
        }
        Err(err) => Err(JsValue::from_str(&format!("{}", err.as_string().unwrap()))),
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct RedocTryItOut {
    document: Document,
}

#[wasm_bindgen(getter_with_clone)]
pub struct RedocTryItOutOptions {
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
    function: Option<js_sys::Function>,
    /** show vendor extensions ("x-" fields).
     * Extensions used by ReDoc are ignored. Can be an array of string with names of extensions to display.
     **/
    show_extensions: Option<Vec<String>>,
    /** sort properties alphabetically. */
    sort_props_alphabetically: Option<bool>,
    /** if set, payload sample will be inserted at this index or last. Indexes start from 0.*/
    payload_sample_idx: Option<u32>,
    /** ReDoc theme. For details check theme docs. */
    // theme?: ThemOptions;
    /** if set, the spec is considered untrusted and all HTML/markdown is sanitized to prevent XSS.
     * Disabled by default for performance reasons. Enable this option if you work with untrusted user data!
     **/
    untrusted_spec: Option<bool>,
}

#[wasm_bindgen]
impl RedocTryItOutOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RedocTryItOutOptions {
        RedocTryItOutOptions::default()
    }

    #[wasm_bindgen(js_name = setDisableSearch)]
    pub fn set_disable_search(mut self, value: JsValue) -> Result<RedocTryItOutOptions, JsValue> {
        set_boolean_option_for(value, |val| self.disable_search = Some(val))
            .map(|_| self)
            .map_err(|err| {
                JsValue::from_str(&format!("set_disable_search {}", err.as_string().unwrap()))
            })
    }

    #[wasm_bindgen(js_name = setExpandDefaultServerVariables)]
    pub fn set_expand_default_server_variables(
        mut self,
        value: JsValue,
    ) -> Result<RedocTryItOutOptions, JsValue> {
        set_boolean_option_for(value, |val| {
            self.expand_default_server_variables = Some(val)
        })
        .map(|_| self)
        .map_err(|err| {
            JsValue::from_str(&format!(
                "set_expand_default_server_variables {}",
                err.as_string().unwrap()
            ))
        })
    }

    #[wasm_bindgen(js_name = setExpandResponses)]
    pub fn set_expand_responses(mut self, value: JsValue) -> Result<RedocTryItOutOptions, JsValue> {
        set_boolean_option_for(value, |val| self.expand_responses = Some(val))
            .map(|_| self)
            .map_err(|err| {
                JsValue::from_str(&format!(
                    "set_expand_responses {}",
                    err.as_string().unwrap()
                ))
            })
    }

    #[wasm_bindgen(js_name = setGeneratedPayloadSamplesMaxDepth)]
    pub fn set_generated_payload_samples_max_depth(
        mut self,
        value: JsValue,
    ) -> Result<RedocTryItOutOptions, JsValue> {
        match set_64_option(value) {
            Ok(val) => {
                self.generated_payload_samples_max_depth = Some(val as u32);
                Ok(self)
            }
            Err(err) => Err(JsValue::from_str(&format!(
                "set_generated_payload_samples_max_depth {}",
                err.as_string().unwrap()
            ))),
        }
    }

    #[wasm_bindgen(js_name = setMaxDisplayedEnumValues)]
    pub fn set_max_displayed_enum_values(
        mut self,
        value: JsValue,
    ) -> Result<RedocTryItOutOptions, JsValue> {
        match set_64_option(value) {
            Ok(val) => {
                self.generated_payload_samples_max_depth = Some(val as u32);
                Ok(self)
            }
            Err(err) => Err(JsValue::from_str(&format!(
                "set_max_displayed_enum_values {}",
                err.as_string().unwrap()
            ))),
        }
    }
}

impl Default for RedocTryItOutOptions {
    fn default() -> Self {
        Self {
            disable_search: Option::default(),
            expand_default_server_variables: Option::default(),
            expand_responses: Option::default(),
            generated_payload_samples_max_depth: Option::default(),
            max_displayed_enum_values: Option::default(),
            hide_download_button: Option::default(),
            hide_hostname: Option::default(),
            hide_loading: Option::default(),
            hide_schema_pattern: Option::default(),
            hide_single_request_sample_tab: Option::default(),
            expand_single_schema_field: Option::default(),
            json_sample_expand_level: Option::default(),
            hide_schema_titles: Option::default(),
            simple_one_of_type_label: Option::default(),
            lazy_rendering: Option::default(),
            menu_toggle: Option::default(),
            native_scrollbars: Option::default(),
            no_auto_auth: Option::default(),
            only_required_in_samples: Option::default(),
            path_in_middle_panel: Option::default(),
            required_props_first: Option::default(),
            scroll_y_offset: Option::default(),
            selector: Option::default(),
            function: Option::default(),
            show_extensions: Option::default(),
            sort_props_alphabetically: Option::default(),
            payload_sample_idx: Option::default(),
            untrusted_spec: Option::default(),
        }
    }
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

    pub async fn init(&self, config: RedocTryItOutOptions) -> Result<(), JsValue> {
        log(config
            .generated_payload_samples_max_depth
            .unwrap_or_default()
            .to_string()
            .as_str());
        self.add_script_tag(
            "https://rebilly.github.io/ReDoc/releases/latest/redoc.min.js".to_string(),
        )
        .await?;
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
        let body = self.document.body().expect("document should have a body");
        body.append_child(&script)?;

        // Wait for the script to load
        JsFuture::from(promise).await?;

        log("Script tag added");

        Ok(())
    }
}
