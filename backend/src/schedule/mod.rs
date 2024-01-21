pub mod day;
pub mod day_info;
pub mod slot;
pub mod year;

use crate::{data::reservation::Reservation, data::reservations::Reservations, schedule::day::Day};
use chrono::Datelike;
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
        let day = reservation.reservation_date.ordinal0() as usize;
        self.days[day].push(reservation);
    }
    pub fn add_all_fifo(&mut self, reservations: Reservations) {
        for reservation in reservations.into_iter() {
            self.add_fifo(reservation);
        }
    }
    pub fn get_day(&self, day: usize) -> Option<&Day> {
        self.days.get(day)
    }
}
