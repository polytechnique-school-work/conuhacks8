pub mod day;
pub mod day_info;
pub mod slot;

use crate::{reservation::Reservation, reservations::Reservations, schedule::day::Day};
use chrono::Datelike;
use std::array;

#[derive(Debug)]
pub struct Schedule {
    days: [Day; 365],
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            days: array::from_fn(|_| Day::default()),
        }
    }
}

impl Schedule {
    pub fn add(&mut self, reservation: Reservation) {
        let day = reservation.reservation_date.ordinal0() as usize;
        self.days[day].push(reservation);
    }
    pub fn add_all(&mut self, reservations: Reservations) {
        for reservation in reservations.into_iter() {
            self.add(reservation);
        }
    }
    pub fn get_day(&self, day: usize) -> Option<&Day> {
        self.days.get(day)
    }
}
