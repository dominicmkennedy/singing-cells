const VERT_SHADER: &str = r##"#version 300 es
    in vec4 position;
    in vec4 cellType;

    out vec4 fragColor;

    void main() {
        fragColor = cellType;
        gl_Position = position;
    }
    "##;

const FRAG_SHADER: &str = r##"#version 300 es
    precision highp float;

    in vec4 fragColor;

    out vec4 outColor;
        
    void main() {
        outColor = fragColor;
    }
    "##;

mod utils;

use itertools::Itertools;
use js_sys::WebAssembly;
use std::collections::BTreeSet;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
// use js_sys::{Math::random, WebAssembly};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn gen_init_cells(n: u8, width: usize) -> Vec<u8> {
    (0..width).map(|_| gen_range(0, n)).collect()
}

fn gen_rule_table(n: u8, width: usize, rule_density: f32) -> Vec<u8> {
    let mut table: Vec<u8> = (0..width).map(|_| gen_range(1, n)).collect();

    let mut to_remove: BTreeSet<usize> = BTreeSet::new();
    while to_remove.len() != ((width as f32) * (1.0 - rule_density)) as usize {
        to_remove.insert(gen_range(0, width as u8) as usize);
    }

    to_remove.iter().for_each(|&x| table[x] = 0);

    table
}

// generate a random number within a range
fn gen_range(low: u8, high: u8) -> u8 {
    let range = (high - low) as f64;
    ((random() * range) as u8) + low
}

// next step function
fn next_generation(rule_table: &Vec<u8>, cell_board: &mut Vec<Vec<u8>>) {
    cell_board.rotate_left(1);
    let time_steps = cell_board.len();

    let mut old_state = cell_board[time_steps - 2].clone();
    old_state.rotate_right(1);
    let new_state_iter = cell_board[time_steps - 1].iter_mut();

    old_state
        .iter()
        .circular_tuple_windows::<(_, _, _)>()
        .zip(new_state_iter)
        .for_each(|((x, y, z), s)| *s = rule_table[(x + y + z) as usize]);
}

// insane workaround
// probably due to something not linking properly
// hope to fix in the future
#[wasm_bindgen(inline_js = "export function random() { return Math.random(); }")]
extern "C" {
    fn random() -> f64;
}

fn ca(
    universe_width: usize,
    max_time_steps: usize,
    num_cell_types: u8,
    rule_density: f32,
) -> Vec<f32> {
    let rule_table_width: usize = (((num_cell_types as usize) - 1) * 3) + 1;

    // rule table
    let rule_table = gen_rule_table(num_cell_types, rule_table_width, rule_density);
    log!("rules: {:?}", rule_table);

    let cell_board: &mut Vec<Vec<u8>> = &mut vec![vec![0; universe_width]; max_time_steps];
    cell_board[max_time_steps - 1] = gen_init_cells(num_cell_types, universe_width);

    (0..max_time_steps - 1).for_each(|_| next_generation(&rule_table, cell_board));

    // return results as verts and colors
    cell_colors(cell_board)
}

// rewrite; this is hidious
fn cell_verts(universe_width: usize, time_steps: usize) -> Vec<f32> {
    let num_verts: usize = universe_width * time_steps * 2 * 3 * 2;
    let mut vertices = vec![0.0; num_verts];

    let cell_width_px: f32 = 2.0 / (universe_width as f32);
    let cell_height_px: f32 = 2.0 / (time_steps as f32);

    for i in 0..time_steps {
        for j in 0..universe_width {
            let idx = (i + (j * time_steps)) * 12;
            let x0 = ((j as f32) * cell_width_px) - 1.0;
            let x1 = (((j + 1) as f32) * cell_width_px) - 1.0;

            let y0 = (((time_steps - i) as f32) * cell_height_px) - 1.0;
            let y1 = ((((time_steps - i) - 1) as f32) * cell_height_px) - 1.0;
            // triangle 1
            vertices[idx + 0] = x0; //x0
            vertices[idx + 1] = y0; //y0

            vertices[idx + 2] = x0; //x1
            vertices[idx + 3] = y1; //y1

            vertices[idx + 4] = x1; //x2
            vertices[idx + 5] = y1; //y2

            // triangidx
            vertices[idx + 6] = x0;
            vertices[idx + 7] = y0;

            vertices[idx + 8] = x1;
            vertices[idx + 9] = y1;

            vertices[idx + 10] = x1;
            vertices[idx + 11] = y0;
        }
    }

    vertices
}

// this function is an absolute disaster
fn cell_colors(cell_board: &mut Vec<Vec<u8>>) -> Vec<f32> {
    let color_palette: [[f32; 4]; 10] = [
        [0.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
        [1.0, 0.0, 1.0, 1.0],
        [0.0, 1.0, 1.0, 1.0],
        [0.0, 0.5, 0.5, 1.0],
        [0.5, 1.0, 0.0, 1.0],
        [0.5, 0.0, 1.0, 1.0],
        [0.22, 0.1, 0.8, 1.0],
        [0.7, 0.1, 0.2, 1.0],
    ];

    let time_steps = cell_board.len();
    let universe_width = cell_board[0].len();
    let num_colors: usize = time_steps * universe_width * 2 * 3 * 2 * 2;

    let mut colors = vec![0.0; num_colors];

    for i in 0..cell_board.len() {
        for j in 0..cell_board[i].len() {
            let idx = (i + (j * time_steps)) * 24;
            let cell = cell_board[i][j] as usize;

            colors[idx + 0] = color_palette[cell][0];
            colors[idx + 1] = color_palette[cell][1];
            colors[idx + 2] = color_palette[cell][2];
            colors[idx + 3] = color_palette[cell][3];

            colors[idx + 4] = color_palette[cell][0];
            colors[idx + 5] = color_palette[cell][1];
            colors[idx + 6] = color_palette[cell][2];
            colors[idx + 7] = color_palette[cell][3];

            colors[idx + 8] = color_palette[cell][0];
            colors[idx + 9] = color_palette[cell][1];
            colors[idx + 10] = color_palette[cell][2];
            colors[idx + 11] = color_palette[cell][3];

            colors[idx + 12] = color_palette[cell][0];
            colors[idx + 13] = color_palette[cell][1];
            colors[idx + 14] = color_palette[cell][2];
            colors[idx + 15] = color_palette[cell][3];

            colors[idx + 16] = color_palette[cell][0];
            colors[idx + 17] = color_palette[cell][1];
            colors[idx + 18] = color_palette[cell][2];
            colors[idx + 19] = color_palette[cell][3];

            colors[idx + 20] = color_palette[cell][0];
            colors[idx + 21] = color_palette[cell][1];
            colors[idx + 22] = color_palette[cell][2];
            colors[idx + 23] = color_palette[cell][3];
        }
    }

    colors
}

#[wasm_bindgen]
pub fn get_gl_context() -> Result<WebGl2RenderingContext, JsValue> {
    utils::set_panic_hook();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    Ok(canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?)
}

#[wasm_bindgen]
pub fn get_program(gl: WebGl2RenderingContext) -> Result<WebGlProgram, JsValue> {
    let vert_shader = compile_shader(&gl, WebGl2RenderingContext::VERTEX_SHADER, VERT_SHADER)?;
    let frag_shader = compile_shader(&gl, WebGl2RenderingContext::FRAGMENT_SHADER, FRAG_SHADER)?;

    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    Ok(program)
}

#[wasm_bindgen]
pub fn render(
    gl: WebGl2RenderingContext,
    program: WebGlProgram,
    num_cell_types: u8,
    universe_width: usize,
    max_time_steps: usize,
    rule_density: f32,
) -> Result<(), JsValue> {
    let position_attribute_location = gl.get_attrib_location(&program, "position");
    let cell_type_attribute_location = gl.get_attrib_location(&program, "cellType");

    let vert_buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
    let cell_type_buffer = gl.create_buffer().ok_or("Failed to create buffer")?;

    let vertices: Vec<f32> = cell_verts(universe_width, max_time_steps);
    let cell_types = ca(universe_width, max_time_steps, num_cell_types, rule_density);

    // ************ VERTS
    let vert_array = {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let vertices_location = vertices.as_ptr() as u32 / 4;
        js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + vertices.len() as u32)
    };

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vert_buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &vert_array,
        WebGl2RenderingContext::DYNAMIC_DRAW,
    );

    gl.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        2,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    gl.enable_vertex_attrib_array(position_attribute_location as u32);
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

    // ************ CELL TYPES
    let cell_type_array = {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let cell_type_location = cell_types.as_ptr() as u32 / 4;
        js_sys::Float32Array::new(&memory_buffer).subarray(
            cell_type_location,
            cell_type_location + cell_types.len() as u32,
        )
    };

    gl.bind_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(&cell_type_buffer),
    );
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &cell_type_array,
        WebGl2RenderingContext::DYNAMIC_DRAW,
    );
    gl.vertex_attrib_pointer_with_i32(
        cell_type_attribute_location as u32,
        4,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    gl.enable_vertex_attrib_array(cell_type_attribute_location as u32);
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

    let vert_count = (vertices.len() / 2) as i32;
    draw(&gl, vert_count);

    Ok(())
}

fn draw(gl: &WebGl2RenderingContext, vert_count: i32) {
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
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
