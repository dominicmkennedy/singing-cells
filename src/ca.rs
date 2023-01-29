use itertools::{iproduct, Itertools};
use std::collections::{BTreeSet, VecDeque};
use wasm_bindgen::prelude::*;
// use js_sys::Math::random;
// including this makes js not run

// insane workaround
// probably due to something not linking properly
// hope to fix in the future
#[wasm_bindgen(inline_js = "export function random() { return Math.random(); }")]
extern "C" {
    fn random() -> f64;
}

// #[wasm_bindgen]
pub struct CA {
    size: usize,
    pub num_steps: usize,
    rule_table: Vec<u8>,
    universe: Vec<Vec<u8>>,
    cell_types: VecDeque<u32>,
    // num_verts: i32,
}

// #[wasm_bindgen]
impl CA {
    // #[wasm_bindgen(constructor)]
    pub fn new(size: usize, num_steps: usize, cell_types: u8, rule_density: f32) -> CA {
        CA {
            size,
            num_steps,
            rule_table: Self::gen_rule_table(cell_types, rule_density),
            universe: Self::gen_init_universe(size, num_steps, cell_types),
            cell_types: VecDeque::from(vec![0; size * num_steps * 6]),
            // num_verts: (num_steps * size * 6) as i32,
        }
    }

    pub fn cell_verts(&self) -> Vec<f32> {
        let cell_width_px: f32 = 2.0 / (self.size as f32);
        let cell_height_px: f32 = 2.0 / (self.num_steps as f32);

        iproduct!((0..self.num_steps).rev(), (0..self.size), (0..12))
        .map(|(i, j, k)| match k {
            0 | 2 | 6  => (j as f32)       * cell_width_px,
            4 | 8 | 10 => ((j + 1) as f32) * cell_width_px,
            3 | 5 | 9  => (i as f32)       * cell_height_px,
            1 | 7 | 11 => ((i + 1) as f32) * cell_height_px,
            _ => 0.0,
        } - 1.0)
        .collect()
    }

    fn gen_init_universe(size: usize, num_steps: usize, num_cell_types: u8) -> Vec<Vec<u8>> {
        let mut u = vec![vec![0; size]; num_steps];
        u[num_steps - 1] = Self::gen_init_cells(num_cell_types, size);

        u
    }

    fn gen_rule_table(n: u8, rule_density: f32) -> Vec<u8> {
        let width = ((n as usize - 1) * 3) + 1;
        let mut table: Vec<u8> = (0..width).map(|_| Self::gen_range(1, n)).collect();

        let mut to_remove: BTreeSet<usize> = BTreeSet::new();
        while to_remove.len() != ((width as f32) * (1.0 - rule_density)) as usize {
            to_remove.insert(Self::gen_range(0, width as u8) as usize);
        }

        to_remove.iter().for_each(|&x| table[x] = 0);

        table
    }

    //TODO reverify that this indeed works
    //TODO write a one shot version of this function
    //to speed up static CA generation 
    //also then remove pub from num_steps
    pub fn next_generation(&mut self) {
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
        (0..width).map(|_| Self::gen_range(0, n)).collect()
    }

    // raw loop is bad but it's the fastest method I've tried
    pub fn update_all_cell_colors(&mut self) -> (*const u32, usize) {
        for (i, cell) in self
            .universe
            .iter()
            .flatten()
            .flat_map(|x| std::iter::repeat(*x as u32).take(6))
            .enumerate()
        {
            self.cell_types[i] = cell as u32;
        }
        (
            self.cell_types.as_slices().0.as_ptr(),
            self.cell_types.len(),
        )
    }

    pub fn update_cell_colors(&mut self) -> (*const u32, usize) {
        self.cell_types
            .iter_mut()
            .zip(iproduct!(self.universe.last().unwrap().iter(), (0..6)))
            .map(|(ct, (u, _))| *ct = *u as u32)
            .count();
        self.cell_types.rotate_left(self.size * 6);
        self.cell_types.make_contiguous();
        (
            self.cell_types.as_slices().0.as_ptr(),
            self.cell_types.len(),
        )

        // self.cell_types.extend(
        //     iproduct!(self.universe.last().unwrap().iter(), (0..6)).map(|(u, _)| *u as u32),
        // );
        // self.cell_types.drain(0..self.size * 6);
    }

    fn gen_range(low: u8, high: u8) -> u8 {
        let range = (high - low) as f64;
        ((random() * range) as u8) + low
    }

    pub fn num_verts(&self) -> i32 {
        (self.num_steps * self.size * 6) as i32
    }
}
