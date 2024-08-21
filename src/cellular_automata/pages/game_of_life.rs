use image::RgbImage;
use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::prelude::*;
use crate::cellular_automata::prelude::game_of_life::*;

#[wasm_bindgen(js_name = CA_GOL)]
pub struct Model {
    states: Vec<image::RgbImage>,
    size: (u32, u32)
}

#[wasm_bindgen(js_name = CA_GOL_Params)]
#[derive(Default)]
pub struct Params {
    state: &'static str,
    max_time: u32,
    fixed_boundary: bool,
}

#[wasm_bindgen(js_class = CA_GOL)]
impl Model {
    pub fn build(params: Params) -> Result<Model, JsValue> {
        let max_time = params.max_time;
        let solver: Box<dyn Iterator<Item = Result<State, String>>> = 
            if params.fixed_boundary {
                Box::new(params.to_model_fixed_boundary()?)
            } else {
                Box::new(params.to_model_periodic_boundary()?)
            };

        let mut last_state = None;
        let mut error = false;
        let last_state = &mut last_state;
        let error = &mut error;
        let states: Vec<_> = solver
            .take(max_time as usize)
            .take_while(move |curr_state| {
                if *error {
                    false
                } else {
                    match curr_state {
                        Ok(curr_state) => {
                            match last_state {
                                Some(last_state) if last_state == curr_state => {
                                    false
                                },
                                _ => {
                                    *last_state = Some(curr_state.clone());
                                    true
                                },
                            }
                        },
                        Err(_) => {
                            *error = true;
                            true
                        },
                    }
                }
            })
            .try_fold::<_, _, Result<_, String>>(Vec::new(), |mut acc, curr_state| {
                let curr_state = curr_state?;

                acc.push(Self::state_to_image(&curr_state));
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

    fn state_to_image(state: &State) -> image::RgbImage {
        let size = state.size();
        let mut image = image::RgbImage::new(size.0, size.1);

        image.fill(255);
        for (y, row) in image.rows_mut().enumerate() {
            for (x, cell) in row.enumerate() {
                let color =
                    match state.get(x as u32, y as u32).unwrap() {
                        true => &BLUE,
                        false => &WHITE,
                    };
                
                cell.0 = [color.0, color.1, color.2];
            }
        }

        image
    }
}

#[wasm_bindgen(js_class = CA_GOL_Params)]
impl Params {
    pub fn builder() -> Self {
        Self { ..Default::default()}
    }

    pub fn max_time(mut self, max_time: u32) -> Self {
        self.max_time = max_time;
        self
    }

    pub fn state(mut self, str: &str) -> Self {
        (self.state, self.fixed_boundary) =
            match str {
                "still" => (states::STILL, true),
                "oscillators" => (states::OSCILLATORS, true),
                "pulsar" => (states::PULSAR, true),
                "glider" => (states::GLIDER, false),
                "lwss" => (states::LWSS, false),
                "diehard" => (states::DIEHARD, true),
                "glider_gun" => (states::GLIDER_GUN, false),
                "and_gate" => (states::AND_GATE, false),
                _ => ("", true),
            };
        self
    }

    fn to_model_fixed_boundary(self) -> Result<GameOfLife<BoundaryFixed>, JsValue> {
        GameOfLife::from_string(&self.state)
            .ok_or(JsValue::from_str("State string parse error"))
    }

    fn to_model_periodic_boundary(self) -> Result<GameOfLife<BoundaryPeriodic>, JsValue> {
        GameOfLife::from_string(&self.state)
            .ok_or(JsValue::from_str("State string parse error"))
    }
}