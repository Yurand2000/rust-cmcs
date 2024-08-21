use full_palette::GREEN_500;
use image::RgbImage;
use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;
use crate::cellular_automata::prelude::forest_fire::*;

#[wasm_bindgen(js_name = CA_FF)]
pub struct Model {
    states: Vec<image::RgbImage>,
    size: (u32, u32)
}

#[wasm_bindgen(js_name = CA_FF_Params)]
#[derive(Default)]
pub struct Params {
    size: u32,
    lightning_probability: f64,
    growing_probability: f64,
    max_time: u32,
    seed: u64,
}

#[wasm_bindgen(js_class = CA_FF)]
impl Model {
    pub fn build(params: Params) -> Result<Model, JsValue> {
        let max_time = params.max_time;
        let solver = params.to_model()?;
        let states: Vec<_> = solver
            .take(max_time as usize)
            .try_fold::<_, _, Result<_, String>>(Vec::new(), |mut acc, curr_state| {
                let curr_state = curr_state?;

                acc.push(Self::maze_to_image(&curr_state));
                Ok(acc)
            })
            .map_err(|err| JsValue::from_str(&err))?;

        let size = (states[0].width(), states[0].height());
        Ok(Self { states, size })
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

    fn maze_to_image(forest: &ForestLattice) -> image::RgbImage {
        let size = forest.size();
        let mut image = image::RgbImage::new(size.0, size.1);

        image.fill(255);
        for (y, row) in image.rows_mut().enumerate() {
            for (x, cell) in row.enumerate() {
                let color =
                    match forest.get(x as u32, y as u32).unwrap() {
                        Cell::GreenTree => &GREEN_500,
                        Cell::BurningTree => &YELLOW,
                        Cell::Empty => &BLACK,
                    };
                
                cell.0 = [color.0, color.1, color.2];
            }
        }

        image
    }
}

#[wasm_bindgen(js_class = CA_FF_Params)]
impl Params {
    pub fn builder() -> Self {
        Self { ..Default::default()}
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    pub fn lightning_probability(mut self, p: f64) -> Self {
        self.lightning_probability = p;
        self
    }

    pub fn growing_probability(mut self, p: f64) -> Self {
        self.growing_probability = p;
        self
    }

    pub fn max_time(mut self, max_time: u32) -> Self {
        self.max_time = max_time;
        self
    }

    pub fn simulation_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    fn to_model(self) -> Result<ForestFireModel, JsValue> {
        ForestFireModel::new(
            self.size,
            self.seed,
            self.lightning_probability,
            self.growing_probability
        ).map_err(|err| JsValue::from_str(&err))
    }
}