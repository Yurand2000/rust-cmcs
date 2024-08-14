use std::{collections::{BTreeMap, HashMap, HashSet}, rc::Rc};

#[derive(Clone)]
pub struct ConditionalEvent<S>
    where S: Clone + Default
{
    handle: Rc<dyn Fn(&mut DESState<S>)>,
    is_enabled: Rc<dyn Fn(&DESState<S>) -> bool>,
}

impl<S> ConditionalEvent<S>
    where S: Clone + Default
{
    pub fn new(handle_fun: impl Fn(&mut DESState<S>) + 'static, is_enabled_fun: impl Fn(&DESState<S>) -> bool + 'static) -> Self {
        Self { handle: Rc::new(handle_fun), is_enabled: Rc::new(is_enabled_fun) }
    }
}

#[derive(Clone)]
pub struct TimedEvent<S>
    where S: Clone + Default
{
    handle: Rc<dyn Fn(&mut DESState<S>)>,
}

impl<S> TimedEvent<S>
    where S: Clone + Default
{
    pub fn new(fun: impl Fn(&mut DESState<S>) + 'static) -> Self {
        Self { handle: Rc::new(fun) }
    }
}

#[derive(Clone, Copy)]
#[derive(PartialEq, PartialOrd)]
struct Time(pub f32);

impl Eq for Time {}
impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[derive(Clone)]
pub struct DiscreteEventSimulation<S>
    where S: Clone + Default
{
    initial_state: S,
    initial_fel: Vec<(Time, Vec<String>)>,
    initial_cel: Vec<String>,
    timed_events: HashMap<String, TimedEvent<S>>,
    conditional_events: HashMap<String, ConditionalEvent<S>>,

    state: Option<(f32, S)>,
    future_event_list: BTreeMap<Time, HashSet<String>>,
    conditional_event_list: HashSet<String>,
}

impl<S> DiscreteEventSimulation<S>
    where S: Clone + Default
{
    pub fn new(
        initial_state: S,
        initial_fel: Vec<(f32, Vec<String>)>,
        initial_cel: Vec<String>,
        timed_events: Vec<(String, TimedEvent<S>)>,
        conditional_events: Vec<(String, ConditionalEvent<S>)>,
    ) -> Self {
        let initial_fel = initial_fel.into_iter()
            .map(|(time, events)| (Time(time), events))
            .collect();

        let timed_events = timed_events.into_iter().collect();
        let conditional_events = conditional_events.into_iter().collect();

        Self {
            initial_state,
            initial_fel,
            initial_cel,
            timed_events,
            conditional_events,
            state: None,
            future_event_list: BTreeMap::new(),
            conditional_event_list: HashSet::with_capacity(0),
        }
    }
}

impl<S> Iterator for DiscreteEventSimulation<S>
    where S: Clone + Default
{
    type Item = (f32, S);

    fn next(&mut self) -> Option<Self::Item> {
        let state = std::mem::take(&mut self.state);
        match state {
            Some((time, mut state)) => {
                let mut des_state = DESState {
                    time,
                    state: &mut state,
                    future_event_list: &mut self.future_event_list,
                    conditional_event_list: &mut self.conditional_event_list,
                };

                // pop the first enabled conditional event
                let key =
                    des_state.conditional_event_list.iter()
                    .find(|event| {
                        let event = self.conditional_events
                            .get(event.as_str()).unwrap();

                        (event.is_enabled)(&des_state)
                    })
                    .cloned();

                let mut event =
                    key.map(|key| {
                        des_state.conditional_event_list.remove(key.as_str());

                        let event = self.conditional_events
                            .get(key.as_str()).unwrap();

                        (time, event.handle.clone())
                    });

                // otherwise pop the first timed event
                event = event.or_else(||
                    des_state.future_event_list
                    .first_entry()
                    .and_then(|mut entry| {
                        let next_time = *entry.key();
                        let set = entry.get_mut();
                        let event = set.iter().next().cloned()
                            .map(|key| {
                                set.remove(key.as_str());
        
                                let event = self.timed_events
                                    .get(key.as_str()).unwrap();

                                (next_time.0, event.handle.clone())
                            });

                        if set.is_empty() {
                            entry.remove_entry();
                        }

                        event
                    })
                );

                let (next_time, event) = event?; // if there is no event applicable, just return None

                event(&mut des_state);

                self.state = Some((next_time, state));
                
                self.state.clone()
            },
            None => {
                self.state = Some((0f32, self.initial_state.clone()));
                self.future_event_list = self.initial_fel.iter()
                    .map(|(time, fel)| (time.clone(), fel.iter().cloned().collect())).collect();
                self.conditional_event_list = self.initial_cel.iter().cloned().collect();

                self.state.clone()
            },
        }
    }
}

pub struct DESState<'a, S>
    where S: Clone + Default
{
    time: f32,
    state: &'a mut S,
    future_event_list: &'a mut BTreeMap<Time, HashSet<String>>,
    conditional_event_list: &'a mut HashSet<String>,
}

impl<'a, S> DESState<'a, S>
    where S: Clone + Default
{
    pub fn get_state(&self) -> &S {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut S {
        &mut self.state
    }

    pub fn schedule(&mut self, delta_time: f32, name: String) {
        let time = Time(self.time + delta_time);

        self.future_event_list.entry(time)
            .and_modify(|map| { map.insert(name.clone()); })
            .or_insert_with(|| [name].into_iter().collect());
    }

    pub fn schedule_conditional(&mut self, name: String) {
        self.conditional_event_list.insert(name);
    }

    pub fn unschedule_next(&mut self, name: &str) {
        let events = self.future_event_list.iter_mut()
            .find_map(|(_, events)| {
                if events.contains(name) {
                    Some(events)
                } else {
                    None
                }
            });

        match events {
            Some(events) => { events.remove(name); },
            None => (),
        };
    }

    pub fn unschedule_conditional(&mut self, name: &str) {
        self.conditional_event_list.remove(name);
    }

    pub fn is_scheduled(&self, name: &str) -> bool {
        self.future_event_list.iter()
            .find(|(_, events)| events.contains(name))
            .is_some()
    }

    pub fn is_conditional_scheduled(&self, name: &str) -> bool {
        self.conditional_event_list.contains(name)
    }
}