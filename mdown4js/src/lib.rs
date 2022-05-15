// Código baseado no exemplo:
//   https://github.com/rustwasm/wasm-bindgen/tree/main/examples/hello_world

use wasm_bindgen::prelude::*;
// Import our markdown parser library, crate
extern crate pulldown_cmark;
use pulldown_cmark::{html, Parser};

#[wasm_bindgen]
extern "C" { // Esta é a referência para a função alert() do JavaScript
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    // Esta é a referência para uma função do modulo Rust que poderá
    // ser chamada à partir do JavaScript
    alert(&format!("Alô, {}!", name));
}

#[wasm_bindgen]
pub fn render_markdown(markdown: String) -> String {
    // Esta é a referência para uma função do modulo Rust que poderá
    // ser chamada à partir do JavaScript
    let mut html_buf = String::new();
    let parser = Parser::new(&markdown[..]);
    html::push_html(&mut html_buf, parser);
    html_buf
}
