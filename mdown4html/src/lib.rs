// Código baseado no exemplo:
//   https://github.com/rustwasm/wasm-bindgen/tree/main/examples/hello_world

use wasm_bindgen;
use wasm_bindgen::prelude::*;

// Import our markdown parser library, crate
extern crate pulldown_cmark;
use pulldown_cmark::{html, Parser};

// Import web_sys to access soma DOM objects
use web_sys;

#[wasm_bindgen]
extern "C" { // Esta é a referência para a função alert() do JavaScript
fn alert(s: &str);
}

fn console_log(msg: &str) -> Result<(), JsValue> {
    // let s_slice: &str = &msg[..];
    let array = js_sys::Array::new();
    array.push(&(wasm_bindgen::JsValue::from_str(msg)));
    web_sys::console::log(&array);
    Ok(())
}

#[wasm_bindgen]
pub fn greet(name: &str) -> Result<(), JsValue> {
    // Esta é a referência para uma função do modulo Rust que poderá
    // ser chamada à partir do JavaScript
    // alert(&format!("Alô, {}!", name));
    console_log(&format!("*** Hello, {}!", name));
    console_log("*** Getting the window object");
    let window = web_sys::window().expect("No window exists");
    console_log("*** Getting the document object");
    // window.confirm_with_message("Getting the document object");
    let document = window.document().expect("Window should have a document");
    console_log("*** Getting the body object");
    let body = document.body().expect("document should have a body");
    console_log("*** Creating a p TAG object");
    let val = document.create_element("p")?;
    console_log("*** Setting inner HTML on p TAG object");
    val.set_inner_html("Hello World from Webassembly Rust code!");
    console_log("*** Appending p TAG object into body object");
    // window.confirm_with_message("Appending p TAG object into body object");
    body.append_child(&val)?;
    Ok(())
}

#[wasm_bindgen]
pub fn render_markdown(markdown: String) -> String {
    // Esta é a referência para uma função do modulo Rust que poderá
    // ser chamada à partir do JavaScript
    let mut html_buf = String::new();
    let parser = Parser::new(&markdown[..]);
    html::push_html(&mut html_buf, parser);
    console_log("*** markdown String parsed using pulldown_cmark module");
    html_buf
}

