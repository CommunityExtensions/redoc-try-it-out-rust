use wasm_bindgen::prelude::*;
use web_sys::{window, js_sys, Event};
use web_sys::HtmlScriptElement;

#[wasm_bindgen]
pub fn create_hello_world_div() {
    let window = window().expect("should have a Window");
    let document = window.document().expect("should have a Document");
    let body = document.body().expect("should have a Body");
    let val = document.create_element("div").unwrap();
    val.set_inner_html("Hello World!");
    body.append_child(&val).expect("should append child");
}

#[wasm_bindgen]
pub fn append_script_and_wait(url: String, onload: js_sys::Function) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Create script element
    let script = document.create_element("script")?.dyn_into::<HtmlScriptElement>()?;
    script.set_src(&url);

    // Set up load event listener
    let closure = Closure::wrap(Box::new(move |event: Event| {
        onload.call1(&event.target().unwrap(), &JsValue::NULL).unwrap();
    }) as Box<dyn FnMut(Event)>);

    script.add_event_listener_with_callback("load", closure.as_ref().unchecked_ref())?;
    closure.forget(); // Important! Prevents the closure from being garbage-collected

    // Append the script to the body
    body.append_child(&script)?;

    Ok(())
}