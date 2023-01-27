mod ca;
mod gl;
mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn render(
    canvas: web_sys::HtmlCanvasElement,
    num_cell_types: u8,
    width: usize,
    rule_density: f32,
) -> Result<(), JsValue> {
    utils::set_panic_hook();

    let gl = gl::GL::new(canvas);
    let mut ca = ca::CA::new(
        width,
        (gl.get_aspect_ratio() * width as f64) as usize,
        num_cell_types,
        rule_density,
    );

    gl.attach_verts(&ca.cell_verts())?;
    let (ct_ptr, ct_len) = ca.cell_types_ptr();
    let num_verts = ca.num_verts();

    // (0..max_time_steps - 1).for_each(|_| c.next_generation());
    // c.update_cell_colors();
    // x.attach_cell_type_ptr(c.cell_types.as_ptr(), c.cell_types.len())?;

    let window = web_sys::window().unwrap();
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let render_loop_closure = move || {
        ca.update_cell_colors();
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

    Ok(())
}
