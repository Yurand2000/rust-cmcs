use full_palette::{GREEN_500, GREY};
use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use image::imageops::FilterType;

use crate::prelude::*;
use crate::cellular_automata::prelude::maze::*;

#[wasm_bindgen(js_name = CA_MAZE)]
pub struct Model {
    cached_states: Vec<Lattice>,
    maze_solver: MazeSolver,
    fix_point: bool,
}

#[wasm_bindgen(js_name = CA_MAZE_Params)]
#[derive(Default)]
pub struct Params {
    maze: &'static str,
}

#[wasm_bindgen(js_class = CA_MAZE)]
impl Model {
    pub fn build(params: Params) -> Self {
        Self {
            cached_states: Vec::new(),
            maze_solver: params.to_model(),
            fix_point: false
        }
    }

    pub fn last_step(&self) -> JsValue {
        JsValue::from_f64((self.cached_states.len() - 1) as f64)
    }

    pub fn draw(&mut self, canvas: HtmlCanvasElement, step: u32) -> Result<(), JsValue> {
        while (step as usize) >= self.cached_states.len() && !self.fix_point {
            let next_state = self.maze_solver.next().unwrap();
            
            if let Some(last_state) = self.cached_states.last() {
                if last_state == &next_state {
                    self.fix_point = true;
                } else {
                    self.cached_states.push(next_state);
                }
            } else {
                self.cached_states.push(next_state);
            }
        }   

        if (step as usize) >= self.cached_states.len() {
            draw_generic(Self::draw_function)(canvas, self.cached_states.last().unwrap())
        } else {
            draw_generic(Self::draw_function)(canvas, &self.cached_states[step as usize])
        }
    }

    fn draw_function(canvas: HtmlCanvasElement, lattice: &Lattice) -> MyDrawResult<()> {
        // generate a bitmap image
        let size = lattice.size();
        let mut image = image::DynamicImage::new(size.0, size.1, image::ColorType::Rgb8);
        let image_rgb = image.as_mut_rgb8().unwrap();

        image_rgb.fill(255);
        for (y, row) in image_rgb.rows_mut().enumerate() {
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

        // fill the canvas
        let (width, height) = (canvas.width(), canvas.height());
        let min_resolution = u32::min(width, height);
        let area = draw_prelude(canvas)?;
        area.fill(&WHITE)?;

        let area =
            if width < height {
                let diff_half = (height - width) as f32 / 2f32;
                area.margin(diff_half, diff_half, 0f32, 0f32)
            } else {
                let diff_half = (width - height) as f32 / 2f32;
                area.margin(0f32, 0f32, diff_half, diff_half)
            };

        let image = image.resize_exact(min_resolution, min_resolution, FilterType::Nearest);
        let image = BitMapElement::with_ref((0, 0), (min_resolution, min_resolution), image.as_bytes()).unwrap();

        area.draw(&image)?;

        Ok(())
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
                _ => "",
            };
        self
    }

    fn to_model(self) -> MazeSolver {
        MazeSolver::new(Lattice::from_string(&self.maze).unwrap())
    }
}