use std::slice::Windows;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsValue};
use web_sys::{Document, MouseEvent, Window};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    let button = document.create_element("button")?;

    val.set_text_content(Some("Hello from Rust!"));
    button.set_text_content(Some("Click Me !"));

    body.append_child(&val)?;
    body.append_child(&button)?;

    Ok(())
}
