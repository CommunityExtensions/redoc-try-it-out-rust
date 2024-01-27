use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, window, Document, HtmlScriptElement};

fn parse_option_for<T>(
    key: String,
    value: JsValue,
    type_str: &str,
    convert: impl Fn(&JsValue) -> Option<T>,
) -> Result<T, JsValue> {
    if type_str == value.js_typeof().as_string().unwrap_or_default() {
        match convert(&value) {
            Some(val) => Ok(val),
            None => Err(JsValue::from_str(&format!(
                "{} failed to convert value to {}",
                key, type_str
            ))),
        }
    } else {
        Err(JsValue::from_str(&format!(
            "{} only accepts {} values",
            key, type_str
        )))
    }
}

fn parse_boolean_option_for(key: String, value: JsValue) -> Result<bool, JsValue> {
    parse_option_for(key, value, "boolean", JsValue::as_bool)
}

fn parse_u32_option_for(key: String, value: JsValue) -> Result<u32, JsValue> {
    parse_option_for(key, value, "number", |v| v.as_f64().map(|v| v as u32))
}

fn parse_string_option_for(key: String, value: JsValue) -> Result<String, JsValue> {
    parse_option_for(key, value, "string", JsValue::as_string)
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
    redoc_version: String,
    try_it_out_enabled: bool,
    try_it_box_container_id: String,
    container_id: String,
    operation_box_selector: String,
    selected_operation_class: String,
    // dependenciesVersions: DependenciesVersions,
    // authBtn: AuthBtnOptions,
    // tryBtn: TryBtnOptions,
    // swaggerOptions: SwaggerOptions,
    // stylerMatcher: StyleMatcherOptions
}

impl Default for RedocTryItOutOptions {
    fn default() -> Self {
        Self {
            redoc_version: "2.1.3".to_string(),
            try_it_out_enabled: true,
            try_it_box_container_id: "try-out-wrapper".to_string(),
            container_id: "redoc-container".to_string(),
            operation_box_selector: "[data-section-id]".to_string(),
            selected_operation_class: "try".to_string(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
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
    redocTryItOut: RedocTryItOutOptions,
}

#[wasm_bindgen]
impl RedocOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RedocOptions {
        RedocOptions::default()
    }

    #[wasm_bindgen(js_name = setDisableSearch)]
    pub fn set_disable_search(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_disable_search".to_string(), value).map(|val| {
            self.disable_search = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setExpandDefaultServerVariables)]
    pub fn set_expand_default_server_variables(
        mut self,
        value: JsValue,
    ) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_expand_default_server_variables".to_string(), value).map(
            |val| {
                self.expand_default_server_variables = Some(val);
                self
            },
        )
    }

    #[wasm_bindgen(js_name = setExpandResponses)]
    pub fn set_expand_responses(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_expand_responses".to_string(), value).map(|val| {
            self.expand_responses = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setGeneratedPayloadSamplesMaxDepth)]
    pub fn set_generated_payload_samples_max_depth(
        mut self,
        value: JsValue,
    ) -> Result<RedocOptions, JsValue> {
        parse_u32_option_for("set_generated_payload_samples_max_depth".to_string(), value).map(
            |val| {
                self.generated_payload_samples_max_depth = Some(val);
                self
            },
        )
    }

    #[wasm_bindgen(js_name = setMaxDisplayedEnumValues)]
    pub fn set_max_displayed_enum_values(
        mut self,
        value: JsValue,
    ) -> Result<RedocOptions, JsValue> {
        parse_u32_option_for("set_max_displayed_enum_values".to_string(), value).map(|val| {
            self.max_displayed_enum_values = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setHideDownloadButton)]
    pub fn set_hide_download_button(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_hide_download_button".to_string(), value).map(|val| {
            self.hide_download_button = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setHideHostname)]
    pub fn set_hide_hostname(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_hide_hostname".to_string(), value).map(|val| {
            self.hide_hostname = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setHideLoading)]
    pub fn set_hide_loading(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_hide_loading".to_string(), value).map(|val| {
            self.hide_loading = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setHideSchemaPattern)]
    pub fn set_hide_schema_pattern(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_hide_schema_pattern".to_string(), value).map(|val| {
            self.hide_schema_pattern = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setHideSingleRequestSampleTab)]
    pub fn set_hide_single_request_sample_tab(
        mut self,
        value: JsValue,
    ) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_hide_single_request_sample_tab".to_string(), value).map(
            |val| {
                self.hide_single_request_sample_tab = Some(val);
                self
            },
        )
    }

    #[wasm_bindgen(js_name = setExpandSingleSchemaField)]
    pub fn set_expand_single_schema_field(
        mut self,
        value: JsValue,
    ) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_expand_single_schema_field".to_string(), value).map(|val| {
            self.expand_single_schema_field = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setJsonSampleExpandLevel)]
    pub fn set_json_sample_expand_level(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_u32_option_for("set_json_sample_expand_level".to_string(), value).map(|val| {
            self.json_sample_expand_level = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setHideSchemaTitles)]
    pub fn set_hide_schema_titles(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_hide_schema_titles".to_string(), value).map(|val| {
            self.hide_schema_titles = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setSimpleOneOfTypeLabel)]
    pub fn set_simple_one_of_type_label(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_simple_one_of_type_label".to_string(), value).map(|val| {
            self.simple_one_of_type_label = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setLazyRendering)]
    pub fn set_lazy_rendering(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_lazy_rendering".to_string(), value).map(|val| {
            self.lazy_rendering = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setMenuToggle)]
    pub fn set_menu_toggle(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_menu_toggle".to_string(), value).map(|val| {
            self.menu_toggle = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setNativeScrollbars)]
    pub fn set_native_scrollbars(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_native_scrollbars".to_string(), value).map(|val| {
            self.native_scrollbars = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setNoAutoAuth)]
    pub fn set_no_auto_auth(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_no_auto_auth".to_string(), value).map(|val| {
            self.no_auto_auth = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setOnlyRequiredInSamples)]
    pub fn set_only_required_in_samples(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_only_required_in_samples".to_string(), value).map(|val| {
            self.only_required_in_samples = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setPathInMiddlePanel)]
    pub fn set_path_in_middle_panel(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_path_in_middle_panel".to_string(), value).map(|val| {
            self.path_in_middle_panel = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setRequiredPropsFirst)]
    pub fn set_required_props_first(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_required_props_first".to_string(), value).map(|val| {
            self.required_props_first = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setScrollYOffset)]
    pub fn set_scroll_y_offset(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_string_option_for("set_scroll_y_offset".to_string(), value).map(|val| {
            self.scroll_y_offset = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setSelector)]
    pub fn set_selector(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_string_option_for("set_selector".to_string(), value).map(|val| {
            self.selector = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setFunction)]
    pub fn set_function(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_option_for("set_function".to_string(), value, "function", |v| {
            v.clone().dyn_into::<js_sys::Function>().ok()
        })
        .map(|val| {
            self.function = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setShowExtensions)]
    pub fn set_show_extensions(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        let mut extensions = vec![];
        if value.is_array() {
            let array = js_sys::Array::from(&value);
            for i in 0..array.length() {
                let val = array.get(i);
                if val.is_string() {
                    extensions.push(val.as_string().unwrap());
                }
            }
        } else {
            return Err(JsValue::from_str(
                "set_show_extensions only accepts array values",
            ));
        }
        self.show_extensions = Some(extensions);
        Ok(self)
    }

    #[wasm_bindgen(js_name = setSortPropsAlphabetically)]
    pub fn set_sort_props_alphabetically(
        mut self,
        value: JsValue,
    ) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_sort_props_alphabetically".to_string(), value).map(|val| {
            self.sort_props_alphabetically = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setPayloadSampleIdx)]
    pub fn set_payload_sample_idx(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_u32_option_for("set_payload_sample_idx".to_string(), value).map(|val| {
            self.payload_sample_idx = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setUntrustedSpec)]
    pub fn set_untrusted_spec(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_untrusted_spec".to_string(), value).map(|val| {
            self.untrusted_spec = Some(val);
            self
        })
    }

    #[wasm_bindgen(js_name = setRedocVersion)]
    pub fn set_redoc_version(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_string_option_for("set_redoc_version".to_string(), value).map(|val| {
            self.redocTryItOut.redoc_version = val;
            self
        })
    }

    #[wasm_bindgen(js_name = setTryItOutEnabled)]
    pub fn set_try_it_out_enabled(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_try_it_out_enabled".to_string(), value).map(|val| {
            self.redocTryItOut.try_it_out_enabled = val;
            self
        })
    }

    #[wasm_bindgen(js_name = setTryItBoxContainerId)]
    pub fn set_try_it_box_container_id(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_string_option_for("set_try_it_box_container_id".to_string(), value).map(|val| {
            self.redocTryItOut.try_it_box_container_id = val;
            self
        })
    }

    #[wasm_bindgen(js_name = setContainerId)]
    pub fn set_container_id(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_string_option_for("set_container_id".to_string(), value).map(|val| {
            self.redocTryItOut.container_id = val;
            self
        })
    }

    #[wasm_bindgen(js_name = setOperationBoxSelector)]
    pub fn set_operation_box_selector(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_string_option_for("set_operation_box_selector".to_string(), value).map(|val| {
            self.redocTryItOut.operation_box_selector = val;
            self
        })
    }

    #[wasm_bindgen(js_name = setSelectedOperationClass)]
    pub fn set_selected_operation_class(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_string_option_for("set_selected_operation_class".to_string(), value).map(|val| {
            self.redocTryItOut.selected_operation_class = val;
            self
        })
    }
}

impl Default for RedocOptions {
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
            redocTryItOut: RedocTryItOutOptions::default(),
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

    pub async fn init(&self, config: RedocOptions) -> Result<(), JsValue> {
        log(config
            .disable_search
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
