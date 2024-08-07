use wasm_bindgen::prelude::*;

pub mod prelude {
    pub use super::{
        GraphType,
        MyDrawResult,
        StringError,
        MyDrawingArea,
        draw_prelude,
        draw_generic,
        Simulation
    };
}

pub mod phase_graph;
pub mod simulation_limits;

use simulation_limits::*;
use phase_graph::*;

pub enum GraphType {
    Function,
    PhaseGraph
}

impl GraphType {
    pub fn from_string(value: String) -> Option<Self> {
        match value.as_str() {
            "normal" => Some(GraphType::Function),
            "phase" => Some(GraphType::PhaseGraph),
            _ => None
        }
    }
}

// Type alias for the result of a drawing function.
pub type MyDrawResult<T> = Result<T, Box<dyn std::error::Error>>;

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
    where T: Iterator<Item = (X, Y)> + Clone, X: Clone, Y: Clone
{
    simulation: T
}

impl<T, X, Y> Simulation<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: Clone, Y: Clone
{
    pub fn new(simulation: T) -> Self {
        Self { simulation }
    }

    pub fn phase_graph_slope(self) -> Simulation<PhaseGraphSlope<T, X, Y>, Y, Y>
        where X: PartialOrd, Y: PartialOrd
    {
        Simulation { simulation: PhaseGraphSlope::wrap(self.simulation) }
    }

    pub fn phase_graph_lines(self) -> Simulation<PhaseGraphLines<T, X, Y>, Y, Y>
        where X: PartialOrd, Y: PartialOrd
    {
        Simulation { simulation: PhaseGraphLines::wrap(self.simulation) }
    }

    pub fn max_steps(self, steps: usize) -> Simulation<MaxStepSimulation<T, X, Y>, X, Y> {
        Simulation { simulation: MaxStepSimulation::wrap(self.simulation, steps) }
    }

    pub fn time_limit(self, max_time: X) -> Simulation<LimitedSimulation<T, X, Y>, X, Y>
        where X: PartialOrd
    {
        Simulation { simulation: LimitedSimulation::wrap(self.simulation, max_time) }
    }

    pub fn fix_point(self, max_time: X) -> Simulation<FixPointSimulation<T, X, Y>, X, Y>
    {
        Simulation { simulation: FixPointSimulation::wrap(self.simulation, max_time) }
    }

    pub fn map<F, X2, Y2>(self, fun: F) -> Simulation<std::iter::Map<T, F>, X2, Y2>
        where X2: PartialOrd + Clone, Y2: PartialOrd + Clone,
              F: FnMut((X, Y)) -> (X2, Y2) + Clone
    {
        Simulation { simulation: self.simulation.map(fun) }
    }
}

impl<T, X, Y> Iterator for Simulation<T, X, Y>
    where T: Iterator<Item = (X, Y)> + Clone, X: Clone, Y: Clone
{
    type Item = (X, Y);

    fn next(&mut self) -> Option<Self::Item> {
        self.simulation.next()
    }
}