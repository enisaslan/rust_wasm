use std::slice::Windows;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsValue};
use web_sys::{console, HtmlButtonElement, Document, MouseEvent, Window, Event, HtmlElement};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

static mut counter:i32 = 0;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().unwrap();
    let document: Document = window.document().unwrap();
    let body = document.body().unwrap();

    let val: web_sys::Element = document.create_element("p")?;
    let increment = document.create_element("button")?;
    let decrement = document.create_element("button")?;

    body.append_child(&increment)?;
    body.append_child(&decrement)?;
    body.append_child(&val)?;

    val.set_text_content(Some("Value: 0"));
    increment.set_text_content(Some(" + "));
    decrement.set_text_content(Some(" - "));
    
    //let valy = val.clone();
    let closure_inc = Closure::wrap(Box::new(move |_eventi: Event| 
    {
        unsafe{
            counter +=1;
            web_sys::console::log_1(&format!("Counter Value: {}", counter).into());
            //val.set_text_content(Some(&format!("Value: {counter}")));
        }

    }) as Box<dyn FnMut(_)>);

    increment.add_event_listener_with_callback("click", closure_inc.as_ref().unchecked_ref())
    .expect("Failed to add event listener");
    closure_inc.forget();

    let closure_dec = Closure::wrap(Box::new(move |_eventxd: Event| 
    {
        unsafe{
            counter -=1;
            web_sys::console::log_1(&format!("Counter Value: {}", counter).into());
            //val.set_text_content(Some(&format!("Value: {counter}")));
        }
    }) as Box<dyn FnMut(_)>);

    decrement.add_event_listener_with_callback("click", closure_dec.as_ref().unchecked_ref())
    .expect("Failed to add event listener");
    closure_dec.forget();

    Ok(())
}
