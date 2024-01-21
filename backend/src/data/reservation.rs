use chrono::{Datelike, NaiveDateTime};
use serde::{Deserialize, Serialize};

use super::vehicle::VehicleType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reservation {
    pub call_date: NaiveDateTime,
    pub reservation_date: NaiveDateTime,
    pub vehicle_type: VehicleType,
}

impl Reservation {
    pub fn new(
        call_date: NaiveDateTime,
        reservation_date: NaiveDateTime,
        vehicle_type: VehicleType,
    ) -> Self {
        Self {
            call_date,
            reservation_date,
            vehicle_type,
        }
    }

    pub fn new_from_string(line: &str) -> Self {
        let mut split = line.split(',');
        let call_date =
            NaiveDateTime::parse_from_str(split.next().unwrap(), "%Y-%m-%d %H:%M").unwrap();
        let reservation_date =
            NaiveDateTime::parse_from_str(split.next().unwrap(), "%Y-%m-%d %H:%M").unwrap();
        let vehicle_type = VehicleType::from_string(split.next().unwrap());
        Self {
            call_date,
            reservation_date,
            vehicle_type,
        }
    }

    pub fn overlap(&self, reservation: &Reservation) -> bool {
        let reservation_time = std::cmp::max(
            self.vehicle_type.get_time(),
            reservation.vehicle_type.get_time(),
        );
        let delta = self.reservation_date - reservation.reservation_date;
        delta.abs() < reservation_time
    }

    pub fn is_walkin(&self) -> bool {
        self.call_date.ordinal() == self.reservation_date.ordinal()
    }
}
