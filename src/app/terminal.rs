use wasm_bindgen::prelude::*;

use crate::webgl::viewport::*;
use crate::utils::*;


#[wasm_bindgen]
pub struct Terminal {
    view_port: ViewPort     // Provides abstraction to WebGl api
}

#[wasm_bindgen]
impl Terminal {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let viewport = ViewPort::new();
        if viewport.is_err() {
            alert("ViewPort could not be initialized! Aborting ...");
            panic!();
        }

        Self { view_port: viewport.unwrap() }
    }

    pub fn resize(&self, width: u32, height: u32) -> () {
        self.view_port.resize(width, height);
    }

    pub fn draw_triangle(&self) -> () {

    } 
}