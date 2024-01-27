use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, window, Document, Element, HtmlScriptElement};

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

    #[wasm_bindgen(js_name = init, js_namespace = Redoc)]
    async fn initRedoc(
        docUrl: String,
        options: JsValue,
        element: Element,
        callback: js_sys::Function,
    );
}

#[wasm_bindgen]
pub struct RedocTryItOut {
    document: Document,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize)]
pub struct ThemOptions {
    spacing: Spacing,
    breakpoints: Breakpoints,
    colors: Colors,
    typography: Typography,
    menu: Menu,
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
            menu: Default::default(),
            logo: Default::default(),
            right_panel: Default::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize)]
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
#[derive(Deserialize, Serialize)]
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
#[derive(Deserialize, Serialize)]
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
#[derive(Clone, Deserialize, Serialize)]
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
#[derive(Clone, Deserialize, Serialize)]
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
#[derive(Clone, Deserialize, Serialize)]
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
#[derive(Clone, Deserialize, Serialize)]
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
#[derive(Deserialize, Serialize)]
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
#[derive(Clone, Deserialize, Serialize)]
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
#[derive(Clone, Deserialize, Serialize)]
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
#[derive(Clone, Deserialize, Serialize)]
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
#[derive(Deserialize, Serialize)]
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
#[derive(Deserialize, Serialize)]
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

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize)]
pub struct DependenciesVersions {
    pub jquery: String,
    pub jquery_scroll_to: String,
}

impl Default for DependenciesVersions {
    fn default() -> Self {
        Self {
            jquery: "3.5.1".to_string(),
            jquery_scroll_to: "2.1.3".to_string(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize)]
pub struct AuthBtnOptions {
    pub pos_selector: Option<String>,
    pub text: Option<String>,
    pub class_name: Option<String>,
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
#[derive(Deserialize, Serialize)]
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
#[derive(Deserialize, Serialize)]
pub struct SwaggerOptions {
    url: Option<String>,
    dom_id: Option<String>,
    version: Option<String>,
    authorize_btn_selector: Option<String>,
    authorize_done_btn_selector: Option<String>,
    authorize_modal_selector: Option<String>,
    authorize_modal_close_btn_selector: Option<String>,
    operation_section_container_selector: Option<String>,
    operation_container_selector: Option<String>,
    operation_summary_pattern_selector: Option<String>,
    hide_class: Option<String>,
    show_class: Option<String>,
    auth_modal_class: Option<String>,
    selected_operation_container_class: Option<String>,
    wrapper_selector: Option<String>,
}

impl Default for SwaggerOptions {
    fn default() -> Self {
        Self {
            url: Option::default(),
            dom_id: Option::default(),
            version: Option::default(),
            authorize_btn_selector: Option::default(),
            authorize_done_btn_selector: Option::default(),
            authorize_modal_selector: Option::default(),
            authorize_modal_close_btn_selector: Option::default(),
            operation_section_container_selector: Option::default(),
            operation_container_selector: Option::default(),
            operation_summary_pattern_selector: Option::default(),
            hide_class: Option::default(),
            show_class: Option::default(),
            auth_modal_class: Option::default(),
            selected_operation_container_class: Option::default(),
            wrapper_selector: Option::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize)]
pub struct StyleMatcherOptions {
    information_container_target_selector: Option<String>,
    auth_wrapper_target_selector: Option<String>,
    models_container_target_selector: Option<String>,
    input_target_selector: Option<String>,
    select_target_selector: Option<String>,
    text_area_target_selector: Option<String>,
    paragraph_target_selector: Option<String>,
    execute_btn_target_selector: Option<String>,
    response_container_target_selector: Option<String>,
    response_title_target_selector: Option<String>,
    response_header_target_selector: Option<String>,
    response_table_target_selector: Option<String>,
    response_wrapper_target_selector: Option<String>,
    response_wrapper_result_target_selector: Option<String>,
    response_microlight_target_selector: Option<String>,
    response_code_target_selector: Option<String>,
    response_clipboard_target_selector: Option<String>,
    response_clipboard_btn_target_selector: Option<String>,
    response_curl_clipboard_target_selector: Option<String>,
    response_download_target_selector: Option<String>,
    server_response_header_target_selector: Option<String>,
    server_response_status_target_selector: Option<String>,
    server_response_description_target_selector: Option<String>,
    server_response_sub_header_target_selector: Option<String>,
    clear_btn_target_selector: Option<String>,
    operation_tag_target_selector: Option<String>,
    operation_header_container_target_selector: Option<String>,
    operation_header_target_selector: Option<String>,
    operation_header_decoration_target_selector: Option<String>,
    operation_try_out_target_selector: Option<String>,
    description_container_target_selector: Option<String>,
    summary_target_selector: Option<String>,
    modal_header_container_target_selector: Option<String>,
    modal_header_target_selector: Option<String>,
    modal_title_target_selector: Option<String>,
    modal_title_code_target_selector: Option<String>,
    modal_label_target_selector: Option<String>,
    modal_code_target_selector: Option<String>,
    modal_btn_target_selector: Option<String>,
    parameters_table_container_target_selector: Option<String>,
    parameters_head_target_selector: Option<String>,
    parameter_name_field_target_selector: Option<String>,
    parameter_type_field_target_selector: Option<String>,
    parameter_deprecated_target_selector: Option<String>,
    parameter_source_target_selector: Option<String>,
    parameter_required_marker_target_selector: Option<String>,
    parameter_required_target_selector: Option<String>,
    api_content_source_selector: Option<String>,
    input_source_selector: Option<String>,
    code_source_selector: Option<String>,
    code_box_source_selector: Option<String>,
    data_section_source_selector: Option<String>,
    field_source_selector: Option<String>,
    required_field_source_selector: Option<String>,
    field_marker_source_selector: Option<String>,
    san_serif_font_source_selector: Option<String>,
    alternative_monospace_font_source_selector: Option<String>,
    alternative_sans_serif_source_selector: Option<String>,
    h2_source_selector: Option<String>,
    h3_source_selector: Option<String>,
    h5_source_selector: Option<String>,
    label_source_selector: Option<String>,
    type_source_selector: Option<String>,
    btn_source_selector: Option<String>,
    default_border_color_selector: Option<String>,
    auth_btn_source_selector: Option<String>,
    http_verb_source_selector: Option<String>,
}

impl Default for StyleMatcherOptions {
    fn default() -> Self {
        Self {
            information_container_target_selector: Option::default(),
            auth_wrapper_target_selector: Option::default(),
            models_container_target_selector: Option::default(),
            input_target_selector: Option::default(),
            select_target_selector: Option::default(),
            text_area_target_selector: Option::default(),
            paragraph_target_selector: Option::default(),
            execute_btn_target_selector: Option::default(),
            response_container_target_selector: Option::default(),
            response_title_target_selector: Option::default(),
            response_header_target_selector: Option::default(),
            response_table_target_selector: Option::default(),
            response_wrapper_target_selector: Option::default(),
            response_wrapper_result_target_selector: Option::default(),
            response_microlight_target_selector: Option::default(),
            response_code_target_selector: Option::default(),
            response_clipboard_target_selector: Option::default(),
            response_clipboard_btn_target_selector: Option::default(),
            response_curl_clipboard_target_selector: Option::default(),
            response_download_target_selector: Option::default(),
            server_response_header_target_selector: Option::default(),
            server_response_status_target_selector: Option::default(),
            server_response_description_target_selector: Option::default(),
            server_response_sub_header_target_selector: Option::default(),
            clear_btn_target_selector: Option::default(),
            operation_tag_target_selector: Option::default(),
            operation_header_container_target_selector: Option::default(),
            operation_header_target_selector: Option::default(),
            operation_header_decoration_target_selector: Option::default(),
            operation_try_out_target_selector: Option::default(),
            description_container_target_selector: Option::default(),
            summary_target_selector: Option::default(),
            modal_header_container_target_selector: Option::default(),
            modal_header_target_selector: Option::default(),
            modal_title_target_selector: Option::default(),
            modal_title_code_target_selector: Option::default(),
            modal_label_target_selector: Option::default(),
            modal_code_target_selector: Option::default(),
            modal_btn_target_selector: Option::default(),
            parameters_table_container_target_selector: Option::default(),
            parameters_head_target_selector: Option::default(),
            parameter_name_field_target_selector: Option::default(),
            parameter_type_field_target_selector: Option::default(),
            parameter_deprecated_target_selector: Option::default(),
            parameter_source_target_selector: Option::default(),
            parameter_required_marker_target_selector: Option::default(),
            parameter_required_target_selector: Option::default(),
            api_content_source_selector: Option::default(),
            input_source_selector: Option::default(),
            code_source_selector: Option::default(),
            code_box_source_selector: Option::default(),
            data_section_source_selector: Option::default(),
            field_source_selector: Option::default(),
            required_field_source_selector: Option::default(),
            field_marker_source_selector: Option::default(),
            san_serif_font_source_selector: Option::default(),
            alternative_monospace_font_source_selector: Option::default(),
            alternative_sans_serif_source_selector: Option::default(),
            h2_source_selector: Option::default(),
            h3_source_selector: Option::default(),
            h5_source_selector: Option::default(),
            label_source_selector: Option::default(),
            type_source_selector: Option::default(),
            btn_source_selector: Option::default(),
            default_border_color_selector: Option::default(),
            auth_btn_source_selector: Option::default(),
            http_verb_source_selector: Option::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize)]
pub struct RedocTryItOutOptions {
    doc_url: String,
    redoc_version: String,
    try_it_out_enabled: bool,
    try_it_box_container_id: String,
    container_id: String,
    operation_box_selector: String,
    selected_operation_class: String,
    dependencies_versions: DependenciesVersions,
    auth_btn: AuthBtnOptions,
    try_btn: TryBtnOptions,
    swagger_options: SwaggerOptions,
    styler_matcher: StyleMatcherOptions,
}

impl Default for RedocTryItOutOptions {
    fn default() -> Self {
        Self {
            doc_url: "https://petstore.swagger.io/v2/swagger.json".to_string(),
            redoc_version: "2.1.3".to_string(),
            try_it_out_enabled: true,
            try_it_box_container_id: "try-out-wrapper".to_string(),
            container_id: "redoc-container".to_string(),
            operation_box_selector: "[data-section-id]".to_string(),
            selected_operation_class: "try".to_string(),
            dependencies_versions: Default::default(),
            auth_btn: Default::default(),
            try_btn: Default::default(),
            swagger_options: Default::default(),
            styler_matcher: Default::default(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize, Serialize)]
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
    theme: ThemOptions,
    /** if set, the spec is considered untrusted and all HTML/markdown is sanitized to prevent XSS.
     * Disabled by default for performance reasons. Enable this option if you work with untrusted user data!
     **/
    untrusted_spec: Option<bool>,
    /** RedocTryItOut options */
    _redoc_try_it_out: RedocTryItOutOptions,
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
            self._redoc_try_it_out.redoc_version = val;
            self
        })
    }

    #[wasm_bindgen(js_name = setTryItOutEnabled)]
    pub fn set_try_it_out_enabled(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_boolean_option_for("set_try_it_out_enabled".to_string(), value).map(|val| {
            self._redoc_try_it_out.try_it_out_enabled = val;
            self
        })
    }

    #[wasm_bindgen(js_name = setTryItBoxContainerId)]
    pub fn set_try_it_box_container_id(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_string_option_for("set_try_it_box_container_id".to_string(), value).map(|val| {
            self._redoc_try_it_out.try_it_box_container_id = val;
            self
        })
    }

    #[wasm_bindgen(js_name = setContainerId)]
    pub fn set_container_id(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_string_option_for("set_container_id".to_string(), value).map(|val| {
            self._redoc_try_it_out.container_id = val;
            self
        })
    }

    #[wasm_bindgen(js_name = setOperationBoxSelector)]
    pub fn set_operation_box_selector(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_string_option_for("set_operation_box_selector".to_string(), value).map(|val| {
            self._redoc_try_it_out.operation_box_selector = val;
            self
        })
    }

    #[wasm_bindgen(js_name = setSelectedOperationClass)]
    pub fn set_selected_operation_class(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_string_option_for("set_selected_operation_class".to_string(), value).map(|val| {
            self._redoc_try_it_out.selected_operation_class = val;
            self
        })
    }

    #[wasm_bindgen(js_name = setDocUrl)]
    pub fn set_doc_url(mut self, value: JsValue) -> Result<RedocOptions, JsValue> {
        parse_string_option_for("set_doc_url".to_string(), value).map(|val| {
            self._redoc_try_it_out.doc_url = val;
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
            show_extensions: Option::default(),
            sort_props_alphabetically: Option::default(),
            payload_sample_idx: Option::default(),
            theme: ThemOptions::default(),
            untrusted_spec: Option::default(),
            _redoc_try_it_out: RedocTryItOutOptions::default(),
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
            format!("https://cdn.jsdelivr.net/npm/redoc@{}/bundles/redoc.standalone.min.js", config._redoc_try_it_out.redoc_version),
        )
        .await?;
        let callback = Closure::wrap(Box::new(move |e: JsValue| {
            if e.is_instance_of::<js_sys::Error>() {
                log(format!("Error: {}", e.as_string().unwrap().as_str()).as_str());
            } else {
                log(format!("Redoc OK").as_str());
            }
        }) as Box<dyn FnMut(JsValue)>);
        let options = serde_wasm_bindgen::to_value(&config).unwrap();
        let redoc_container = self
            .document
            .get_element_by_id(config._redoc_try_it_out.container_id.as_str())
            .expect("should have a redoc container");
        let callback_function = callback
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .clone();
        initRedoc(
            config._redoc_try_it_out.doc_url,
            options,
            redoc_container,
            callback_function,
        )
        .await;
        callback.forget();
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
