use full_palette::{GREEN_500, GREY};
use image::RgbImage;
use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;
use crate::cellular_automata::prelude::maze::*;

#[wasm_bindgen(js_name = CA_MAZE)]
pub struct Model {
    states: Vec<image::RgbImage>,
    size: (u32, u32)
}

#[wasm_bindgen(js_name = CA_MAZE_Params)]
#[derive(Default)]
pub struct Params {
    maze: &'static str,
}

#[wasm_bindgen(js_class = CA_MAZE)]
impl Model {
    pub fn build(params: Params) -> Self {
        let solver = params.to_model();
        let mut last_state = None;
        let last_state = &mut last_state;
        let states: Vec<_> = 
            solver.take_while(move |curr_state| {
                match last_state {
                    Some(last_state) if last_state == curr_state => {
                        false
                    },
                    _ => {
                        *last_state = Some(curr_state.clone());
                        true
                    },
                }
            })
            .map(|lattice| Self::lattice_to_image(&lattice))
            .collect();

        let size = (states[0].width(), states[0].height());
        Self { states, size }
    }

    pub fn max_step(&self) -> JsValue {
        JsValue::from_f64((self.states.len() - 1) as f64)
    }

    pub fn draw(&mut self, canvas: HtmlCanvasElement, step: u32) -> Result<(), JsValue> {
        canvas.set_width(self.size.0);
        canvas.set_height(self.size.1);

        let step = usize::min(self.states.len() - 1, step as usize);
        draw_generic(Self::draw_function)(canvas, &self.states[step])
    }

    fn draw_function(canvas: HtmlCanvasElement, image: &RgbImage) -> MyDrawResult<()> {
        let image = BitMapElement::with_owned_buffer(
            (0, 0), (image.width(), image.height()),
            image.pixels().flat_map(|elem| elem.0.clone().into_iter()).collect()
        ).unwrap();

        draw_prelude(canvas)?.draw(&image)?;

        Ok(())
    }

    fn lattice_to_image(lattice: &Lattice) -> image::RgbImage {
        let size = lattice.size();
        let mut image = image::RgbImage::new(size.0, size.1);

        image.fill(255);
        for (y, row) in image.rows_mut().enumerate() {
            for (x, cell) in row.enumerate() {
                let color =
                    match lattice.get(x as u32, y as u32).unwrap() {
                        Cell::Wall => &GREY,
                        Cell::NotVisited => &WHITE,
                        Cell::Visited { len } if *len == 0 => &RED,
                        Cell::Visited { .. } => &BLUE,
                        Cell::End => &GREEN_500,
                        Cell::Backtrace { .. } => &GREEN,
                    };
                
                cell.0 = [color.0, color.1, color.2];
            }
        }

        image
    }
}

#[wasm_bindgen(js_class = CA_MAZE_Params)]
impl Params {
    pub fn builder() -> Self {
        Self { ..Default::default()}
    }

    pub fn maze(mut self, str: &str) -> Self {
        self.maze =
            match str {
                "maze0" => mazes::MAZE0,
                "maze1" => mazes::MAZE1,
                _ => "",
            };
        self
    }

    fn to_model(self) -> MazeSolver {
        MazeSolver::new(Lattice::from_string(&self.maze).unwrap())
    }
}