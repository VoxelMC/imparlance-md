use wasm_bindgen::prelude::*;
mod parser;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//
//     // The `console.log` is quite polymorphic, so we can bind it with multiple
//     // signatures. Note that we need to use `js_name` to ensure we always call
//     // `log` in JS.
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_u32(a: u32);
//
//     // Multiple arguments too!
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_many(a: &str, b: &str);
// }

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    log(&format!("Hello to {} from Rust!", name));
    String::from("Testing Return Type")
}

#[wasm_bindgen]
pub fn parse_markdown(markdown_text: &str) -> String {
    parser::parse(markdown_text.to_string())
}
