extern crate wasm_bindgen;
mod gl_setup;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

mod programs;
mod shaders;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct MyClient {
    gl: WebGlRenderingContext,
}

#[wasm_bindgen]
impl MyClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // to get console error messeges
        console_error_panic_hook::set_once();
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();
        Self { gl }
    }

    pub fn update(&mut self, _item: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT)
    }
}
