use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use chrono::prelude::*;
use web_sys::{Window, Document};

#[wasm_bindgen]
pub struct Terminal {
    window: Window,
    commands: HashMap<&'static str, String>,
}

#[wasm_bindgen]
impl Terminal {
    #[wasm_bindgen(constructor)]
    pub fn new(window: Window) -> Self {
        let mut commands = HashMap::new();
        commands.insert("hello", "Hello World!".to_string());
        commands.insert("time", Utc::now().to_string());

        Terminal { window, commands }
    }

    #[wasm_bindgen]
    pub fn handle_command(&self, command: &str) -> String {
        if let Some(response) = self.commands.get(command) {
            (response).to_string()
        } else {
            "Command not found".to_string()
        }
    }
}