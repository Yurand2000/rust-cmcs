use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

pub mod prelude {
    pub use super::utils::prelude::*;
}

pub mod discrete_dynamical_systems;

pub mod utils;

mod func_plot;
mod mandelbrot;
mod plot3d;

use prelude::*;

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}


#[wasm_bindgen]
impl Chart {
    /// Draw provided power function on the canvas element using it's id.
    /// Return `Chart` struct suitable for coordinate conversion.
    pub fn power(canvas_id: &str, power: i32) -> Result<Chart, JsValue> {
        let map_coord = func_plot::draw(canvas_id, power).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    /// Draw Mandelbrot set on the provided canvas element.
    /// Return `Chart` struct suitable for coordinate conversion.
    pub fn mandelbrot(canvas: HtmlCanvasElement) -> Result<Chart, JsValue> {
        let map_coord = mandelbrot::draw(canvas).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(map_coord),
        })
    }

    pub fn plot3d(canvas: HtmlCanvasElement, pitch: f64, yaw: f64) -> Result<(), JsValue> {
        plot3d::draw(canvas, pitch, yaw).map_err(|err| err.to_string())?;
        Ok(())
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}
