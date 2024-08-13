use super::prelude::*;

#[derive(Clone, PartialEq, Eq)]
struct Frog {
    genotype: Genotype,
    adult: bool,
}

impl Frog {
    fn new(genotype: Genotype, adult: bool) -> Self {
        Self { genotype, adult }
    }

    fn to_object(&self) -> Object {
        use Genotype::*;
        
        let suffix = if self.adult { "_adult" } else { "_juvenile" };
        let genotype = match self.genotype {
            LL => "ll",
            LyL => "lyl",
            LR => "lr",
            LyR => "lyr",
            LRd => "lrd",
            LyRd => "lyrd",
            RR => "rr",
            RyR => "ryr",
            RdRd => "rdrd",
            RydRd => "rydrd",
            RdR => "rdr",
            RydR => "rydr",
            RyRd => "ryrd",
        };

        Object(format!("{genotype}{suffix}"))
    }

    fn get_adult(&self) -> Self {
        Self { genotype: self.genotype, adult: true }
    }

    fn get_all_combinations() -> Vec<Self> {
        [true, false].into_iter()
            .flat_map(|adult| 
                Genotype::get_all_combinations().into_iter()
                    .map(move |genotype| Self { genotype, adult })
            )
            .collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Genotype {
    LL, LyL,        // lessonae
    LR, LyR,        // hybrids
    LRd, LyRd,      // hybrids with mutations
    RR, RyR,        // ridibundus
    RdRd, RydRd,    // ridibundus with mutations
    RdR, RydR, RyRd,
}

impl Genotype {
    fn get_sex(&self) -> Sex {
        match self {
            Genotype::LyL | Genotype::LyR | Genotype::LyRd => Sex::Male,
            _ => Sex::Female,
        }
    }

    fn get_all_combinations() -> Vec<Self> {
        vec![
            Self::LL, Self::LyL,
            Self::LR, Self::LyR,
            Self::LRd, Self::LyRd,
            Self::RR, Self::RyR,
            Self::RdRd, Self::RydRd,
            Self::RdR, Self::RydR, Self::RyRd,
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sex {
    Male,
    Female
}

pub struct FrogLEComplexes;

impl FrogLEComplexes {
    pub fn build_model(initial_frogs: (u32, u32, u32), selection_strength: f32, carry_capacity: f32, seed: u64) -> MinimalProbabilisticPSystem {
        use Genotype::*;

        let initial_state = [
            (Frog::new(LL, true).to_object(), initial_frogs.0 / 2),
            (Frog::new(LyL, true).to_object(), initial_frogs.0 / 2),
            (Frog::new(LR, true).to_object(), initial_frogs.1 / 2),
            (Frog::new(LyR, true).to_object(), initial_frogs.1 / 2),
            (Frog::new(RR, true).to_object(), initial_frogs.2 / 2),
            (Frog::new(RyR, true).to_object(), initial_frogs.2 / 2),
        ];

        let rules = Self::rules(selection_strength, carry_capacity);
        MinimalProbabilisticPSystem::new(initial_state, rules, seed)
    }

    fn control_objects() -> (Object, Object, Object, Object, Object, Object) {
        (
            Object::from_str("SEL"),
            Object::from_str("REPR"),
            Object::from_str("REPR1"),
            Object::from_str("REPR2"),
            Object::from_str("REPR3"),
            Object::from_str("POP"),
        )
    }

    fn rules(selection_strength: f32, carry_capacity: f32) -> Vec<EvolutionRule> {
        Self::reproduction_rules().into_iter()
            .chain(Self::selection_rules(selection_strength, carry_capacity).into_iter())
            .chain(Self::stages_alternation_rules().into_iter())
            .collect()
    }

    fn reproduction_rules() -> Vec<EvolutionRule> {
        let frog_types = Frog::get_all_combinations();
        let adults: Vec<_>
            = frog_types.into_iter().filter(|frog| frog.adult).collect();
        let (adult_males, adult_females): (Vec<_>, Vec<_>)
            = adults.into_iter().partition(|frog| frog.genotype.get_sex() == Sex::Male);
        let (_, reproduction_stage, _, _, _, pop) = Self::control_objects();

        // cartesian product of all the involved frogs
        let iter = adult_males.iter()
            .flat_map(|x| std::iter::repeat(x).zip(adult_females.iter()))
            .flat_map(|(male, female)| {
                let juveniles: Vec<_> = Self::offspring_kinds(male, female);

                let offspring_kinds = juveniles.len() as f32;

                juveniles.into_iter()
                    .map(|genotype| Frog { genotype, adult: false })
                    .map(move |juvenile| (male, female, juvenile, offspring_kinds))
            });

        iter.filter_map(|(male, female, juvenile, offspring_kinds)| {
            let male_obj = male.to_object();
            let female_obj = female.to_object();

            let reactants = [(male_obj.clone(), 1), (female_obj.clone(), 1), (pop.clone(), 2)];
            let products = [(male_obj.clone(), 1), (female_obj.clone(), 1), (juvenile.to_object(), 1), (pop.clone(), 3)];
            let promoters = [(reproduction_stage.clone(), 1)];

            let mating_preference = Self::mating_preference(&male, &female);
            let rate_fn = move |state: &MultiSet| -> f32 {
                let males = state.get(&male_obj) as f32;
                let females = state.get(&female_obj) as f32;

                mating_preference * males * females * (1f32 / offspring_kinds)
            };

            Some(EvolutionRule::new(reactants, products, promoters, rate_fn))
        }).collect()
    }

    fn selection_rules(selection_strength: f32, carry_capacity: f32) -> Vec<EvolutionRule> {
        let frog_types = Frog::get_all_combinations();
        let (selection_stage, _, _, _, _, pop) = Self::control_objects();

        frog_types.into_iter().flat_map(|frog| {
            let frog_obj = frog.to_object();

            let reactants = [(frog_obj.clone(), 1), (pop.clone(), 1)];
            let products = [(frog.get_adult().to_object(), 1), (pop.clone(), 1)];
            let promoters = [(selection_stage.clone(), 1)];
            let fitness = Self::fitness(&frog);

            let pop_clone = pop.clone();
            let rate_function = move |state: &MultiSet| -> f32 {
                let frogs = state.get(&pop_clone) as f32;

                1f32 / (selection_strength + (frogs / (fitness * carry_capacity)))
            };

            let rate_function_clone = rate_function.clone();
            let inv_rate_function = move |state: &MultiSet| -> f32 {
                1f32 - rate_function_clone(state)
            };

            [
                EvolutionRule::new(reactants.clone(), products, promoters.clone(), rate_function),
                EvolutionRule::new(reactants, [], promoters, inv_rate_function),
            ].into_iter()
        }).collect()
    }

    fn stages_alternation_rules() -> Vec<EvolutionRule> {
        let (sel, repr, repr1, repr2, repr3, _) = Self::control_objects();

        // rate functions returning zero should ensure that these rules are triggered when nothing else can be applied.
        vec![
            EvolutionRule::new([(sel.clone(), 1)], [(repr.clone(), 1), (repr1.clone(), 1)], [], |_| { 0f32 }),
            EvolutionRule::new([(repr1.clone(), 1)], [(repr2.clone(), 1)], [], |_| { 0f32 }),
            EvolutionRule::new([(repr2.clone(), 1)], [(repr3.clone(), 1)], [], |_| { 0f32 }),
            EvolutionRule::new([(repr.clone(), 1), (repr3.clone(), 1)], [(sel.clone(), 1)], [], |_| { 0f32 }),
        ]
    }

    fn mating_preference(male: &Frog, female: &Frog) -> f32 {
        use Genotype::*;

        match (male.genotype, female.genotype) {
            (LyL, LL) => 6f32,
            (LyL, LR) | (LyL, LRd) => 2f32,
            (RyR, LL) | (RydR, LL) | (RyRd, LL) => 0f32,
            _ => 1f32,
        }
    }

    fn offspring_kinds(male: &Frog, female: &Frog) -> Vec<Genotype> {
        use Genotype::*;

        match (female.genotype, male.genotype) {
            (LL, LyL) => vec![LyL, LL],
            (LL, LyRd) => vec![LRd],
            (LL, LyR) => vec![LR],
            (LRd, LyL) => vec![LyRd, LRd],
            (LRd, LyRd) => vec![RdRd],
            (LRd, LyR) => vec![RdR],
            (LR, LyL) => vec![LyR, LR],
            (LR, LyRd) => vec![RdR],
            (LR, LyR) => vec![RR],
            (RdR, LyL) => vec![LyRd, LRd, LyR, LR],
            (RdR, LyRd) => vec![RdR, RdRd],
            (RdR, LyR) => vec![RdR, RR],
            (RR, LyL) => vec![LyR, LR],
            (RR, LyRd) => vec![RdR, RR],
            (RR, LyR) => vec![RR],

            (LRd, RyR) => vec![RR, RdR, RyR, RydR, RyRd],
            (LRd, RydR) => vec![RdR, RdRd, RyRd, RydR, RydRd],
            (LRd, RyRd) => vec![RdR, RdRd, RyRd, RydR, RydRd],
            (LR, RyR) => vec![RR, RyR],
            (LR, RydR) => vec![RR, RdR, RyR, RydR, RyRd],
            (LR, RyRd) => vec![RR, RdR, RyR, RydR, RyRd],
            (RdR, RyR) => vec![RR, RyR, RyRd, RdR],
            (RdR, RydR) => vec![RR, RdR, RdRd, RyR, RydR, RyRd, RydRd],
            (RdR, RyRd) => vec![RR, RdR, RdRd, RyR, RydR, RyRd, RydRd],
            (RR, RyR) => vec![RR, RyR],
            (RR, RydR) => vec![RR, RdR, RyR, RydR, RyRd],
            (RR, RyRd) => vec![RR, RdR, RyR, RydR, RyRd],
            _ => Vec::with_capacity(0)
        }
    }

    fn fitness(frog: &Frog) -> f32 {
        use Genotype::*;

        if frog.adult {
            match frog.genotype {
                LRd | LR | LyRd | LyR => 0.55,
                _ => 0.5,
            }
        } else {
            match frog.genotype {
                LRd | LR | LyRd | LyR => 0.88,
                _ => 0.8,
            }
        }
    }
}