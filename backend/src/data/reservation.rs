use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{time::date::Date, vehicle::VehicleType};

#[derive(Serialize, Deserialize, Clone)]
pub struct Reservation {
    pub call_date: Date,
    pub reservation_date: Date,
    pub vehicle_type: VehicleType,
}

impl fmt::Debug for Reservation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Reservation")
            .field("duration", &self.reservation_date.duration)
            .field("vehicle_type", &self.vehicle_type)
            .finish()
    }
}

impl Reservation {
    pub fn new_from_string(line: &str) -> Self {
        let mut split = line.split(',');
        let call_date =
            NaiveDateTime::parse_from_str(split.next().unwrap(), "%Y-%m-%d %H:%M").unwrap();
        let reservation_date =
            NaiveDateTime::parse_from_str(split.next().unwrap(), "%Y-%m-%d %H:%M").unwrap();
        let vehicle_type = VehicleType::from_string(split.next().unwrap());
        Self {
            call_date: call_date.into(),
            reservation_date: reservation_date.into(),
            vehicle_type,
        }
    }

    pub fn overlap(&self, reservation: &Reservation) -> bool {
        let reservation_time = std::cmp::max(
            self.vehicle_type.get_time(),
            reservation.vehicle_type.get_time(),
        );
        let delta = self.reservation_date - reservation.reservation_date;
        delta.abs() < *reservation_time
    }

    pub fn is_walkin(&self) -> bool {
        self.call_date.ordinal == self.reservation_date.ordinal
    }
}
