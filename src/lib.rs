mod ca;
mod gl;
mod utils;
mod audio;

use std::time::Duration;
use wasm_bindgen::prelude::*;

// for debugging
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// for debugging
fn duration(start: f64, end: f64) -> Duration {
    let amt = end - start;
    let secs = (amt as u64) / 1_000;
    let nanos = (((amt as u64) % 1_000) as u32) * 1_000_000;
    Duration::new(secs, nanos)
}

#[wasm_bindgen]
pub struct GLandCA {
    gl: gl::GL,
    ca: ca::CA,
}

#[wasm_bindgen]
impl GLandCA {
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: web_sys::HtmlCanvasElement,
        num_cell_types: u8,
        width: usize,
        rule_density: f32,
    ) -> GLandCA {
        let gl = gl::GL::new(canvas);
        let num_steps = (gl.get_aspect_ratio() * width as f64) as usize;
        let ca = ca::CA::new(width, num_steps, num_cell_types, rule_density);
        gl.attach_verts(&ca.cell_verts()).unwrap();
        GLandCA { gl, ca }
    }

    pub fn draw_animation_frame(&mut self) {
        let (ct_ptr, ct_len) = self.ca.update_cell_colors();
        self.gl.attach_cell_type_ptr(ct_ptr, ct_len).unwrap();
        self.gl.draw(self.ca.num_verts());
        self.ca.next_generation();
    }

    pub fn draw_entire_ca(&mut self) {
        (0..self.ca.num_steps - 1).for_each(|_| self.ca.next_generation());
        let (ct_ptr, ct_len) = self.ca.update_all_cell_colors();
        self.gl.attach_cell_type_ptr(ct_ptr, ct_len).unwrap();
        self.gl.draw(self.ca.num_verts());
    }
}
