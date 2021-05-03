#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;
extern crate yew_material_macro;

pub mod index;
pub mod theme;

use index::Index;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::utils::document;
use yew_material_utils::log;

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    log::initialize();
    theme::initialize();
    App::<Index>::new().mount(document().get_element_by_id("root").unwrap());
    Ok(())
}
