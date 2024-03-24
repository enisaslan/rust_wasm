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

// append html element to html body 
fn append_element_to_body(element: &HtmlElement){
    let document: Document = get_document().expect("document not found !");
    let body = document.body().unwrap();
    body.append_child(element).expect("Failed to append child element to body");
}

// get the html document object 
fn get_document() -> Option<Document> {
    web_sys::window()?.document()
}

// set id to html element
fn set_element_id(element: &HtmlElement, id: &str) {
    element.set_id(id);
}

// set text content in to html element
fn set_element_text(element: &HtmlElement, text: &str) {
    element.set_text_content(Some(text));
}

// create a html element 
fn create_html_element(tag_name: &str) -> Option<HtmlElement> {
    let document: Document = get_document()?;
    let element = document.create_element(tag_name).ok()?;
    element.dyn_into::<HtmlElement>().ok()
}

// get a html element by id
fn get_element_by_id(id: &str) -> Option<HtmlElement> {
    let document = web_sys::window()?.document()?;
    document.get_element_by_id(id).and_then(|elem| elem.dyn_into::<HtmlElement>().ok())
}

static mut counter:i32 = 0;

/** integer counter value update to html element  */
fn update_counter_display(id: &str) 
{
    unsafe {
        // Counter değerini alıyoruz
        let counter_value = counter;
        let str_data = format!("Value: {}",counter_value);
        // Counter değerini gösterecek olan <p> öğesini tespit ediyoruz
        if let Some(counter_p) = get_element_by_id(id) {
            // <p> öğesinin içeriğini güncelliyoruz
            //counter_p.set_inner_html(&counter_value.to_string());
            set_element_text(&counter_p, str_data.as_str());
        }
    }
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> 
{
    // create the increment button   
    if let Some(inc_btn) = create_html_element("button") {
        append_element_to_body(&inc_btn);
        set_element_id(&inc_btn, "inc_button");
        set_element_text(&inc_btn, " + ");

        let closure_inc = Closure::wrap(Box::new(move |_eventi: Event| 
            {
                unsafe{
                    counter +=1;
                    web_sys::console::log_1(&format!("Counter Value: {}", counter).into());
                    update_counter_display("counter_display");
                }
        
            }) as Box<dyn FnMut(_)>);
        
        inc_btn.add_event_listener_with_callback("click", closure_inc.as_ref().unchecked_ref())
        .expect("Failed to add event listener");
        closure_inc.forget();
    }
    
    // create the display paragraph 
    if let Some(paragraph) = create_html_element("p") {
        append_element_to_body(&paragraph);
        set_element_id(&paragraph, "counter_display");
        set_element_text(&paragraph, "Value: 0");
    }

    // create the decrement button 
    if let Some(dec_btn) = create_html_element("button") {
        append_element_to_body(&dec_btn);
        set_element_id(&dec_btn, "dec_button");
        set_element_text(&dec_btn, " - ");

        let closure_dec = Closure::wrap(Box::new(move |_eventxd: Event| 
        {
            unsafe{
                counter -=1;
                web_sys::console::log_1(&format!("Counter Value: {}", counter).into());
                update_counter_display("counter_display");
            }
        }) as Box<dyn FnMut(_)>);
    
        dec_btn.add_event_listener_with_callback("click", closure_dec.as_ref().unchecked_ref())
        .expect("Failed to add event listener");
        closure_dec.forget();
    }

    Ok(())
}
