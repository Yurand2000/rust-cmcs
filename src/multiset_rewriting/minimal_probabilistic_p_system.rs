use std::{collections::HashMap, rc::Rc};

use rand::SeedableRng;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Object(pub String);

impl Object {
    pub fn from_str(str: &'static str) -> Self {
        Self(str.to_owned())
    }
}

#[derive(Clone)]
pub struct MultiSet(HashMap<Object, u32>);

impl MultiSet {
    pub fn empty() -> Self {
        Self(HashMap::with_capacity(0))
    }

    pub fn from_array<const C: usize>(vec: [(Object, u32); C]) -> Self {
        Self(vec.into_iter().collect())
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.0.iter()
            .all(|(object, quantity)| {
                other.0.get(object)
                    .map_or(false, |other_quantity| quantity <= other_quantity)
            })
    }

    fn join(mut self, other: Self) -> Self {
        for (object, other_quantity) in other.0.into_iter() {
            self.0.entry(object)
                .and_modify(|quantity| *quantity += other_quantity)
                .or_insert(other_quantity);
        }

        self
    }

    pub fn get(&self, obj: &Object) -> u32 {
        self.0.get(obj).cloned().unwrap_or(0)
    }
}

#[derive(Clone)]
pub struct EvolutionRule {
    reactants: MultiSet,
    products: MultiSet,
    promoters: MultiSet,
    rate_function: Rc<dyn Fn(&MultiSet) -> f32>,
}

impl EvolutionRule {
    pub fn new<const C1: usize, const C2: usize, const C3: usize, F: Fn(&MultiSet) -> f32 + 'static> (
        reactants: [(Object, u32); C1], products: [(Object, u32); C2], promoters: [(Object, u32); C3],
        rate_function: F,
    ) -> Self {
        Self {
            reactants: MultiSet::from_array(reactants),
            products: MultiSet::from_array(products),
            promoters: MultiSet::from_array(promoters),
            rate_function: Rc::new(rate_function),
        }
    }

    fn apply(&self, reagents: &mut MultiSet, products: &mut MultiSet) {
        for (reagent, quantity) in self.reactants.0.iter() {
            *reagents.0.get_mut(reagent).unwrap() -= quantity;
        }
        for (product, quantity) in self.products.0.iter() {
            products.0.entry(product.clone())
                .and_modify(|q| *q += quantity)
                .or_insert(*quantity);
        }
    }
}

#[derive(Clone)]
pub struct MinimalProbabilisticPSystem {
    initial_state: MultiSet,
    rules: Vec<EvolutionRule>,
    rng: rand::rngs::SmallRng,

    last_state: Option<(f32, MultiSet)>
}

impl MinimalProbabilisticPSystem {
    pub fn new<const C: usize>(initial_state: [(Object, u32); C], rules: Vec<EvolutionRule>, seed: u64) -> Self {
        Self {
            initial_state: MultiSet::from_array(initial_state),
            rules,
            rng: rand::rngs::SmallRng::seed_from_u64(seed),
            last_state: None,
        }
    }

    fn step<R: rand::Rng>(state: MultiSet, rules: &Vec<EvolutionRule>, rng: &mut R) -> MultiSet {
        let mut reagents = state.clone();
        let mut products = MultiSet::empty();

        // filter the rules that cannot be applied
        let mut applicable: Vec<_> = rules.iter()
            .filter(|rule| rule.promoters.is_subset(&state))
            .map(|rule| (rule, (rule.rate_function)(&state)))
            .collect();

        loop {
            applicable = applicable.into_iter()
                .filter(|(rule, _)| rule.reactants.is_subset(&reagents)).collect();

            if applicable.is_empty() {
                return reagents.join(products);
            }

            //compute rule rates
            let rate_sum: f32 = applicable.iter()
                .map(|(_, rate)| rate).sum();
                    
            let rate_partial_sums =
                applicable.iter()
                .map(|(_, rate)| rate)
                .scan(0f32, |partial_sum, rate| {
                    *partial_sum = *partial_sum + rate;
                    Some(*partial_sum)
                });

            //chose one of the rules with probability proportional to their rate
            let distibution = rand::distributions::Uniform::new(0f32, rate_sum);
            let chosen_rate = rng.sample(distibution);

            let rule = applicable.iter()
                .map(|(rule, _)| rule)
                .zip(rate_partial_sums.into_iter())
                .find_map(|(rule, rate)| if rate > chosen_rate { Some(rule) } else { None })
                .unwrap();

            rule.apply(&mut reagents, &mut products);
        }
    }
}

impl Iterator for MinimalProbabilisticPSystem {
    type Item = (f32, MultiSet);

    fn next(&mut self) -> Option<Self::Item> {
        let state = std::mem::take(&mut self.last_state);
        match state {
            Some((time, state)) => {
                let next_time = time + 1f32;
                let next_state = Self::step(state, &self.rules, &mut self.rng);

                self.last_state = Some((next_time, next_state.clone()));
                Some((next_time, next_state.clone()))
            },
            None => {
                self.last_state = Some((0f32, self.initial_state.clone()));
                Some((0f32, self.initial_state.clone()))
            },
        }
    }
}