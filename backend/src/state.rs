use crate::{data::reservations::Reservations, schedule::Schedule};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub schedule: Arc<RwLock<Schedule>>,
}

impl AppState {
    pub fn new() -> AppState {
        let reservations = Reservations::new_from_file("datafile.csv");
        let schedule = Schedule::new(reservations);
        let schedule = Arc::new(RwLock::new(schedule));
        AppState { schedule }
    }
}
