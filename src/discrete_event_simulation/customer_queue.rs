use rand::{Rng, SeedableRng};

use super::prelude::*;

#[derive(Clone)]
pub struct CustomerQueueState {
    queue_length: u32,
    operator_available: bool,
    rng: rand::rngs::SmallRng,
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
    fn customer_arrival_event(lambda: f32) -> TimedEvent<CustomerQueueState> {
        TimedEvent::new(move |des_state| {
            let state: &mut CustomerQueueState = des_state.get_state_mut();
            let distribution = rand_distr::Exp::new(lambda).unwrap();
            let delta_time = state.rng.sample(distribution);

            state.queue_length += 1;
            des_state.schedule(delta_time, "CustomerArrival".to_owned());
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
                des_state.schedule(delta_time, "CustomerServed".to_owned());
                des_state.schedule_conditional("CustomerMovingToService".to_owned());
            },
            move |des_state| {
                let state: &CustomerQueueState = des_state.get_state();
                state.queue_length > 0 && state.operator_available
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