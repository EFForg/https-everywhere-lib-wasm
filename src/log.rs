use wasm_bindgen::prelude::*;
use std::panic;
extern crate console_error_panic_hook;
use wasm_bindgen_console_logger::DEFAULT_LOGGER;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    log::set_logger(&DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);
    Ok(())
}
