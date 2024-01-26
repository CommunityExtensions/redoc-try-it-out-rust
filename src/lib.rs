use wasm_bindgen::prelude::*;
use web_sys::{js_sys, window, Document, Event, HtmlScriptElement};

#[wasm_bindgen]
pub struct RedocTryItOut {
    document: Document,
}

#[wasm_bindgen]
impl RedocTryItOut {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<RedocTryItOut, JsValue> {
        let window = window().ok_or("no global `window` exists")?;
        let document = window.document().ok_or("should have a document on window")?;
        Ok(RedocTryItOut { document })
    }

    pub fn add_script_tag(&self, url: String, onload: js_sys::Function) -> Result<(), JsValue> {
        let script = self.document.create_element("script")?.dyn_into::<HtmlScriptElement>()?;
        script.set_src(&url);

        // Set up load event listener
        let closure = Closure::wrap(Box::new(move |event: Event| {
            onload.call1(&event.target().unwrap(), &JsValue::NULL).unwrap();
        }) as Box<dyn FnMut(Event)>);

        script.add_event_listener_with_callback("load", closure.as_ref().unchecked_ref())?;
        closure.forget(); // Important! Prevents the closure from being garbage-collected

        // Append the script to the body
        let body = self.document.body().expect("document should have a body");
        body.append_child(&script)?;
        Ok(())
    }
}