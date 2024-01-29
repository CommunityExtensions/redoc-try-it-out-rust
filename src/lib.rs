mod theme;
mod options;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, window, Document, Element, HtmlScriptElement};
use options::RedocTryItOutOptions;
use options::RedocOptions;

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