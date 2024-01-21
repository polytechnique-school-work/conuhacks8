use crate::{data::reservation::Reservation, data::reservations::Reservations};
use std::array;

#[derive(Default, Debug)]
pub struct DayPossibility {
    pub reservation: Vec<Reservation>,
    pub walkin: Vec<Reservation>,
}

pub struct Year {
    year: [DayPossibility; 366],
}

impl Default for Year {
    fn default() -> Self {
        Self {
            year: array::from_fn(|_| DayPossibility::default()),
        }
    }
}

impl Year {
    pub fn new(reservations: Reservations) -> Self {
        let mut year = Self::default();
        for reservation in reservations.into_iter() {
            let day_builder = &mut year.year[reservation.reservation_date.ordinal as usize];
            match reservation.is_walkin() {
                true => day_builder.walkin.push(reservation),
                false => day_builder.reservation.push(reservation),
            }
        }
        year
    }
    pub fn get_day(&self, ordinal: usize) -> &DayPossibility {
        &self.year[ordinal]
    }
}
