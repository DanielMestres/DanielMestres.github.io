use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::{
    WebGl2RenderingContext,
    Window,
    js_sys,
    HtmlCanvasElement
};

use crate::consts::{CANVAS_ID, WEB_GL_CONTEXT};

#[derive(Debug)]
pub enum ViewPortError {
    FailedToGetWindowElement,
    FailedToGetDocumentElement,
    FailedToGetCanvasElement,
    FailedToGetGlContext,
    FailedToConvertToHtmlCanvasElement
}

impl fmt::Display for ViewPortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViewPortError::FailedToGetWindowElement => write!(f, "Window could not be accessed ..."),
            ViewPortError::FailedToGetDocumentElement => write!(f, "Document could not be accessed ..."),
            ViewPortError::FailedToGetCanvasElement => write!(f, "Canvas could not be accessed ..."),
            ViewPortError::FailedToGetGlContext => write!(f, "Gl rendering context could not be accessed ..."),
            ViewPortError::FailedToConvertToHtmlCanvasElement => write!(f, "Cast to HtmlCanvasElement could not be made ...")
        }
    }
}

pub struct ViewPort {
    pub canvas: HtmlCanvasElement
}

impl ViewPort {
    /*
    Constructor for the ViewPort struct, performs the necessary calls
    to the WebGl api
     */
    pub fn new() -> Result<Self, ViewPortError> {
        // Get the document and canvas to init ViewPort instance
        let document = web_sys::window()
            .ok_or(ViewPortError::FailedToGetCanvasElement)?
            .document()
            .ok_or(ViewPortError::FailedToGetDocumentElement)?;

        let canvas = document
            .get_element_by_id(&CANVAS_ID)
            .ok_or(ViewPortError::FailedToGetCanvasElement)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ViewPortError::FailedToConvertToHtmlCanvasElement);

        Ok(Self { canvas: canvas.unwrap() })
    }

    pub fn resize(&self, width: u32, height: u32) -> () {
        self.canvas.set_width(width);
        self.canvas.set_height(height);
    }

    /*
    Clears the rendering context
    */
    pub fn clear(&self) -> () {
        let context = self.get_gl_context();
        if context.is_err() {
            panic!("Context could not be obtained ...")
        }

        context.unwrap()
            .clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    /*
    Recovers the gl context from the canvas
    */
    fn get_gl_context(&self) -> Result<WebGl2RenderingContext, ViewPortError> {
        let context = self.canvas
            .get_context(&WEB_GL_CONTEXT)
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        Ok(context)
    }

    // Add more methods as needed for interacting with the WebGL context
}