mod ca;
mod gl;
mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::{prelude::*, JsCast};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn duration(start: f64, end: f64) -> Duration {
    let amt = end - start;
    let secs = (amt as u64) / 1_000;
    let nanos = (((amt as u64) % 1_000) as u32) * 1_000_000;
    Duration::new(secs, nanos)
}

#[wasm_bindgen(js_name = generateCa)]
pub fn generate_ca(
    canvas: web_sys::HtmlCanvasElement,
    num_cell_types: u8,
    width: usize,
    rule_density: f32,
    animate: bool,
) -> Result<(), JsValue> {
    utils::set_panic_hook();

    let gl = gl::GL::new(canvas);
    let num_steps = (gl.get_aspect_ratio() * width as f64) as usize;
    let mut ca = ca::CA::new(width, num_steps, num_cell_types, rule_density);

    gl.attach_verts(&ca.cell_verts())?;
    let num_verts = ca.num_verts();

    if animate {
        let window = web_sys::window().unwrap();

        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();
        let mut start = p.now();

        let render_loop_closure = move || {
            ca.update_cell_colors();
            let (ct_ptr, ct_len) = ca.cell_types_ptr();
            gl.attach_cell_type_ptr(ct_ptr, ct_len).unwrap();
            gl.draw(num_verts);
            ca.next_generation();
            window
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
            // let _ = f.borrow_mut().take();
        };

        *g.borrow_mut() = Some(Closure::wrap(
            Box::new(render_loop_closure) as Box<dyn FnMut()>
        ));
        web_sys::window()
            .unwrap()
            .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    } else {
        (0..num_steps - 1).for_each(|_| ca.next_generation());
        ca.update_all_cell_colors();
        let (ct_ptr, ct_len) = ca.cell_types_ptr();
        gl.attach_cell_type_ptr(ct_ptr, ct_len)?;
        gl.draw(num_verts);
    }

    Ok(())
}
