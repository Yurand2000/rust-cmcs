use std::marker::PhantomData;

use wasm_bindgen::prelude::*;
use crate::prelude::*;

pub mod prelude {
    pub use super::{
        Point,
        MyDrawResult,
        StringError,
        MyDrawingArea,
        draw_prelude,
        draw_generic,
        LimitedSimulation,
    };
}

#[global_allocator]
// WASM specific allocator
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Type alias for the result of a drawing function.
pub type MyDrawResult<T> = Result<T, Box<dyn std::error::Error>>;

#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug)]
pub struct StringError {
    pub error: String,
}

impl StringError {
    pub fn new(error: String) -> Self {
        Self { error: error }
    }
}

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

impl std::error::Error for StringError { }


// helper functions for canvas drawing
pub type MyDrawingArea = plotters::drawing::DrawingArea<plotters_canvas::CanvasBackend, plotters::coord::Shift>;

pub fn draw_prelude(canvas: web_sys::HtmlCanvasElement) -> MyDrawResult<MyDrawingArea> {
    let Some(area) = plotters_canvas::CanvasBackend::with_canvas_object(canvas) else {
        return Err(Box::new(StringError::new(format!(""))));
    };

    Ok(plotters::drawing::IntoDrawingArea::into_drawing_area(area))
}

pub fn draw_generic<F, T, Params>(fun: F) -> impl Fn(web_sys::HtmlCanvasElement, Params) -> Result<T, JsValue>
    where F: Fn(web_sys::HtmlCanvasElement, Params) -> MyDrawResult<T>
{
    move |canvas, params| {
        fun(canvas, params).map_err(|err| err.to_string().into())
    }
}

// simulation helper structs
pub struct LimitedSimulation<T, X, Y, XIT, YIT>
    where T: Iterator<Item = (XIT, YIT)>, XIT: PartialOrd + Into<X>, YIT: Into<Y>
{
    iterator: T,
    max_time: XIT,
    _phantom: PhantomData<(X, Y)>
}

impl<T, X, Y, XIT, YIT> LimitedSimulation<T, X, Y, XIT, YIT>
    where T: Iterator<Item = (XIT, YIT)>, XIT: PartialOrd + Into<X>, YIT: Into<Y>
{
    pub fn wrap(simulator: T, max_time: XIT) -> Self {
        Self { iterator: simulator, max_time, _phantom: PhantomData }
    }
}

impl<T, X, Y, XIT, YIT> Iterator for LimitedSimulation<T, X, Y, XIT, YIT>
    where T: Iterator<Item = (XIT, YIT)>, XIT: PartialOrd + Into<X>, YIT: Into<Y>
{
    type Item = (X, Y);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
            .filter(|(time, _)| time <= &self.max_time)
            .map(|(x,y)| (x.into(), y.into()))
    }
}