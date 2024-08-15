use rand::{Rng, SeedableRng};

use super::prelude::*;

#[derive(Clone)]
pub struct CustomerQueueState {
    pub queue_length: u32,
    pub operator_available: bool,
    rng: rand::rngs::SmallRng,
}

impl CustomerQueueState {
    fn new(seed: u64) -> Self {
        Self {
            queue_length: 0,
            operator_available: true,
            rng: rand::rngs::SmallRng::seed_from_u64(seed),
        }
    }
}

impl Default for CustomerQueueState {
    fn default() -> Self {
        Self {
            queue_length: 0,
            operator_available: false,
            rng: rand::rngs::SmallRng::seed_from_u64(0),
        }
    }
}

pub struct CustomerQueue;

impl CustomerQueue {
    const CUSTOMER_ARRIVAL: &'static str = "CustomerArrival";
    const CUSTOMER_SERVED: &'static str = "CustomerServed";
    const CUSTOMER_MOVING_TO_SERVICE: &'static str = "CustomerMovingToService";

    pub fn build_des(lambda: f32, mean: f32, std_dev: f32, seed: u64) -> DiscreteEventSimulation<CustomerQueueState> {
        DiscreteEventSimulation::new(
            CustomerQueueState::new(seed),
            vec![
                (0f32, vec![Self::CUSTOMER_ARRIVAL.to_owned()])
            ],
            vec![
                Self::CUSTOMER_MOVING_TO_SERVICE.to_owned()
            ],
            vec![
                (Self::CUSTOMER_ARRIVAL.to_owned(), Self::customer_arrival_event(lambda)),
                (Self::CUSTOMER_SERVED.to_owned(), Self::customer_served())
            ],
            vec![
                (Self::CUSTOMER_MOVING_TO_SERVICE.to_owned(), Self::customer_moving_to_service(mean, std_dev))
            ]
        )
    }

    fn customer_arrival_event(lambda: f32) -> TimedEvent<CustomerQueueState> {
        TimedEvent::new(move |des_state| {
            let state: &mut CustomerQueueState = des_state.get_state_mut();
            let distribution = rand_distr::Exp::new(lambda).unwrap();
            let delta_time = state.rng.sample(distribution);

            state.queue_length += 1;
            des_state.schedule(delta_time, Self::CUSTOMER_ARRIVAL.to_owned());
        })
    }

    fn customer_moving_to_service(mean: f32, std_dev: f32) -> ConditionalEvent<CustomerQueueState> {
        ConditionalEvent::new(
            move |des_state| {
                let state: &mut CustomerQueueState = des_state.get_state_mut();
                let distribution = rand_distr::Normal::new(mean, std_dev).unwrap();
                let delta_time = state.rng.sample(distribution);

                state.queue_length -= 1;
                state.operator_available = false;
                des_state.schedule(delta_time, Self::CUSTOMER_SERVED.to_owned());
                des_state.schedule_conditional(Self::CUSTOMER_MOVING_TO_SERVICE.to_owned());
            },
            move |des_state| {
                let state: &CustomerQueueState = des_state.get_state();
                state.operator_available && state.queue_length > 0
            }
        )
    }

    fn customer_served() -> TimedEvent<CustomerQueueState> {
        TimedEvent::new(move |des_state| {
            let state: &mut CustomerQueueState = des_state.get_state_mut();

            state.operator_available = true;
        })
    }
}