use wasm_bindgen::prelude::*;

pub mod prelude {
    pub use super::{
        Point,
        MyDrawResult,
        StringError,
        MyDrawingArea,
        draw_prelude,
        draw_generic,
        Simulation,
        LimitedSimulation,
        PhaseGraphSlope,
        PhaseGraphLines,
        MaxStepSimulation
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

// simulation builder

#[derive(Clone)]
pub struct Simulation<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd + Clone, Y: PartialOrd + Clone
{
    simulation: T
}

impl<T, X, Y> Simulation<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd + Clone, Y: PartialOrd + Clone
{
    pub fn new(simulation: T) -> Self {
        Self { simulation }
    }

    pub fn phase_graph_slope(simulation: T) -> Simulation<PhaseGraphSlope<T, X, Y>, Y, Y> {
        Simulation { simulation: PhaseGraphSlope::wrap(simulation) }
    }

    pub fn phase_graph_lines(simulation: T) -> Simulation<PhaseGraphLines<T, X, Y>, Y, Y> {
        Simulation { simulation: PhaseGraphLines::wrap(simulation) }
    }

    pub fn max_steps(self, steps: usize) -> Simulation<MaxStepSimulation<T, X, Y>, X, Y> {
        Simulation { simulation: MaxStepSimulation::wrap(self.simulation, steps) }
    }

    pub fn time_limit(self, max_time: X) -> Simulation<LimitedSimulation<T, X, Y>, X, Y> {
        Simulation { simulation: LimitedSimulation::wrap(self.simulation, max_time) }
    }

    pub fn map<F, X2, Y2>(self, fun: F) -> Simulation<std::iter::Map<T, F>, X2, Y2>
        where X2: PartialOrd + Clone, Y2: PartialOrd + Clone,
              F: FnMut((X, Y)) -> (X2, Y2) + Clone
    {
        Simulation { simulation: self.simulation.map(fun) }
    }
}

impl<T, X, Y> Iterator for Simulation<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd + Clone, Y: PartialOrd + Clone
{
    type Item = (X, Y);

    fn next(&mut self) -> Option<Self::Item> {
        self.simulation.next()
    }
}

// simulation helper structs
#[derive(Clone)]
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

#[derive(Clone)]
pub struct MaxStepSimulation<T, X, Y>
    where T: Iterator<Item = (X, Y)>
{
    iterator: T,
    last_step: usize,
    max_steps: usize
}

impl<T, X, Y> MaxStepSimulation<T, X, Y>
    where T: Iterator<Item = (X, Y)>
{
    pub fn wrap(simulation: T, max_steps: usize) -> Self {
        Self { iterator: simulation, last_step: 0, max_steps }
    }
}

impl<T, X, Y> Iterator for MaxStepSimulation<T, X, Y>
    where T: Iterator<Item = (X, Y)>
{
    type Item = (X, Y);

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_step < self.max_steps {
            self.last_step += 1;
            self.iterator.next()
        } else {
            None
        }
    }
}

// Phase Graph Utils
#[derive(Clone)]
pub struct PhaseGraphSlope<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd
{
    iterator_pre: T,
    iterator_post: T,
}

impl<T, X, Y> PhaseGraphSlope<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: PartialOrd, Y: PartialOrd
{
    pub fn wrap(iterator: T) -> Self {
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

#[derive(Clone)]
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
    pub fn wrap(iterator: T) -> Self {
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