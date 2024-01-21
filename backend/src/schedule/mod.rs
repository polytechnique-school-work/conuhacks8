pub mod day_info;
pub mod slot;
pub mod year;

use crate::{
    algorithm::{builder::DayBuilder, day::Day},
    data::{reservation::Reservation, reservations::Reservations},
};
use std::array;

use self::year::Year;

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
    pub fn new(reservation: Reservations) -> Self {
        let mut schedule = Self::default();
        let year = Year::new(reservation);
        for i in 0..366 {
            let day = year.get_day(i);
            schedule.days[i] = DayBuilder::build(&day.reservation);
            schedule.add_all_fifo(day.walkin.clone());
        }
        schedule
    }
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
