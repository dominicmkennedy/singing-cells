use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub struct GL {
    context: WebGl2RenderingContext,
    program: WebGlProgram,
    canvas_width: i32,
    canvas_height: i32,
}

impl GL {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> GL {
        let context = Self::get_gl_context(canvas);
        let device_pixel_ratio = web_sys::window().unwrap().device_pixel_ratio();
        let canvas_width = (context
            .canvas()
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap()
            .client_width() as f64
            * device_pixel_ratio) as i32;
        let canvas_height = (context
            .canvas()
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap()
            .client_height() as f64
            * device_pixel_ratio) as i32;
        Self::set_canvas_res(&context, canvas_width, canvas_height);
        GL {
            program: Self::get_program(&context),
            context,
            canvas_width,
            canvas_height,
        }
    }

    pub fn get_aspect_ratio(&self) -> f64 {
        self.canvas_height as f64 / self.canvas_width as f64
    }

    fn set_canvas_res(canvas: &WebGl2RenderingContext, canvas_width: i32, canvas_height: i32) {
        canvas
            .canvas()
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap()
            .set_width(canvas_width as u32);
        canvas
            .canvas()
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap()
            .set_height(canvas_height as u32);
        canvas.viewport(0, 0, canvas_width, canvas_height);
    }

    fn get_gl_context(canvas: web_sys::HtmlCanvasElement) -> WebGl2RenderingContext {
        canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap()
    }

    fn get_program(gl: &WebGl2RenderingContext) -> WebGlProgram {
        let vert_shader = Self::compile_shader(
            &gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            include_str!("./shaders/CA.vert"),
        )
        .unwrap();
        let frag_shader = Self::compile_shader(
            &gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            include_str!("./shaders/CA.frag"),
        )
        .unwrap();

        let program = Self::link_program(&gl, &vert_shader, &frag_shader).unwrap();
        gl.use_program(Some(&program));

        program
    }

    fn compile_shader(
        gl: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = gl
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(gl
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    fn link_program(
        gl: &WebGl2RenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = gl
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        gl.attach_shader(&program, vert_shader);
        gl.attach_shader(&program, frag_shader);
        gl.link_program(&program);

        if gl
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }

    pub fn attach_verts(&self, vertices: &Vec<f32>) -> Result<(), JsValue> {
        let position_attribute_location =
            self.context.get_attrib_location(&self.program, "position");
        let vert_buffer = self
            .context
            .create_buffer()
            .ok_or("Failed to create buffer")?;
        let vert_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>()?
                .buffer();
            let vertices_location = vertices.as_ptr() as u32 / 4;
            js_sys::Float32Array::new(&memory_buffer)
                .subarray(vertices_location, vertices_location + vertices.len() as u32)
        };

        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vert_buffer));
        self.context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        self.context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.context
            .enable_vertex_attrib_array(position_attribute_location as u32);
        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        Ok(())
    }

    // TODO break this into two functions
    // so there's less messing with buffers between frames
    pub fn attach_cell_type_ptr(&self, ct_ptr: *const u32, ct_len: usize) -> Result<(), JsValue> {
        let cell_type_attribute_location = self
            .context
            .get_attrib_location(&self.program, "cellTypeVert");
        let cell_type_buffer = self
            .context
            .create_buffer()
            .ok_or("Failed to create buffer")?;
        let cell_type_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>()?
                .buffer();
            let cell_type_location = ct_ptr as u32 / 4;
            js_sys::Uint32Array::new(&memory_buffer)
                .subarray(cell_type_location, cell_type_location + ct_len as u32)
        };
        self.context.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&cell_type_buffer),
        );
        self.context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &cell_type_array,
            WebGl2RenderingContext::DYNAMIC_DRAW,
        );
        self.context.vertex_attrib_i_pointer_with_i32(
            cell_type_attribute_location as u32,
            1,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
            0,
        );
        self.context
            .enable_vertex_attrib_array(cell_type_attribute_location as u32);
        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        Ok(())
    }

    pub fn draw(&self, vert_count: i32) {
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
    }
}
