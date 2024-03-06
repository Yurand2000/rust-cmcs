use wasm_bindgen::prelude::*;

pub mod prelude {
    pub use super::{
        Point,
        MyDrawResult,
        StringError,
        MyDrawingArea,
        draw_prelude,
        draw_generic,
        LimitedSimulation,
        PhaseGraphSlope,
        PhaseGraphLines,
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
pub struct LimitedSimulation<T, X, Y>
    where T: Iterator<Item = (X, Y)>, X: PartialOrd
{
    iterator: T,
    max_x: X,
}

impl<T, X, Y> LimitedSimulation<T, X, Y>
    where T: Iterator<Item = (X, Y)>, X: PartialOrd
{
    pub fn wrap(simulator: T, max_time: X) -> Self {
        Self { iterator: simulator, max_x: max_time }
    }
}

impl<T, X, Y> Iterator for LimitedSimulation<T, X, Y>
    where T: Iterator<Item = (X, Y)>, X: PartialOrd
{
    type Item = (X, Y);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
            .filter(|(x, _)| x <= &self.max_x)
    }
}

// Phase Graph Utils
pub struct PhaseGraphSlope<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd
{
    iterator_pre: T,
    iterator_post: T,
}

impl<T, X, Y> PhaseGraphSlope<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd
{
    pub fn new(iterator: T) -> Self {
        let mut iterator_post = iterator.clone();
        iterator_post.next();

        Self {
            iterator_pre: iterator,
            iterator_post,
        }
    }
}

impl<T, X, Y> Iterator for PhaseGraphSlope<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd
{
    type Item = (Y, Y);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator_pre.next()
            .zip(self.iterator_post.next())
            .map(|((_, y0),(_, y1))| (y0, y1))
    }
}

pub struct PhaseGraphLines<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd + Clone
{
    iterator_pre: T,
    iterator_post: T,
    pre_last_moved: bool,
    last_coord: (Y, Y)
}

impl<T, X, Y> PhaseGraphLines<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd + Clone
{
    pub fn new(iterator: T) -> Self {
        let mut iterator_post = iterator.clone();
        let Some((_, y)) = iterator_post.next() else { panic!() };
        
        Self {
            iterator_pre: iterator,
            iterator_post,
            pre_last_moved: false,
            last_coord: (y.clone(), y),
        }
    }
}

impl<T, X, Y> Iterator for PhaseGraphLines<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd + Clone
{
    type Item = (Y, Y);

    fn next(&mut self) -> Option<Self::Item> {
        let (last_pre, last_post) = self.last_coord.clone();
        let pre_last_moved = self.pre_last_moved;
        self.pre_last_moved = !self.pre_last_moved;
        if pre_last_moved {
            let (_, post) = self.iterator_post.next()?;
            self.last_coord = (last_pre, post);
            Some(self.last_coord.clone())
        } else {
            let (_, pre) = self.iterator_pre.next()?;
            self.last_coord = (pre, last_post);
            Some(self.last_coord.clone())
        }
    }
}