use std::{collections::{BTreeMap, HashMap}, rc::Rc};

#[derive(Clone)]
pub struct ConditionalEvent<S>
    where S: Clone + Default
{
    handle: Rc<dyn Fn(&mut DESState<S>)>,
    is_enabled: Rc<dyn Fn(&DESState<S>) -> bool>,
}

#[derive(Clone)]
pub struct TimedEvent<S>
    where S: Clone + Default
{
    handle: Rc<dyn Fn(&mut DESState<S>)>,
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
    initial_fel: Vec<(Time, Vec<(String, TimedEvent<S>)>)>,
    initial_cel: Vec<(String, ConditionalEvent<S>)>,

    state: Option<(f32, S)>,
    future_event_list: BTreeMap<Time, Vec<(String, TimedEvent<S>)>>,
    conditional_event_list: HashMap<String, ConditionalEvent<S>>,
}

impl<S> DiscreteEventSimulation<S>
    where S: Clone + Default
{
    pub fn new(
        initial_state: S,
        initial_fel: Vec<(f32, Vec<(String, TimedEvent<S>)>)>,
        initial_cel: Vec<(String, ConditionalEvent<S>)>,
    ) -> Self {
        let initial_fel = initial_fel.into_iter()
            .map(|(time, events)| (Time(time), events))
            .collect();

        Self {
            initial_state,
            initial_fel,
            initial_cel,
            state: None,
            future_event_list: BTreeMap::new(),
            conditional_event_list: HashMap::with_capacity(0),
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
                    .find(|(_, event)| (event.is_enabled)(&des_state))
                    .map(|(key, _)| key.clone());

                let mut event =
                    key.and_then(|key| {
                        des_state.conditional_event_list.remove(&key)
                    })
                    .map(|event| (time, event.handle.clone()));

                // otherwise pop the first timed event
                event = event.or_else(||
                    des_state.future_event_list
                    .first_entry()
                    .and_then(|mut entry| {
                        let next_time = *entry.key();
                        let event = entry.get_mut().pop()
                            .map(|(_, event)| (next_time.0, event.handle.clone()));

                        if entry.get().is_empty() {
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
                self.future_event_list = self.initial_fel.iter().cloned().collect();
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
    future_event_list: &'a mut BTreeMap<Time, Vec<(String, TimedEvent<S>)>>,
    conditional_event_list: &'a mut HashMap<String, ConditionalEvent<S>>,
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

    pub fn schedule(&mut self, delta_time: f32, name: String, event: TimedEvent<S>) {
        let time = Time(self.time + delta_time);
        let value = (name, event);

        self.future_event_list.entry(time)
            .and_modify(|vec| vec.push(value.clone()))
            .or_insert_with(|| vec![value]);
    }

    pub fn schedule_conditional(&mut self, name: String, event: ConditionalEvent<S>) {
        self.conditional_event_list.insert(name, event);
    }

    pub fn unschedule_next(&mut self, name: &str) {
        let events = self.future_event_list.iter_mut()
            .find_map(|(_, events)| {
                events.iter().enumerate()
                    .find_map(|(index, (event_name, _))| {
                        if event_name == name {
                            Some(index)
                        } else {
                            None
                        }
                    })
                    .map(|index| (events, index))
            });

        match events {
            Some((events, index)) => { events.remove(index); },
            None => (),
        };
    }

    pub fn unschedule_conditional(&mut self, name: &str) {
        self.conditional_event_list.remove(name);
    }

    pub fn is_scheduled(&self, name: &str) -> bool {
        self.future_event_list.iter()
            .find_map(|(_, events)| 
                events.iter().find(|(event_name, _)| event_name == name)
            )
            .is_some()
    }

    pub fn is_conditional_scheduled(&self, name: &str) -> bool {
        self.conditional_event_list.contains_key(name)
    }
}