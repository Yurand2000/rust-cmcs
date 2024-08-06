mod pages {
    mod linear_birth_model;
    mod radioactive_decay;
    mod logistic_equation;
    mod male_female_fish_population;
    mod lotka_volterra;
    mod sir_model;
    mod sir_model_birth_deaths;
    mod sir_model_vaccination;
}

mod prelude {
    pub use super::linear_birth_model::*;
    pub use super::logistic_equation::*;
    pub use super::male_female_fish_population::*;
    pub use super::lotka_volterra::*;
    pub use super::sir_model::*;
    pub use super::ODESolver;
}

mod linear_birth_model;
mod logistic_equation;
mod male_female_fish_population;
mod lotka_volterra;
mod sir_model;

#[derive(Clone, Copy)]
#[derive(Default)]
pub enum ODESolver {
    DOP853,
    DOPRI5,
    #[default]
    RK4
}

impl ODESolver {
    pub fn from_string(str: String) -> Option<Self> {
        match str.as_str() {
            "dop853" => Some(Self::DOP853),
            "dopri5" => Some(Self::DOPRI5),
            "rk4" => Some(Self::RK4),
            _ => None,
        }
    }
}