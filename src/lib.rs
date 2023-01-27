// TODO put this somewhere
// utils::set_panic_hook();
//
// TODO use Vec::with_capacity instead of vec!
// to construct
// or just use iters if posible
//
// use std::cell::RefCell;
// todo consider wrapping the universe in a ref cell
// aliviating extra cloning from the next_generation fn

mod utils;

use itertools::{iproduct, Itertools};
use js_sys::WebAssembly;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
// use js_sys::{Math::random, WebAssembly};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct CA {
    size: usize,
    num_steps: usize,
    // rule_density: f32,
    // num_cell_types: u8,
    rule_table: Vec<u8>,
    universe: Vec<Vec<u8>>,
    cell_types: Vec<u32>,
}

#[wasm_bindgen]
impl CA {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize, num_steps: usize, rule_density: f32, num_cell_types: u8) -> CA {
        CA {
            size,
            num_steps,
            // rule_density,
            // num_cell_types,
            rule_table: Self::gen_rule_table(num_cell_types, rule_density),
            universe: Self::gen_init_universe(size, num_steps, num_cell_types),
            cell_types: vec![0; size * num_steps * 6],
            // cell_types: Vec::with_capacity(size*num_steps*6),
        }
    }

    fn gen_init_universe(size: usize, num_steps: usize, num_cell_types: u8) -> Vec<Vec<u8>> {
        let mut u = vec![vec![0; size]; num_steps];
        u[num_steps - 1] = Self::gen_init_cells(num_cell_types, size);

        u
    }

    fn gen_rule_table(n: u8, rule_density: f32) -> Vec<u8> {
        let width = ((n as usize - 1) * 3) + 1;
        let mut table: Vec<u8> = (0..width).map(|_| gen_range(1, n)).collect();

        let mut to_remove: BTreeSet<usize> = BTreeSet::new();
        while to_remove.len() != ((width as f32) * (1.0 - rule_density)) as usize {
            to_remove.insert(gen_range(0, width as u8) as usize);
        }

        to_remove.iter().for_each(|&x| table[x] = 0);

        table
    }

    //tODO reverify that this indeed works
    fn next_generation(&mut self) {
        self.universe.rotate_left(1);

        let mut new_state = self.universe[self.num_steps - 2]
            .iter()
            .circular_tuple_windows::<(_, _, _)>()
            .map(|(x, y, z)| self.rule_table[(x + y + z) as usize])
            .collect::<Vec<_>>();

        new_state.rotate_right(1);
        self.universe[self.num_steps - 1] = new_state;
    }

    fn gen_init_cells(n: u8, width: usize) -> Vec<u8> {
        (0..width).map(|_| gen_range(0, n)).collect()
    }

    fn update_cell_colors(&mut self) {
        self.universe
            .iter()
            .flatten()
            .flat_map(|x| std::iter::repeat(*x as u32).take(6))
            .zip(self.cell_types.iter_mut())
            .for_each(|(u, ct)| *ct = u);
    }
}

#[wasm_bindgen]
pub struct MyGL {
    context: WebGl2RenderingContext,
    program: WebGlProgram,
    device_pixel_ratio: f64,
    canvas_width: i32,
    canvas_height: i32,
}

#[wasm_bindgen]
impl MyGL {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> MyGL {
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
        let x = MyGL {
            program: Self::get_program(&context),
            device_pixel_ratio,
            context,
            canvas_width,
            canvas_height,
        };
        x.set_canvas_res();
        x
    }

    fn set_canvas_res(&self) {
        self.context
            .canvas()
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap()
            .set_width(self.canvas_width as u32);
        self.context
            .canvas()
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap()
            .set_height(self.canvas_height as u32);
        self.context
            .viewport(0, 0, self.canvas_width, self.canvas_height);
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

    fn attach_verts(&self, vertices: &Vec<f32>) -> Result<(), JsValue> {
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

    fn attach_cell_type_ptr(&self, ct_ptr: *const u32, ct_len: usize) -> Result<(), JsValue> {
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

    fn draw(&self, vert_count: i32) {
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
    }
}

// generate a random number within a range
fn gen_range(low: u8, high: u8) -> u8 {
    let range = (high - low) as f64;
    ((random() * range) as u8) + low
}

// insane workaround
// probably due to something not linking properly
// hope to fix in the future
#[wasm_bindgen(inline_js = "export function random() { return Math.random(); }")]
extern "C" {
    fn random() -> f64;
}

fn cell_verts(universe_width: usize, time_steps: usize) -> Vec<f32> {
    let cell_width_px: f32 = 2.0 / (universe_width as f32);
    let cell_height_px: f32 = 2.0 / (time_steps as f32);

    iproduct!((0..time_steps).rev(), (0..universe_width), (0..12))
        .map(|(i, j, k)| match k {
            0 | 2 | 6  => (j as f32)       * cell_width_px,
            4 | 8 | 10 => ((j + 1) as f32) * cell_width_px,
            3 | 5 | 9  => (i as f32)       * cell_height_px,
            1 | 7 | 11 => ((i + 1) as f32) * cell_height_px,
            _ => 0.0,
        } - 1.0)
        .collect()
}

#[wasm_bindgen]
pub fn render(
    canvas: web_sys::HtmlCanvasElement,
    num_cell_types: u8,
    universe_width: usize,
    rule_density: f32,
) -> Result<(), JsValue> {
    utils::set_panic_hook();

    let x = MyGL::new(canvas);

    let max_time_steps =
        ((x.canvas_height as f64 * universe_width as f64) / x.canvas_width as f64) as usize;
    x.attach_verts(&cell_verts(universe_width, max_time_steps))?;

    let mut c = CA::new(universe_width, max_time_steps, rule_density, num_cell_types);
    log!("rules: {:?}", c.rule_table);
    // (0..max_time_steps - 1).for_each(|_| c.next_generation());
    // c.update_cell_colors();
    // x.attach_cell_type_ptr(c.cell_types.as_ptr(), c.cell_types.len())?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        render_loop(&mut c, &x);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn render_loop(c: &mut CA, x: &MyGL) {
    c.update_cell_colors();
    x.attach_cell_type_ptr(c.cell_types.as_ptr(), c.cell_types.len())
        .unwrap();
    x.draw((c.num_steps * c.size * 6) as i32);
    c.next_generation();
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}
