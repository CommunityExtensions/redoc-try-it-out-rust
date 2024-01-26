use wasm_bindgen::prelude::*;
use web_sys::{js_sys, window, Document, HtmlScriptElement};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub struct RedocTryItOut {
    document: Document,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(getter_with_clone)]
pub struct RedocTryItOutOptions {
    pub option1: String,
}

#[wasm_bindgen]
impl RedocTryItOutOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RedocTryItOutOptions {
        RedocTryItOutOptions::default()
    }
}

impl Default for RedocTryItOutOptions {
    fn default() -> Self {
        Self {
            option1: "".to_string(),
        }
    }
}

#[wasm_bindgen]
impl RedocTryItOut {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<RedocTryItOut, JsValue> {
        let window = window().ok_or("no global `window` exists")?;
        let document = window.document().ok_or("should have a document on window")?;
        Ok(RedocTryItOut { document })
    }

    pub async fn init(&self, config: RedocTryItOutOptions) -> Result<(), JsValue> {
        log(config.option1.as_str());
        self.add_script_tag("https://rebilly.github.io/ReDoc/releases/latest/redoc.min.js".to_string()).await?;
        Ok(())
    }

    async fn add_script_tag(&self, url: String) -> Result<(), JsValue> {
        let script = self.document.create_element("script")?.dyn_into::<HtmlScriptElement>()?;
        script.set_src(&url);

        let promise = js_sys::Promise::new(&mut |resolve, reject| {
            let onload = Closure::wrap(Box::new(move |_| {
                resolve.call0(&JsValue::NULL).unwrap();
            }) as Box<dyn FnMut(JsValue)>);
    
            let onerror = Closure::wrap(Box::new(move |_| {
                reject.call0(&JsValue::NULL).unwrap();
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