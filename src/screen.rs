use wasm_bindgen::prelude::*;
use web_sys::{
    js_sys,
    HtmlCanvasElement,
    WebGl2RenderingContext,
    WebGlProgram,
    WebGlShader, Window
};

#[wasm_bindgen]
pub struct Screen {
    canvas: HtmlCanvasElement,
    gl_context: WebGl2RenderingContext,     // WebGL context to use for rendering
}

#[wasm_bindgen]
impl Screen {
    /*
    Grabs the WebGL context from the Canvas
     */
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let context = canvas.
            get_context("webgl2")
                .unwrap()
                .expect("Could not obtain webgl context from canvas ...");
        Self {
            canvas: canvas,
            gl_context: context.dyn_into::<WebGl2RenderingContext>().expect("Error assigning gl context"),
        }
    }

    #[wasm_bindgen]
    pub fn resize(&self, window: Window) -> () {
        self.canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
        self.canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);
    }

    #[wasm_bindgen]
    pub fn draw_triangle(&self) -> Result<(), JsValue> {
        let context = &self.gl_context;

        let vert_shader = compile_shader(
            context,
            WebGl2RenderingContext::VERTEX_SHADER,
            r##"#version 300 es
     
            in vec4 position;
    
            void main() {
            
                gl_Position = position;
            }
            "##,
        )?;
    
        let frag_shader = compile_shader(
            context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r##"#version 300 es
        
            precision highp float;
            out vec4 outColor;
            
            void main() {
                outColor = vec4(1, 1, 1, 1);
            }
            "##,
        )?;
        let program = link_program(context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));
    
        let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    
        let position_attribute_location = context.get_attrib_location(&program, "position");
        let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    
        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);
    
            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    
        let vao = context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        context.bind_vertex_array(Some(&vao));
    
        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location as u32);
    
        context.bind_vertex_array(Some(&vao));
    
        let vert_count = (vertices.len() / 3) as i32;
        draw(&context, vert_count);
    
        Ok(())
    }
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}