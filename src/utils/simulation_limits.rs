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

#[derive(Clone)]
pub struct FixPointSimulation<T, X, Y>
    where T: Iterator<Item = (X, Y)>, X: Clone, Y: Clone
{
    iterator: T,
    last_state: Option<Y>,
    last_time: X,
}

impl<T, X, Y> FixPointSimulation<T, X, Y>
    where T: Iterator<Item = (X, Y)>, X: Clone, Y: Clone
{
    pub fn wrap(simulation: T, last_time: X) -> Self {
        Self { iterator: simulation, last_state: None , last_time}
    }
}

impl<T, X, Y> Iterator for FixPointSimulation<T, X, Y>
    where T: Iterator<Item = (X, Y)>, X: Clone, Y: Clone
{
    type Item = (X, Y);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some((next_time, next_state)) => {
                self.last_state = Some(next_state.clone());
                Some((next_time, next_state))
            },
            None => {
                self.last_state.clone()
                    .map(|state| (self.last_time.clone(), state))
            },
        }
    }
}