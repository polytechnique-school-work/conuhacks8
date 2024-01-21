pub mod day_info;
pub mod slot;
pub mod year;

use crate::{algorithm::day::Day, data::reservation::Reservation};
use std::array;

#[derive(Debug)]
pub struct Schedule {
    days: [Day; 366],
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            days: array::from_fn(|_| Day::default()),
        }
    }
}

impl Schedule {
    pub fn add_fifo(&mut self, reservation: Reservation) {
        let day = reservation.reservation_date.ordinal as usize;
        self.days[day].push_walkin(reservation);
    }
    pub fn add_all_fifo(&mut self, mut reservations: Vec<Reservation>) {
        reservations.sort_unstable_by_key(|r| (r.reservation_date, r.call_date));
        for reservation in reservations.into_iter() {
            self.add_fifo(reservation);
        }
    }
    pub fn get_day(&self, day: usize) -> Option<&Day> {
        self.days.get(day)
    }
}
