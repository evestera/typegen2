use wasm_bindgen::prelude::*;
use typegen_core::typegen;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run(
    _name: &str,
    input: &str,
    _options: &str
) -> String {
    typegen(input.as_bytes())
}

#[wasm_bindgen]
pub fn init() {
    #[cfg(feature = "debug")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    #[cfg(feature = "debug")]
    console_log::init_with_level(log::Level::Trace).expect("Unable to initialize console_log");
}
