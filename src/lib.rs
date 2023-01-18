const UNIVERSE_WIDTH: usize = 16;
const MAX_TIME_STEPS: usize = 32;
const NUM_CELL_TYPES: u8 = 10;
const COLORS: [[f32; 4]; 10] = [
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

mod utils;

use js_sys::{Math::random, WebAssembly};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn gen_init_cells(n: u8, width: usize) -> Vec<u8> {
    vec![0; width].iter().map(|&_| gen_range(0, n)).collect()
}

fn gen_rule_table(n: u8, width: usize) -> Vec<u8> {
    let mut table: Vec<u8> = vec![0; width].iter().map(|&_| gen_range(1, n)).collect();
    table[0] = 0;
    table
}

// generate a random number within a range
fn gen_range(low: u8, high: u8) -> u8 {
    let range = (high - low) as f64;
    ((random() * range) as u8) + low
}

// next step function
// consider using a more functional approch for the new state
fn next_generation(rule_table: &Vec<u8>, old_state: &Vec<u8>, new_state: &mut Vec<u8>) {
    let len = new_state.len();

    for i in 0..len {
        let tot =
            usize::from(old_state[i] + old_state[(i + 1) % len] + old_state[((i + len) - 1) % len]);
        new_state[i] = rule_table[tot];
    }
}

fn ca(universe_width: usize, max_time_steps: usize, num_cell_types: u8) -> Vec<f32> {
    let rule_table_width: usize = (((num_cell_types as usize) - 1) * 3) + 1;

    // rule table
    let rule_table = gen_rule_table(num_cell_types, rule_table_width);

    // initial cells
    let init_cells = gen_init_cells(num_cell_types, universe_width);

    // blank cell board
    let mut cell_board = vec![vec![0; universe_width]; max_time_steps];

    // advance time steps
    let mut prev_row = &init_cells;
    for next_row in cell_board.iter_mut() {
        next_generation(&rule_table, prev_row, next_row);
        prev_row = next_row;
    }

    // return results as verts and colors
    cell_colors(&cell_board)
}

// rewrite better, this is hidious
fn cell_verts(num_verts: usize) -> Vec<f32> {
    let mut vertices = vec![0.0; num_verts];

    const CELL_WIDTH: f32 = 2.0 / (UNIVERSE_WIDTH as f32);
    const CELL_HEIGHT: f32 = 2.0 / (MAX_TIME_STEPS as f32);

    for i in 0..MAX_TIME_STEPS {
        for j in 0..UNIVERSE_WIDTH {
            let idx = (i + (j * MAX_TIME_STEPS)) * 12;
            let x0 = ((j as f32) * CELL_WIDTH) - 1.0;
            let x1 = (((j + 1) as f32) * CELL_WIDTH) - 1.0;

            let y0 = (((MAX_TIME_STEPS - i) as f32) * CELL_HEIGHT) - 1.0;
            let y1 = ((((MAX_TIME_STEPS - i) - 1) as f32) * CELL_HEIGHT) - 1.0;
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
fn cell_colors(cell_board: &Vec<Vec<u8>>) -> Vec<f32> {
    let total_cells: usize = cell_board.len() * cell_board[0].len();
    let num_verts: usize = total_cells * 2 * 3 * 2;
    let num_colors: usize = num_verts * 2;

    let mut colors = vec![0.0; num_colors];

    for i in 0..cell_board.len() {
        for j in 0..cell_board[i].len() {
            let idx = (i + (j * MAX_TIME_STEPS)) * 24;
            let cell = cell_board[i][j] as usize;

            colors[idx + 0] = COLORS[cell][0];
            colors[idx + 1] = COLORS[cell][1];
            colors[idx + 2] = COLORS[cell][2];
            colors[idx + 3] = COLORS[cell][3];

            colors[idx + 4] = COLORS[cell][0];
            colors[idx + 5] = COLORS[cell][1];
            colors[idx + 6] = COLORS[cell][2];
            colors[idx + 7] = COLORS[cell][3];

            colors[idx + 8] = COLORS[cell][0];
            colors[idx + 9] = COLORS[cell][1];
            colors[idx + 10] = COLORS[cell][2];
            colors[idx + 11] = COLORS[cell][3];

            colors[idx + 12] = COLORS[cell][0];
            colors[idx + 13] = COLORS[cell][1];
            colors[idx + 14] = COLORS[cell][2];
            colors[idx + 15] = COLORS[cell][3];

            colors[idx + 16] = COLORS[cell][0];
            colors[idx + 17] = COLORS[cell][1];
            colors[idx + 18] = COLORS[cell][2];
            colors[idx + 19] = COLORS[cell][3];

            colors[idx + 20] = COLORS[cell][0];
            colors[idx + 21] = COLORS[cell][1];
            colors[idx + 22] = COLORS[cell][2];
            colors[idx + 23] = COLORS[cell][3];
        }
    }

    colors
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
 
        in vec4 position;
        in vec4 cellType;

        out vec4 fragColor;

        void main() {
            fragColor = cellType;
            gl_Position = position;
        }
        "##,
    )?;

    let frag_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
    
        precision highp float;

        in vec4 fragColor;

        out vec4 outColor;
        
        void main() {
            outColor = fragColor;
        }
        "##,
    )?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    let position_attribute_location = gl.get_attrib_location(&program, "position");
    let cell_type_attribute_location = gl.get_attrib_location(&program, "cellType");

    let vert_buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
    let cell_type_buffer = gl.create_buffer().ok_or("Failed to create buffer")?;

    let total_cells: usize = UNIVERSE_WIDTH * MAX_TIME_STEPS;
    let num_verts: usize = total_cells * 2 * 3 * 2;

    let vertices: Vec<f32> = cell_verts(num_verts);
    let cell_types = ca(UNIVERSE_WIDTH, MAX_TIME_STEPS, NUM_CELL_TYPES);

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

    // may need this later
    // let vao = gl
    //     .create_vertex_array()
    //     .ok_or("Could not create vertex array object")?;
    // gl.bind_vertex_array(Some(&vao));
    // gl.bind_vertex_array(Some(&vao));

    let vert_count = (vertices.len() / 2) as i32;
    draw(&gl, vert_count);

    Ok(())
}

fn draw(gl: &WebGl2RenderingContext, vert_count: i32) {
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

pub fn compile_shader(
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

pub fn link_program(
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
