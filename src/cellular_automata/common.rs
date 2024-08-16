#[derive(Clone, Copy)]
#[derive(Default)]
pub enum StartingState {
    #[default]
    SingleCell,
    Random,
    Full,
    Empty,
}

impl StartingState {
    pub fn from_str(str: &str) -> Option<Self> {
        match str {
            "single" => Some(Self::SingleCell),
            "random" => Some(Self::Random),
            "full" => Some(Self::Full),
            "empty" => Some(Self::Empty),
            _ => None,
        }
    }
}